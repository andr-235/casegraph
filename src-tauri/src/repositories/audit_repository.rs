use rusqlite::{params, Connection};
use uuid::Uuid;

use crate::errors::app_error::AppErrorDto;

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
}
