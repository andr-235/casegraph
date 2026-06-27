use std::collections::HashMap;

use rusqlite::Connection;

use crate::errors::app_error::AppErrorDto;
use crate::models::settings_catalog::{
    KEY_ACCESS_ANALYST_CAN_CREATE_BACKUP, KEY_ACCESS_VIEWER_CAN_EXPORT_DOCX,
};
use crate::repositories::settings_repository::SettingsRepository;

#[derive(Debug, Clone, Copy)]
pub struct SettingsAccessPolicy {
    pub viewer_can_export_docx: bool,
    pub analyst_can_create_backup: bool,
}

impl SettingsAccessPolicy {
    pub fn from_connection(conn: &Connection) -> Result<Self, AppErrorDto> {
        let values = SettingsRepository::get_all(conn)?;

        Ok(Self {
            viewer_can_export_docx: read_bool(&values, KEY_ACCESS_VIEWER_CAN_EXPORT_DOCX, false),
            analyst_can_create_backup: read_bool(
                &values,
                KEY_ACCESS_ANALYST_CAN_CREATE_BACKUP,
                false,
            ),
        })
    }
}

fn read_bool(values: &HashMap<String, String>, key: &str, default_value: bool) -> bool {
    values
        .get(key)
        .map(|value| value == "true")
        .unwrap_or(default_value)
}

#[cfg(test)]
mod tests {
    use super::*;

    fn setup_conn_with_settings(pairs: &[(&str, &str)]) -> Connection {
        let conn = Connection::open_in_memory().unwrap();
        conn.execute_batch(
            "CREATE TABLE app_settings (key TEXT PRIMARY KEY, value TEXT NOT NULL);",
        )
        .unwrap();
        for (k, v) in pairs {
            conn.execute(
                "INSERT INTO app_settings (key, value) VALUES (?1, ?2)",
                rusqlite::params![k, v],
            )
            .unwrap();
        }
        conn
    }

    #[test]
    fn defaults_when_no_rows() {
        let conn = Connection::open_in_memory().unwrap();
        conn.execute_batch(
            "CREATE TABLE app_settings (key TEXT PRIMARY KEY, value TEXT NOT NULL);",
        )
        .unwrap();
        let policy = SettingsAccessPolicy::from_connection(&conn).unwrap();
        assert!(!policy.viewer_can_export_docx);
        assert!(!policy.analyst_can_create_backup);
    }

    #[test]
    fn reads_bool_values() {
        let conn = setup_conn_with_settings(&[
            (KEY_ACCESS_VIEWER_CAN_EXPORT_DOCX, "true"),
            (KEY_ACCESS_ANALYST_CAN_CREATE_BACKUP, "false"),
        ]);
        let policy = SettingsAccessPolicy::from_connection(&conn).unwrap();
        assert!(policy.viewer_can_export_docx);
        assert!(!policy.analyst_can_create_backup);
    }

    #[test]
    fn missing_key_uses_default() {
        let conn = setup_conn_with_settings(&[("some.other_key", "true")]);
        let policy = SettingsAccessPolicy::from_connection(&conn).unwrap();
        assert!(!policy.viewer_can_export_docx);
        assert!(!policy.analyst_can_create_backup);
    }
}
