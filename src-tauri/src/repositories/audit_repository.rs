use rusqlite::{params, Connection, OptionalExtension};
use uuid::Uuid;

use crate::domain::audit::{
    AuditAccessDeniedInput, AuditActionOptionDto, AuditLogDto, AuditUserOptionDto,
};
use crate::domain::audit_action_label::normalize_legacy_action;
use crate::errors::app_error::AppErrorDto;
use crate::security::session::CurrentUserDto;

#[derive(Debug)]
pub struct CreateAuditLogRecord {
    pub user_id: Option<String>,
    pub username: String,
    pub user_role: String,

    pub action: String,
    pub entity_type: String,
    pub entity_id: Option<String>,
    pub case_id: Option<String>,

    pub result: String,
    pub severity: String,

    pub old_value: Option<String>,
    pub new_value: Option<String>,
    pub technical_details: Option<String>,

    pub app_version: String,
}

pub struct AuditRepository;

impl AuditRepository {
    pub fn insert_access_denied(
        conn: &Connection,
        current_user: &CurrentUserDto,
        input: &AuditAccessDeniedInput,
    ) -> Result<(), AppErrorDto> {
        let audit_id = Uuid::new_v4().to_string();

        let technical_details = format!(
            r#"{{"command":"{}","requiredRole":"{}","actualRole":"{}","description":"{}"}}"#,
            input.command_name.replace('"', "\\\""),
            input.required_role.replace('"', "\\\""),
            current_user.role.replace('"', "\\\""),
            input.description.replace('"', "\\\""),
        );

        conn.execute(
            "
            INSERT INTO audit_logs (
                id, user_id, username, user_role,
                action, entity_type, entity_id,
                result, severity,
                technical_details, app_version
            )
            VALUES (?1, ?2, ?3, ?4, ?5, ?6, ?7, ?8, ?9, ?10, ?11)
            ",
            params![
                audit_id,
                current_user.user_id,
                current_user.username,
                current_user.role,
                crate::domain::audit_action::audit::ACCESS_DENIED,
                input.entity_type,
                input.entity_id,
                "denied",
                "warning",
                technical_details,
                env!("CARGO_PKG_VERSION"),
            ],
        )
        .map_err(|err| AppErrorDto::database(err.to_string()))?;

        Ok(())
    }

    pub fn insert(conn: &Connection, record: CreateAuditLogRecord) -> Result<(), AppErrorDto> {
        let id = Uuid::new_v4().to_string();

        conn.execute(
            "
            INSERT INTO audit_logs (
                id,
                user_id,
                username,
                user_role,
                action,
                entity_type,
                entity_id,
                case_id,
                result,
                severity,
                old_value,
                new_value,
                technical_details,
                app_version
            )
            VALUES (?1, ?2, ?3, ?4, ?5, ?6, ?7, ?8, ?9, ?10, ?11, ?12, ?13, ?14)
            ",
            params![
                id,
                record.user_id,
                record.username,
                record.user_role,
                record.action,
                record.entity_type,
                record.entity_id,
                record.case_id,
                record.result,
                record.severity,
                record.old_value,
                record.new_value,
                record.technical_details,
                record.app_version,
            ],
        )
        .map_err(|err| AppErrorDto::database(err.to_string()))?;

        Ok(())
    }

    pub fn get_audit_logs(
        conn: &Connection,
        filters: &AuditLogFilters,
    ) -> Result<Vec<AuditLogDto>, AppErrorDto> {
        let mut stmt = conn
            .prepare(
                "
                SELECT
                    id,
                    user_id,
                    username,
                    user_role,
                    action,
                    entity_type,
                    entity_id,
                    case_id,
                    result,
                    severity,
                    old_value,
                    new_value,
                    technical_details,
                    app_version,
                    created_at
                FROM audit_logs
                WHERE
                    (?1 IS NULL OR action = ?1)
                    AND (?2 IS NULL OR result = ?2)
                    AND (?3 IS NULL OR severity = ?3)
                    AND (?4 IS NULL OR case_id = ?4)
                    AND (?5 IS NULL OR entity_type = ?5)
                    AND (?6 IS NULL OR created_at >= ?6)
                    AND (?7 IS NULL OR created_at <= ?7)
                    AND (?8 IS NULL OR user_id = ?8)
                ORDER BY created_at DESC
                LIMIT ?9 OFFSET ?10
                ",
            )
            .map_err(|err| AppErrorDto::database(err.to_string()))?;

        let rows = stmt
            .query_map(
                params![
                    filters.action.as_deref(),
                    filters.result.as_deref(),
                    filters.severity.as_deref(),
                    filters.case_id.as_deref(),
                    filters.entity_type.as_deref(),
                    filters.date_from.as_deref(),
                    filters.date_to.as_deref(),
                    filters.user_id.as_deref(),
                    filters.limit,
                    filters.offset,
                ],
                |row| {
                    Ok(AuditLogDto {
                        id: row.get(0)?,
                        user_id: row.get(1)?,
                        username: row.get(2)?,
                        user_role: row.get(3)?,
                        action: normalize_legacy_action(&row.get::<_, String>(4)?).to_string(),
                        entity_type: row.get(5)?,
                        entity_id: row.get(6)?,
                        case_id: row.get(7)?,
                        result: row.get(8)?,
                        severity: row.get(9)?,
                        old_value: row.get(10)?,
                        new_value: row.get(11)?,
                        technical_details: row.get(12)?,
                        app_version: row.get(13)?,
                        created_at: row.get(14)?,
                    })
                },
            )
            .map_err(|err| AppErrorDto::database(err.to_string()))?;

        let mut items = Vec::new();

        for row in rows {
            items.push(row.map_err(|err| AppErrorDto::database(err.to_string()))?);
        }

        Ok(items)
    }

    pub fn get_audit_log_by_id(
        conn: &Connection,
        audit_log_id: &str,
    ) -> Result<Option<AuditLogDto>, AppErrorDto> {
        conn.query_row(
            "
            SELECT
                id,
                user_id,
                username,
                user_role,
                action,
                entity_type,
                entity_id,
                case_id,
                result,
                severity,
                old_value,
                new_value,
                technical_details,
                app_version,
                created_at
            FROM audit_logs
            WHERE id = ?1
            LIMIT 1
            ",
            params![audit_log_id],
            |row| {
                Ok(AuditLogDto {
                    id: row.get(0)?,
                    user_id: row.get(1)?,
                    username: row.get(2)?,
                    user_role: row.get(3)?,
                    action: normalize_legacy_action(&row.get::<_, String>(4)?).to_string(),
                    entity_type: row.get(5)?,
                    entity_id: row.get(6)?,
                    case_id: row.get(7)?,
                    result: row.get(8)?,
                    severity: row.get(9)?,
                    old_value: row.get(10)?,
                    new_value: row.get(11)?,
                    technical_details: row.get(12)?,
                    app_version: row.get(13)?,
                    created_at: row.get(14)?,
                })
            },
        )
        .optional()
        .map_err(|err| AppErrorDto::database(err.to_string()))
    }

    pub fn get_audit_actions(
        conn: &Connection,
        restricted_user_id: Option<&str>,
    ) -> Result<Vec<AuditActionOptionDto>, AppErrorDto> {
        let mut stmt = conn
            .prepare(
                "
                SELECT action, COUNT(*) as count
                FROM audit_logs
                WHERE (?1 IS NULL OR user_id = ?1)
                GROUP BY action
                ORDER BY action ASC
                ",
            )
            .map_err(|err| AppErrorDto::database(err.to_string()))?;

        let rows = stmt
            .query_map(params![restricted_user_id], |row| {
                Ok(AuditActionOptionDto {
                    action: row.get(0)?,
                    count: row.get(1)?,
                })
            })
            .map_err(|err| AppErrorDto::database(err.to_string()))?;

        let mut items = Vec::new();

        for row in rows {
            items.push(row.map_err(|err| AppErrorDto::database(err.to_string()))?);
        }

        Ok(items)
    }

    pub fn get_audit_users(conn: &Connection) -> Result<Vec<AuditUserOptionDto>, AppErrorDto> {
        let mut stmt = conn
            .prepare(
                "
                SELECT
                    user_id,
                    username,
                    user_role,
                    COUNT(*) as count
                FROM audit_logs
                WHERE user_id IS NOT NULL AND TRIM(user_id) != ''
                GROUP BY user_id, username, user_role
                ORDER BY username ASC, user_role ASC
                ",
            )
            .map_err(|err| AppErrorDto::database(err.to_string()))?;

        let rows = stmt
            .query_map([], |row| {
                Ok(AuditUserOptionDto {
                    user_id: row.get(0)?,
                    username: row.get(1)?,
                    user_role: row.get(2)?,
                    count: row.get(3)?,
                })
            })
            .map_err(|err| AppErrorDto::database(err.to_string()))?;

        let mut items = Vec::new();

        for row in rows {
            items.push(row.map_err(|err| AppErrorDto::database(err.to_string()))?);
        }

        Ok(items)
    }

    pub fn export_audit_logs(
        conn: &Connection,
        filters: &AuditLogFilters,
    ) -> Result<Vec<AuditLogDto>, AppErrorDto> {
        let mut stmt = conn
            .prepare(
                "
                SELECT
                    id,
                    user_id,
                    username,
                    user_role,
                    action,
                    entity_type,
                    entity_id,
                    case_id,
                    result,
                    severity,
                    old_value,
                    new_value,
                    technical_details,
                    app_version,
                    created_at
                FROM audit_logs
                WHERE
                    (?1 IS NULL OR action = ?1)
                    AND (?2 IS NULL OR result = ?2)
                    AND (?3 IS NULL OR severity = ?3)
                    AND (?4 IS NULL OR case_id = ?4)
                    AND (?5 IS NULL OR entity_type = ?5)
                    AND (?6 IS NULL OR created_at >= ?6)
                    AND (?7 IS NULL OR created_at <= ?7)
                    AND (?8 IS NULL OR user_id = ?8)
                ORDER BY created_at DESC
                ",
            )
            .map_err(|err| AppErrorDto::database(err.to_string()))?;

        let rows = stmt
            .query_map(
                params![
                    filters.action.as_deref(),
                    filters.result.as_deref(),
                    filters.severity.as_deref(),
                    filters.case_id.as_deref(),
                    filters.entity_type.as_deref(),
                    filters.date_from.as_deref(),
                    filters.date_to.as_deref(),
                    filters.user_id.as_deref(),
                ],
                |row| {
                    Ok(AuditLogDto {
                        id: row.get(0)?,
                        user_id: row.get(1)?,
                        username: row.get(2)?,
                        user_role: row.get(3)?,
                        action: normalize_legacy_action(&row.get::<_, String>(4)?).to_string(),
                        entity_type: row.get(5)?,
                        entity_id: row.get(6)?,
                        case_id: row.get(7)?,
                        result: row.get(8)?,
                        severity: row.get(9)?,
                        old_value: row.get(10)?,
                        new_value: row.get(11)?,
                        technical_details: row.get(12)?,
                        app_version: row.get(13)?,
                        created_at: row.get(14)?,
                    })
                },
            )
            .map_err(|err| AppErrorDto::database(err.to_string()))?;

        let mut items = Vec::new();

        for row in rows {
            items.push(row.map_err(|err| AppErrorDto::database(err.to_string()))?);
        }

        Ok(items)
    }

    pub fn count_audit_logs(
        conn: &Connection,
        filters: &AuditLogFilters,
    ) -> Result<i64, AppErrorDto> {
        conn.query_row(
            "
            SELECT COUNT(*)
            FROM audit_logs
            WHERE
                (?1 IS NULL OR action = ?1)
                AND (?2 IS NULL OR result = ?2)
                AND (?3 IS NULL OR severity = ?3)
                AND (?4 IS NULL OR case_id = ?4)
                AND (?5 IS NULL OR entity_type = ?5)
                AND (?6 IS NULL OR created_at >= ?6)
                AND (?7 IS NULL OR created_at <= ?7)
                AND (?8 IS NULL OR user_id = ?8)
            ",
            params![
                filters.action.as_deref(),
                filters.result.as_deref(),
                filters.severity.as_deref(),
                filters.case_id.as_deref(),
                filters.entity_type.as_deref(),
                filters.date_from.as_deref(),
                filters.date_to.as_deref(),
                filters.user_id.as_deref(),
            ],
            |row| row.get(0),
        )
        .map_err(|err| AppErrorDto::database(err.to_string()))
    }
}

#[derive(Debug)]
pub struct AuditLogFilters {
    pub action: Option<String>,
    pub result: Option<String>,
    pub severity: Option<String>,
    pub case_id: Option<String>,
    pub entity_type: Option<String>,
    pub date_from: Option<String>,
    pub date_to: Option<String>,
    pub user_id: Option<String>,
    pub limit: i64,
    pub offset: i64,
}
