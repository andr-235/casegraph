use rusqlite::{params, Connection};

use crate::errors::app_error::AppErrorDto;
use crate::models::settings::AppSettingsDto;
use crate::models::settings_catalog::{
    allowed_setting_keys, default_setting_pairs, KEY_ACCESS_ANALYST_CAN_CREATE_BACKUP,
    KEY_ACCESS_VIEWER_CAN_EXPORT_DOCX, KEY_BACKUP_DEFAULT_DIR, KEY_BACKUP_SAFETY_BEFORE_RESTORE,
    KEY_BACKUP_VERIFY_AFTER_CREATE, KEY_DOCX_DEFAULT_EXPORT_DIR, KEY_DOCX_DEFAULT_TEMPLATE,
    KEY_DOCX_INCLUDE_MATERIALS_TABLE, KEY_DOCX_INCLUDE_SHA256_TABLE,
    KEY_INTEGRITY_WARN_BEFORE_BACKUP, KEY_INTEGRITY_WARN_BEFORE_DOCX,
};

pub struct SettingsRepository;

impl SettingsRepository {
    // ─── Read ───────────────────────────────────────────────────────────────

    /// Load all known settings from the DB and map them to AppSettingsDto.
    /// Unknown keys (e.g. leftover from migrations) are silently ignored.
    pub fn get_settings(conn: &Connection) -> Result<AppSettingsDto, AppErrorDto> {
        let mut settings = AppSettingsDto::default();

        let mut stmt = conn
            .prepare("SELECT key, value FROM app_settings")
            .map_err(|e| AppErrorDto::database(e.to_string()))?;

        let rows = stmt
            .query_map([], |row| {
                let key: String = row.get(0)?;
                let value: String = row.get(1)?;
                Ok((key, value))
            })
            .map_err(|e| AppErrorDto::database(e.to_string()))?;

        for row in rows {
            let (key, value) = row.map_err(|e| AppErrorDto::database(e.to_string()))?;

            match key.as_str() {
                k if k == KEY_DOCX_DEFAULT_TEMPLATE => {
                    if !value.trim().is_empty() {
                        settings.docx.default_template = value;
                    }
                }
                k if k == KEY_DOCX_DEFAULT_EXPORT_DIR => {
                    settings.docx.default_export_dir = value;
                }
                k if k == KEY_DOCX_INCLUDE_MATERIALS_TABLE => {
                    settings.docx.include_materials_table = parse_bool(&value);
                }
                k if k == KEY_DOCX_INCLUDE_SHA256_TABLE => {
                    settings.docx.include_sha256_table = parse_bool(&value);
                }
                k if k == KEY_BACKUP_DEFAULT_DIR => {
                    settings.backup.default_backup_dir = value;
                }
                k if k == KEY_BACKUP_SAFETY_BEFORE_RESTORE => {
                    settings.backup.safety_backup_before_restore = parse_bool(&value);
                }
                k if k == KEY_BACKUP_VERIFY_AFTER_CREATE => {
                    settings.backup.verify_backup_after_create = parse_bool(&value);
                }
                k if k == KEY_INTEGRITY_WARN_BEFORE_DOCX => {
                    settings.integrity.warn_before_docx_export = parse_bool(&value);
                }
                k if k == KEY_INTEGRITY_WARN_BEFORE_BACKUP => {
                    settings.integrity.warn_before_backup = parse_bool(&value);
                }
                k if k == KEY_ACCESS_VIEWER_CAN_EXPORT_DOCX => {
                    settings.access.viewer_can_export_docx = parse_bool(&value);
                }
                k if k == KEY_ACCESS_ANALYST_CAN_CREATE_BACKUP => {
                    settings.access.analyst_can_create_backup = parse_bool(&value);
                }
                _ => {} // unknown key — ignore, do not panic
            }
        }

        Ok(settings)
    }

    /// Load all settings as a raw HashMap<key, value> (used by service-layer diff logic).
    pub fn get_all(
        conn: &Connection,
    ) -> Result<std::collections::HashMap<String, String>, AppErrorDto> {
        let mut stmt = conn
            .prepare("SELECT key, value FROM app_settings")
            .map_err(|e| AppErrorDto::database(e.to_string()))?;

        let rows = stmt
            .query_map([], |row| {
                let key: String = row.get(0)?;
                let value: String = row.get(1)?;
                Ok((key, value))
            })
            .map_err(|e| AppErrorDto::database(e.to_string()))?;

        let mut map = std::collections::HashMap::new();
        for row in rows {
            let (k, v) = row.map_err(|e| AppErrorDto::database(e.to_string()))?;
            map.insert(k, v);
        }

        Ok(map)
    }

    // ─── Write ──────────────────────────────────────────────────────────────

    /// Upsert a slice of (key, value) pairs inside a single transaction.
    /// Rejects any key not present in the settings catalog — defense-in-depth guard.
    pub fn upsert_many(
        conn: &mut Connection,
        pairs: &[(String, String)],
    ) -> Result<(), AppErrorDto> {
        let allowed = allowed_setting_keys();

        for (key, _) in pairs {
            if !allowed.contains(key.as_str()) {
                let msg = format!("Неизвестный ключ настройки: {key}");
                return Err(AppErrorDto::validation(msg.as_str()));
            }
        }

        let tx = conn
            .transaction()
            .map_err(|e| AppErrorDto::database(e.to_string()))?;

        for (key, value) in pairs {
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
            .map_err(|e| AppErrorDto::database(e.to_string()))?;
        }

        tx.commit()
            .map_err(|e| AppErrorDto::database(e.to_string()))?;

        Ok(())
    }

    /// Reset all settings to catalog defaults atomically.
    /// Uses the catalog directly — no separate defaults list to drift out of sync.
    pub fn reset_to_defaults(conn: &mut Connection) -> Result<(), AppErrorDto> {
        let defaults = default_setting_pairs();
        Self::upsert_many(conn, &defaults)
    }
}

// ─── Helpers ──────────────────────────────────────────────────────────────────

fn parse_bool(value: &str) -> bool {
    matches!(value.trim().to_lowercase().as_str(), "1" | "true" | "yes")
}
