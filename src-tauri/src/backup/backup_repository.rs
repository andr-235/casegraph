use rusqlite::{Connection, Row};

use crate::backup::BackupHistoryItemDto;
use crate::errors::app_error::AppErrorDto;

#[derive(Debug, Clone)]
pub struct NewBackupHistoryRow {
    pub id: String,
    pub backup_code: String,
    pub backup_type: String,
    pub status: String,
    pub file_path: String,
    pub file_name: String,
    pub file_size: i64,
    pub sha256: String,
    pub case_id: Option<String>,
    pub case_code: Option<String>,
    pub app_version: String,
    pub schema_version: i64,
    pub created_by: String,
    pub created_at: String,
    pub metadata_json: String,
}

pub struct BackupRepository;

impl BackupRepository {
    pub fn list_history(
        conn: &Connection,
        limit: i64,
    ) -> Result<Vec<BackupHistoryItemDto>, AppErrorDto> {
        let mut stmt = conn
            .prepare(
                r#"
                SELECT
                    id,
                    backup_code,
                    backup_type,
                    status,
                    file_name,
                    file_size,
                    sha256,
                    case_id,
                    case_code,
                    app_version,
                    schema_version,
                    created_by,
                    created_at,
                    verified_at,
                    restored_at
                FROM backup_history
                ORDER BY created_at DESC
                LIMIT ?1
                "#,
            )
            .map_err(|err| AppErrorDto::database(err.to_string()))?;

        let rows = stmt
            .query_map([limit], Self::map_history_item)
            .map_err(|err| AppErrorDto::database(err.to_string()))?;

        let mut items = Vec::new();

        for row in rows {
            items.push(row.map_err(|err| AppErrorDto::database(err.to_string()))?);
        }

        Ok(items)
    }

    pub fn insert_history(conn: &Connection, row: &NewBackupHistoryRow) -> Result<(), AppErrorDto> {
        conn.execute(
            r#"
            INSERT INTO backup_history (
                id,
                backup_code,
                backup_type,
                status,
                file_path,
                file_name,
                file_size,
                sha256,
                case_id,
                case_code,
                app_version,
                schema_version,
                created_by,
                created_at,
                metadata_json
            )
            VALUES (?1, ?2, ?3, ?4, ?5, ?6, ?7, ?8, ?9, ?10, ?11, ?12, ?13, ?14, ?15)
            "#,
            rusqlite::params![
                row.id,
                row.backup_code,
                row.backup_type,
                row.status,
                row.file_path,
                row.file_name,
                row.file_size,
                row.sha256,
                row.case_id,
                row.case_code,
                row.app_version,
                row.schema_version,
                row.created_by,
                row.created_at,
                row.metadata_json,
            ],
        )
        .map_err(|err| AppErrorDto::database(err.to_string()))?;

        Ok(())
    }

    fn map_history_item(row: &Row<'_>) -> rusqlite::Result<BackupHistoryItemDto> {
        Ok(BackupHistoryItemDto {
            id: row.get("id")?,
            backup_code: row.get("backup_code")?,
            backup_type: row.get("backup_type")?,
            status: row.get("status")?,
            file_name: row.get("file_name")?,
            file_size: row.get("file_size")?,
            sha256: row.get("sha256")?,
            case_id: row.get("case_id")?,
            case_code: row.get("case_code")?,
            app_version: row.get("app_version")?,
            schema_version: row.get("schema_version")?,
            created_by: row.get("created_by")?,
            created_at: row.get("created_at")?,
            verified_at: row.get("verified_at")?,
            restored_at: row.get("restored_at")?,
        })
    }
}
