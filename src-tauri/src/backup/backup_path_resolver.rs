use std::path::PathBuf;

use rusqlite::OptionalExtension;
use tauri::{AppHandle, Manager};

use crate::errors::app_error::AppErrorDto;

pub struct BackupPathResolver;

impl BackupPathResolver {
    pub fn resolve_safety_backup_dir(
        app: &AppHandle,
        conn: &rusqlite::Connection,
    ) -> Result<PathBuf, AppErrorDto> {
        if let Some(configured_dir) = Self::read_default_backup_dir(conn)? {
            let path = PathBuf::from(configured_dir);

            Self::validate_local_directory(&path)?;

            std::fs::create_dir_all(&path)
                .map_err(|err| AppErrorDto::filesystem(err.to_string()))?;

            return Ok(path);
        }

        let mut fallback = app
            .path()
            .app_data_dir()
            .map_err(|err| AppErrorDto::filesystem(err.to_string()))?;

        fallback.push("backups");
        fallback.push("safety");

        std::fs::create_dir_all(&fallback)
            .map_err(|err| AppErrorDto::filesystem(err.to_string()))?;

        Ok(fallback)
    }

    fn read_default_backup_dir(conn: &rusqlite::Connection) -> Result<Option<String>, AppErrorDto> {
        let value: Option<String> = conn
            .query_row(
                r#"
                SELECT value
                FROM app_settings
                WHERE key = 'backup.defaultBackupDir'
                "#,
                [],
                |row| row.get(0),
            )
            .optional()
            .map_err(|err| AppErrorDto::database(err.to_string()))?;

        Ok(value.filter(|value| !value.trim().is_empty()))
    }

    fn validate_local_directory(path: &PathBuf) -> Result<(), AppErrorDto> {
        let raw = path.to_string_lossy();

        if raw.starts_with("\\\\") {
            return Err(AppErrorDto::validation(
                "Сетевые UNC-пути не поддерживаются для safety backup",
            ));
        }

        Ok(())
    }
}
