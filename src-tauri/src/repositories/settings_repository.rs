use rusqlite::Connection;

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
                "storage_path" => settings.storage_path = empty_to_none(value),
                "docx_default_template" => {
                    if !value.trim().is_empty() {
                        settings.docx_default_template = value;
                    }
                }
                "backup_default_path" => settings.backup_default_path = empty_to_none(value),
                "integrity_check_on_startup" => {
                    settings.integrity_check_on_startup = parse_bool(&value);
                }
                "viewer_can_export_docx" => {
                    settings.viewer_can_export_docx = parse_bool(&value);
                }
                "analyst_can_create_backup" => {
                    settings.analyst_can_create_backup = parse_bool(&value);
                }
                "audit_strict_mode" => {
                    settings.audit_strict_mode = parse_bool(&value);
                }
                _ => {}
            }
        }

        Ok(settings)
    }
}

fn parse_bool(value: &str) -> bool {
    matches!(value.trim().to_lowercase().as_str(), "1" | "true" | "yes")
}

fn empty_to_none(value: String) -> Option<String> {
    let trimmed = value.trim();

    if trimmed.is_empty() {
        None
    } else {
        Some(trimmed.to_string())
    }
}
