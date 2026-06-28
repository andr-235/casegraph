use std::path::PathBuf;

use chrono::Utc;
use rusqlite::OptionalExtension;
use tauri::AppHandle;
use tauri_plugin_dialog::DialogExt;

use crate::audit::audit_metadata::{
    safe_restore_preflight_details, safe_restore_preflight_snapshot,
};
use crate::audit::audit_service::{AuditService, AuditWriteInput};
use crate::backup::{
    BackupArchiveReader, BackupRepository, RestoreBackupMetadataPreviewDto,
    RestoreBackupPreflightPayload, RestoreBackupPreflightResponse, RestoreCompatibilityDto,
    RestorePreflightIssueDto, RestorePreflightIssueSeverity, SelectRestoreBackupFileResponse,
};
use crate::domain::audit_action;
use crate::errors::app_error::AppErrorDto;
use crate::security::{ProtectedOperation, ProtectedServiceContext};

pub struct RestoreService;

impl RestoreService {
    pub fn choose_restore_backup_file(
        app: &AppHandle,
    ) -> Result<SelectRestoreBackupFileResponse, AppErrorDto> {
        ProtectedServiceContext::require_operation(app, ProtectedOperation::BackupRestore)?;

        let file = app
            .dialog()
            .file()
            .set_title("Выберите backup для восстановления")
            .add_filter("CaseGraph backup", &["zip"])
            .blocking_pick_file();

        let file_path = file.map(|file_path| match file_path {
            tauri_plugin_dialog::FilePath::Path(p) => p.to_string_lossy().to_string(),
            tauri_plugin_dialog::FilePath::Url(u) => {
                if let Ok(p) = u.to_file_path() {
                    p.to_string_lossy().to_string()
                } else {
                    u.path().to_string()
                }
            }
        });

        Ok(SelectRestoreBackupFileResponse { file_path })
    }

    pub fn restore_backup_preflight(
        app: &AppHandle,
        payload: RestoreBackupPreflightPayload,
    ) -> Result<RestoreBackupPreflightResponse, AppErrorDto> {
        let ctx =
            ProtectedServiceContext::require_operation(app, ProtectedOperation::BackupRestore)?;

        let checked_at = Utc::now().to_rfc3339();

        let history_row = match payload.backup_id.as_deref() {
            Some(backup_id) => BackupRepository::find_private_by_id(&ctx.conn, backup_id)?,
            None => None,
        };

        let file_path = match (&history_row, payload.file_path.as_deref()) {
            (Some(row), _) => PathBuf::from(&row.file_path),
            (None, Some(path)) if !path.trim().is_empty() => PathBuf::from(path),
            _ => {
                return Err(AppErrorDto::validation(
                    "Не выбран backup для preflight restore",
                ))
            }
        };

        Self::validate_path(&file_path)?;

        let verification = BackupArchiveReader::verify(&file_path)?;

        let mut warnings = Vec::new();
        let mut errors = Vec::new();

        for issue in &verification.issues {
            let target = match issue.severity {
                crate::backup::BackupVerificationIssueSeverity::Error => &mut errors,
                crate::backup::BackupVerificationIssueSeverity::Warning => &mut warnings,
            };

            target.push(RestorePreflightIssueDto {
                code: issue.code.clone(),
                message: issue.message.clone(),
                severity: RestorePreflightIssueSeverity::Error,
            });
        }

        let metadata = verification
            .metadata
            .as_ref()
            .ok_or_else(|| AppErrorDto::validation("Backup metadata отсутствует"))?;

        let manifest_file_count = verification
            .manifest
            .as_ref()
            .map(|m| m.files.len())
            .unwrap_or(0);

        let current_app_version = env!("CARGO_PKG_VERSION").to_owned();
        let current_schema_version = Self::current_schema_version(&ctx.conn)?;

        let compatibility = Self::build_compatibility(
            &current_app_version,
            current_schema_version,
            &metadata.app_version,
            metadata.schema_version,
            &metadata.backup_type,
        );

        if !compatibility.backup_type_ok {
            errors.push(RestorePreflightIssueDto {
                code: "ERR_RESTORE_UNSUPPORTED_BACKUP_TYPE".to_owned(),
                message: "Этот тип backup пока нельзя восстановить в MVP".to_owned(),
                severity: RestorePreflightIssueSeverity::Error,
            });
        }

        if !compatibility.schema_version_ok {
            errors.push(RestorePreflightIssueDto {
                code: "ERR_RESTORE_SCHEMA_VERSION_INCOMPATIBLE".to_owned(),
                message: "Версия схемы backup несовместима с текущей базой".to_owned(),
                severity: RestorePreflightIssueSeverity::Error,
            });
        }

        if !compatibility.app_version_ok {
            warnings.push(RestorePreflightIssueDto {
                code: "WARN_RESTORE_APP_VERSION_DIFFERS".to_owned(),
                message: "Версия приложения backup отличается от текущей".to_owned(),
                severity: RestorePreflightIssueSeverity::Warning,
            });
        }

        if verification.summary.error_count > 0 {
            errors.push(RestorePreflightIssueDto {
                code: "ERR_RESTORE_BACKUP_VERIFICATION_FAILED".to_owned(),
                message: "Backup не прошёл проверку целостности".to_owned(),
                severity: RestorePreflightIssueSeverity::Error,
            });
        }

        let can_restore = errors.is_empty();

        let preview = RestoreBackupMetadataPreviewDto {
            backup_type: metadata.backup_type.clone(),
            app_version: metadata.app_version.clone(),
            schema_version: metadata.schema_version,
            created_at: metadata.created_at.clone(),
            created_by: Some(metadata.created_by_username.clone()),
            case_id: None,
            case_code: None,
            file_count: manifest_file_count,
        };

        let backup_id = history_row
            .as_ref()
            .map(|row| row.id.clone())
            .or_else(|| Some(metadata.backup_id.clone()));

        let backup_code = history_row
            .as_ref()
            .map(|row| row.backup_code.clone())
            .or_else(|| Some(metadata.backup_code.clone()));

        let action = if can_restore {
            audit_action::backup::RESTORE_PREFLIGHT_CHECKED
        } else {
            audit_action::backup::RESTORE_PREFLIGHT_FAILED
        };

        Self::audit_restore_preflight(
            app,
            &ctx.current_user,
            action,
            backup_code.as_deref(),
            can_restore,
            &metadata.backup_type,
            metadata.schema_version,
            manifest_file_count,
            errors.len(),
            warnings.len(),
            verification.summary.error_count,
        );

        let file_name = file_path
            .file_name()
            .and_then(|value| value.to_str())
            .unwrap_or("backup.zip")
            .to_owned();

        Ok(RestoreBackupPreflightResponse {
            backup_id,
            backup_code,
            file_name,
            archive_sha256: verification.archive_sha256,
            checked_at,
            can_restore,
            requires_safety_backup: true,
            metadata: preview,
            compatibility,
            verification: verification.summary,
            warnings,
            errors,
        })
    }

    fn validate_path(path: &PathBuf) -> Result<(), AppErrorDto> {
        if path.to_string_lossy().starts_with("\\\\") {
            return Err(AppErrorDto::validation(
                "Сетевые UNC-пути не поддерживаются в MVP",
            ));
        }

        if path.extension().and_then(|v| v.to_str()) != Some("zip") {
            return Err(AppErrorDto::validation("Backup должен быть ZIP-архивом"));
        }

        Ok(())
    }

    fn build_compatibility(
        current_app_version: &str,
        current_schema_version: i64,
        backup_app_version: &str,
        backup_schema_version: i64,
        backup_type: &str,
    ) -> RestoreCompatibilityDto {
        RestoreCompatibilityDto {
            app_version_ok: current_app_version == backup_app_version,
            schema_version_ok: backup_schema_version <= current_schema_version,
            backup_type_ok: backup_type == "full",
            current_app_version: current_app_version.to_owned(),
            backup_app_version: backup_app_version.to_owned(),
            current_schema_version,
            backup_schema_version,
        }
    }

    fn current_schema_version(conn: &rusqlite::Connection) -> Result<i64, AppErrorDto> {
        let version: Option<i64> = conn
            .query_row(
                r#"
                SELECT version
                FROM schema_migrations
                ORDER BY version DESC
                LIMIT 1
                "#,
                [],
                |row| row.get(0),
            )
            .optional()
            .map_err(|err| AppErrorDto::database(err.to_string()))?;

        Ok(version.unwrap_or(0))
    }

    fn audit_restore_preflight(
        app: &AppHandle,
        current_user: &crate::security::session::CurrentUserDto,
        action: &str,
        backup_code: Option<&str>,
        can_restore: bool,
        backup_type: &str,
        schema_version: i64,
        file_count: usize,
        error_count: usize,
        warning_count: usize,
        verification_error_count: usize,
    ) {
        let result = (|| {
            let snapshot = safe_restore_preflight_snapshot(
                backup_code,
                can_restore,
                backup_type,
                schema_version,
                file_count,
            )?;

            let details = safe_restore_preflight_details(
                "restore_backup_preflight",
                error_count,
                warning_count,
                verification_error_count,
            )?;

            AuditService::write_best_effort(
                app,
                AuditWriteInput::success(current_user, action)
                    .with_entity("backup", backup_code.unwrap_or("unknown"))
                    .with_snapshots(None, Some(snapshot))
                    .with_details(details),
            );

            Ok::<(), AppErrorDto>(())
        })();

        if let Err(e) = result {
            eprintln!("[restore] audit_restore_preflight failed: {}", e.message);
        }
    }
}
