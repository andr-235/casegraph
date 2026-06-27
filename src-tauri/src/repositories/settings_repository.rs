use rusqlite::{params, Connection};

use crate::errors::app_error::AppErrorDto;
use crate::models::settings::AppSettingsDto;

pub struct SettingsRepository;

impl SettingsRepository {
    pub fn get_settings(conn: &Connection) -> Result<AppSettingsDto, AppErrorDto> {
        let mut settings = AppSettingsDto::default();

        let mut stmt = conn
            .prepare(
                r#"
                SELECT key, value
                FROM app_settings
                "#,
            )
            .map_err(|err| AppErrorDto::database(err.to_string()))?;

        let rows = stmt
            .query_map([], |row| {
                let key: String = row.get(0)?;
                let value: String = row.get(1)?;
                Ok((key, value))
            })
            .map_err(|err| AppErrorDto::database(err.to_string()))?;

        for row in rows {
            let (key, value) = row.map_err(|err| AppErrorDto::database(err.to_string()))?;

            match key.as_str() {
                "docx.default_template" => {
                    if !value.trim().is_empty() {
                        settings.docx.default_template = value;
                    }
                }
                "docx.default_export_dir" => {
                    settings.docx.default_export_dir = value;
                }
                "docx.include_materials_table" => {
                    settings.docx.include_materials_table = parse_bool(&value);
                }
                "docx.include_sha256_table" => {
                    settings.docx.include_sha256_table = parse_bool(&value);
                }
                "backup.default_dir" => {
                    settings.backup.default_backup_dir = value;
                }
                "backup.safety_before_restore" => {
                    settings.backup.safety_backup_before_restore = parse_bool(&value);
                }
                "backup.verify_after_create" => {
                    settings.backup.verify_backup_after_create = parse_bool(&value);
                }
                "integrity.warn_before_docx_export" => {
                    settings.integrity.warn_before_docx_export = parse_bool(&value);
                }
                "integrity.warn_before_backup" => {
                    settings.integrity.warn_before_backup = parse_bool(&value);
                }
                "access.viewer_can_export_docx" => {
                    settings.access.viewer_can_export_docx = parse_bool(&value);
                }
                "access.analyst_can_create_backup" => {
                    settings.access.analyst_can_create_backup = parse_bool(&value);
                }
                _ => {}
            }
        }

        Ok(settings)
    }

    pub fn upsert_many(
        conn: &mut Connection,
        settings: &[(String, String)],
    ) -> Result<(), AppErrorDto> {
        let tx = conn
            .transaction()
            .map_err(|err| AppErrorDto::database(err.to_string()))?;

        for (key, value) in settings {
            tx.execute(
                r#"
                INSERT INTO app_settings (key, value, value_type, category, description)
                VALUES (?1, ?2, 'string', 'general', '')
                ON CONFLICT(key) DO UPDATE SET
                    value = excluded.value,
                    updated_at = CURRENT_TIMESTAMP
                "#,
                params![key, value],
            )
            .map_err(|err| AppErrorDto::database(err.to_string()))?;
        }

        tx.commit()
            .map_err(|err| AppErrorDto::database(err.to_string()))?;
        Ok(())
    }
}

fn parse_bool(value: &str) -> bool {
    matches!(value.trim().to_lowercase().as_str(), "1" | "true" | "yes")
}
