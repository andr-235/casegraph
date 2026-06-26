use rusqlite::{params, Connection, OptionalExtension};

use crate::domain::case_status::CASE_STATUS_DRAFT;
use crate::errors::app_error::AppErrorDto;

#[derive(Debug)]
pub struct CaseRow {
    pub id: String,
    pub case_code: String,
    pub title: String,
    pub subject: String,
    pub description: String,
    pub status: String,
    pub period_start: Option<String>,
    pub period_end: Option<String>,
    pub created_by_user_id: String,
    pub created_at: String,
    pub updated_at: String,
}

#[derive(Debug)]
pub struct CreateCaseRecord {
    pub id: String,
    pub case_code: String,
    pub title: String,
    pub subject: String,
    pub description: String,
    pub period_start: Option<String>,
    pub period_end: Option<String>,
    pub created_by_user_id: String,
}

#[derive(Debug)]
pub struct UpdateCaseRecord {
    pub case_id: String,
    pub title: String,
    pub subject: String,
    pub description: String,
    pub period_start: Option<String>,
    pub period_end: Option<String>,
}

pub struct CaseRepository;

impl CaseRepository {
    pub fn get_next_case_code(conn: &Connection) -> Result<String, AppErrorDto> {
        let next_number: i64 = conn
            .query_row(
                r#"
                SELECT COALESCE(
                    MAX(CAST(SUBSTR(case_code, 6) AS INTEGER)),
                    0
                ) + 1
                FROM cases
                WHERE case_code LIKE 'CASE-%'
                "#,
                [],
                |row| row.get(0),
            )
            .map_err(|err| AppErrorDto::database(err.to_string()))?;

        Ok(format!("CASE-{next_number:03}"))
    }

    pub fn create_case(conn: &Connection, record: CreateCaseRecord) -> Result<(), AppErrorDto> {
        conn.execute(
            r#"
            INSERT INTO cases (
                id,
                case_code,
                title,
                subject,
                description,
                status,
                period_start,
                period_end,
                created_by_user_id
            )
                VALUES (?1, ?2, ?3, ?4, ?5, ?6, ?7, ?8, ?9)
            "#,
            params![
                record.id,
                record.case_code,
                record.title,
                record.subject,
                record.description,
                CASE_STATUS_DRAFT,
                record.period_start,
                record.period_end,
                record.created_by_user_id
            ],
        )
        .map_err(|err| AppErrorDto::database(err.to_string()))?;

        Ok(())
    }

    pub fn get_cases(conn: &Connection) -> Result<Vec<CaseRow>, AppErrorDto> {
        let mut statement = conn
            .prepare(
                r#"
                SELECT
                    id,
                    case_code,
                    title,
                    subject,
                    description,
                    status,
                    period_start,
                    period_end,
                    created_by_user_id,
                    created_at,
                    updated_at
                FROM cases
                WHERE archived_at IS NULL
                ORDER BY created_at DESC
                "#,
            )
            .map_err(|err| AppErrorDto::database(err.to_string()))?;

        let rows = statement
            .query_map([], |row| {
                Ok(CaseRow {
                    id: row.get(0)?,
                    case_code: row.get(1)?,
                    title: row.get(2)?,
                    subject: row.get(3)?,
                    description: row.get(4)?,
                    status: row.get(5)?,
                    period_start: row.get(6)?,
                    period_end: row.get(7)?,
                    created_by_user_id: row.get(8)?,
                    created_at: row.get(9)?,
                    updated_at: row.get(10)?,
                })
            })
            .map_err(|err| AppErrorDto::database(err.to_string()))?;

        let mut cases = Vec::new();

        for row in rows {
            cases.push(row.map_err(|err| AppErrorDto::database(err.to_string()))?);
        }

        Ok(cases)
    }

    pub fn get_case_by_id(
        conn: &Connection,
        case_id: &str,
    ) -> Result<Option<CaseRow>, AppErrorDto> {
        conn.query_row(
            r#"
            SELECT
                id,
                case_code,
                title,
                subject,
                description,
                status,
                period_start,
                period_end,
                created_by_user_id,
                created_at,
                updated_at
            FROM cases
            WHERE id = ?1
              AND archived_at IS NULL
            LIMIT 1
            "#,
            params![case_id],
            |row| {
                Ok(CaseRow {
                    id: row.get(0)?,
                    case_code: row.get(1)?,
                    title: row.get(2)?,
                    subject: row.get(3)?,
                    description: row.get(4)?,
                    status: row.get(5)?,
                    period_start: row.get(6)?,
                    period_end: row.get(7)?,
                    created_by_user_id: row.get(8)?,
                    created_at: row.get(9)?,
                    updated_at: row.get(10)?,
                })
            },
        )
        .optional()
        .map_err(|err| AppErrorDto::database(err.to_string()))
    }

    pub fn update_case(conn: &Connection, record: UpdateCaseRecord) -> Result<(), AppErrorDto> {
        let changed_count = conn
            .execute(
                r#"
            UPDATE cases
            SET
                title = ?2,
                subject = ?3,
                description = ?4,
                period_start = ?5,
                period_end = ?6,
                updated_at = CURRENT_TIMESTAMP
            WHERE id = ?1
            AND archived_at IS NULL
            "#,
                params![
                    record.case_id,
                    record.title,
                    record.subject,
                    record.description,
                    record.period_start,
                    record.period_end
                ],
            )
            .map_err(|err| AppErrorDto::database(err.to_string()))?;

        if changed_count == 0 {
            return Err(AppErrorDto::new(
                "ERR_CASE_NOT_FOUND",
                "Дело не найдено.",
                None,
            ));
        }

        Ok(())
    }

    pub fn update_case_status(
        conn: &Connection,
        case_id: &str,
        status: &str,
    ) -> Result<(), AppErrorDto> {
        let changed_count = conn
            .execute(
                r#"
                UPDATE cases
                SET
                    status = ?2,
                    updated_at = CURRENT_TIMESTAMP
                WHERE id = ?1
                AND archived_at IS NULL
                "#,
                params![case_id, status],
            )
            .map_err(|err| AppErrorDto::database(err.to_string()))?;

        if changed_count == 0 {
            return Err(AppErrorDto::new(
                "ERR_CASE_NOT_FOUND",
                "Дело не найдено.",
                None,
            ));
        }

        Ok(())
    }
}
