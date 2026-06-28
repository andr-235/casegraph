use std::path::PathBuf;

use chrono::Utc;
use rusqlite::OptionalExtension;
use tauri::AppHandle;
use tauri::Manager;
use tauri_plugin_dialog::DialogExt;

use crate::audit::audit_metadata::{
    restore_completed_details, restore_completed_snapshot, restore_failed_details,
    restore_failed_snapshot, restore_started_details, restore_started_snapshot,
    safe_restore_preflight_details, safe_restore_preflight_snapshot, safe_safety_backup_details,
    safe_safety_backup_failed_details, safe_safety_backup_failed_snapshot,
    safe_safety_backup_snapshot,
};
use crate::audit::audit_service::{AuditService, AuditWriteInput};
use crate::backup::{
    BackupArchiveReader, BackupPathResolver, BackupRepository, BackupService,
    CreateRestoreSafetyBackupPayload, CreateRestoreSafetyBackupResponse,
    InternalCreateBackupRequest, RestoreBackupMetadataPreviewDto, RestoreBackupPayload,
    RestoreBackupPreflightPayload, RestoreBackupPreflightResponse, RestoreBackupResponse,
    RestoreCompatibilityDto, RestoreOperationPaths, RestorePreflightIssueDto,
    RestorePreflightIssueSeverity, RestoreSafetyBackupCheck, RestoreSafetyTargetDto,
    SelectRestoreBackupFileResponse,
};
use crate::db::connection::get_database_path;
use crate::domain::audit_action;
use crate::errors::app_error::AppErrorDto;
use crate::security::session::CurrentUserDto;
use crate::security::{ProtectedOperation, ProtectedServiceContext};

pub struct RestoreService;

struct RestoreTarget {
    pub backup_id: Option<String>,
    pub backup_code: Option<String>,
    pub file_path: PathBuf,
}

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

    // ── Safety backup ──────────────────────────────────────────────────────

    pub fn create_restore_safety_backup(
        app: &AppHandle,
        payload: CreateRestoreSafetyBackupPayload,
    ) -> Result<CreateRestoreSafetyBackupResponse, AppErrorDto> {
        let ctx =
            ProtectedServiceContext::require_operation(app, ProtectedOperation::BackupRestore)?;

        if payload.restore_archive_sha256.trim().is_empty() {
            return Err(AppErrorDto::validation(
                "Не передан SHA-256 backup, который планируется восстановить",
            ));
        }

        // 1. Re-resolve restore target.
        let restore_history_row = match payload.restore_backup_id.as_deref() {
            Some(backup_id) => BackupRepository::find_private_by_id(&ctx.conn, backup_id)?,
            None => None,
        };

        let restore_file_path = match (&restore_history_row, payload.restore_file_path.as_deref()) {
            (Some(row), _) => PathBuf::from(&row.file_path),
            (None, Some(path)) if !path.trim().is_empty() => PathBuf::from(path),
            _ => {
                return Err(AppErrorDto::validation(
                    "Не выбран backup для восстановления",
                ));
            }
        };

        let backup_id = restore_history_row.as_ref().map(|row| row.id.clone());
        let preflight_backup_code = restore_history_row
            .as_ref()
            .map(|row| row.backup_code.clone());

        // 2. Re-run preflight before safety backup.
        let preflight = Self::restore_backup_preflight(
            app,
            crate::backup::RestoreBackupPreflightPayload {
                backup_id: payload.restore_backup_id.clone(),
                file_path: payload.restore_file_path.clone(),
            },
        )?;

        if !preflight.can_restore {
            Self::audit_safety_backup_failed(
                app,
                &ctx.current_user,
                backup_id,
                preflight_backup_code,
                "ERR_RESTORE_PREFLIGHT_REQUIRED",
            );

            return Err(AppErrorDto::validation(
                "Safety backup нельзя создать: restore preflight не пройден",
            ));
        }

        if preflight.archive_sha256 != payload.restore_archive_sha256 {
            Self::audit_safety_backup_failed(
                app,
                &ctx.current_user,
                backup_id,
                preflight_backup_code,
                "ERR_RESTORE_TARGET_CHANGED",
            );

            return Err(AppErrorDto::validation(
                "Restore target изменился после preflight. Повторите проверку восстановления.",
            ));
        }

        // 3. Resolve output dir for safety backup.
        let output_dir = BackupPathResolver::resolve_safety_backup_dir(app, &ctx.conn)?;

        // 4. Create safety backup of current state.
        let result = BackupService::create_full_backup_internal(
            app,
            &ctx.conn,
            &ctx.current_user.user_id,
            InternalCreateBackupRequest {
                backup_type: "safety".to_owned(),
                output_dir,
                safety_reason: Some("before_restore".to_owned()),
                restore_target_backup_id: preflight.backup_id.clone(),
                restore_target_backup_code: preflight.backup_code.clone(),
                restore_target_archive_sha256: Some(preflight.archive_sha256.clone()),
            },
        );

        let result = match result {
            Ok(value) => value,
            Err(error) => {
                Self::audit_safety_backup_failed(
                    app,
                    &ctx.current_user,
                    preflight.backup_id.clone(),
                    preflight.backup_code.clone(),
                    &error.code,
                );

                return Err(error);
            }
        };

        // 5. Audit success.
        let audit_result = (|| {
            let snapshot = safe_safety_backup_snapshot(
                &result.backup_code,
                &result.archive_sha256,
                result.file_size,
                preflight.backup_code.as_deref(),
                &preflight.archive_sha256,
            )?;

            let details = safe_safety_backup_details("before_restore", true)?;

            AuditService::write_best_effort(
                app,
                AuditWriteInput::success(
                    &ctx.current_user,
                    crate::domain::audit_action::backup::SAFETY_BACKUP_CREATED,
                )
                .with_entity("backup", &result.backup_code)
                .with_snapshots(None, Some(snapshot))
                .with_details(details),
            );

            Ok::<(), AppErrorDto>(())
        })();

        if let Err(e) = audit_result {
            eprintln!("[restore] safety backup audit failed: {}", e.message);
        }

        let restore_file_name = restore_file_path
            .file_name()
            .and_then(|value| value.to_str())
            .unwrap_or("backup.zip")
            .to_owned();

        Ok(CreateRestoreSafetyBackupResponse {
            safety_backup_id: result.backup_id,
            safety_backup_code: result.backup_code,
            safety_file_name: result.file_name,
            safety_archive_sha256: result.archive_sha256,
            safety_file_size: result.file_size,
            created_at: result.created_at,
            restore_target: RestoreSafetyTargetDto {
                backup_id: preflight.backup_id,
                backup_code: preflight.backup_code,
                file_name: restore_file_name,
                archive_sha256: preflight.archive_sha256,
            },
            can_continue_to_restore: true,
        })
    }

    fn audit_safety_backup_failed(
        app: &AppHandle,
        current_user: &crate::security::session::CurrentUserDto,
        restore_backup_id: Option<String>,
        restore_backup_code: Option<String>,
        error_code: &str,
    ) {
        let result = (|| {
            let snapshot = safe_safety_backup_failed_snapshot(restore_backup_code.as_deref())?;

            let details = safe_safety_backup_failed_details("before_restore", error_code)?;

            AuditService::write_best_effort(
                app,
                AuditWriteInput::failure(
                    current_user,
                    crate::domain::audit_action::backup::SAFETY_BACKUP_FAILED,
                )
                .with_entity(
                    "backup",
                    restore_backup_id.unwrap_or_else(|| "unknown".to_owned()),
                )
                .with_snapshots(None, Some(snapshot))
                .with_details(details),
            );

            Ok::<(), AppErrorDto>(())
        })();

        if let Err(e) = result {
            eprintln!("[restore] audit_safety_backup_failed: {}", e.message);
        }
    }

    // ── Restore backup execution ────────────────────────────────────────────

    pub fn restore_backup(
        app: &AppHandle,
        payload: RestoreBackupPayload,
    ) -> Result<RestoreBackupResponse, AppErrorDto> {
        let ctx =
            ProtectedServiceContext::require_operation(app, ProtectedOperation::BackupRestore)?;

        Self::validate_restore_confirmation(&payload)?;

        let started_at = Utc::now().to_rfc3339();
        let operation_id = uuid::Uuid::new_v4().to_string();

        // 1. Safety backup check first.
        let safety = Self::verify_safety_backup(
            &ctx.conn,
            &payload.safety_backup_id,
            &payload.safety_archive_sha256,
        )?;

        // 2. Resolve restore target.
        let restore_target = Self::resolve_restore_target(
            &ctx.conn,
            payload.restore_backup_id.clone(),
            payload.restore_file_path.clone(),
        )?;

        // 3. Re-run preflight. Do not trust old UI state.
        let preflight = Self::restore_backup_preflight(
            app,
            RestoreBackupPreflightPayload {
                backup_id: payload.restore_backup_id.clone(),
                file_path: payload.restore_file_path.clone(),
            },
        )?;

        if !preflight.can_restore {
            return Err(AppErrorDto::validation(
                "Restore запрещён: preflight больше не проходит",
            ));
        }

        if preflight.archive_sha256 != payload.restore_archive_sha256 {
            return Err(AppErrorDto::validation(
                "Restore archive изменился после preflight. Повторите проверку.",
            ));
        }

        // 4. Audit started before destructive stage.
        // Extract user data before dropping ctx.
        let user_id = ctx.current_user.user_id.clone();
        let username = ctx.current_user.username.clone();
        let user_role = ctx.current_user.role.clone();
        let user_dto = CurrentUserDto {
            user_id,
            username,
            display_name: ctx.current_user.display_name.clone(),
            role: user_role,
            is_active: ctx.current_user.is_active,
            must_change_password: ctx.current_user.must_change_password,
        };

        let _ = Self::audit_restore_started(
            app,
            &user_dto,
            &operation_id,
            preflight.backup_code.as_deref(),
            &preflight.archive_sha256,
            &safety.backup_code,
            &safety.archive_sha256,
        );

        // 5. Build paths and restore lock.
        let paths = Self::build_restore_operation_paths(app, &operation_id)?;
        Self::create_restore_lock(&paths)?;

        // 6. Close live DB connection before destructive file operations.
        drop(ctx);

        let restore_result = Self::execute_restore_with_rollback(
            app,
            &paths,
            &restore_target.file_path,
            &operation_id,
        );

        Self::remove_restore_lock_best_effort(&paths);

        match restore_result {
            Ok(_) => {
                let completed_at = Utc::now().to_rfc3339();

                let _ = BackupRepository::mark_restored(
                    &crate::db::connection::open_connection(app)?,
                    preflight.backup_id.as_deref(),
                    &completed_at,
                );

                // Write RESTORE_COMPLETED to the restored DB.
                if let Ok(restored_conn) = crate::db::connection::open_connection(app) {
                    let _ = Self::audit_restore_completed_with_conn(
                        &restored_conn,
                        &user_dto,
                        &operation_id,
                        preflight.backup_code.as_deref(),
                        &preflight.archive_sha256,
                        &safety.backup_code,
                        &safety.archive_sha256,
                    );
                }

                Ok(RestoreBackupResponse {
                    restore_operation_id: operation_id,
                    restored_backup_id: preflight.backup_id,
                    restored_backup_code: preflight.backup_code,
                    restored_archive_sha256: preflight.archive_sha256,
                    safety_backup_id: safety.backup_id,
                    safety_backup_code: safety.backup_code,
                    safety_archive_sha256: safety.archive_sha256,
                    started_at,
                    completed_at,
                    restored_database: true,
                    restored_app_data: true,
                    requires_restart: true,
                    message: "Восстановление выполнено. Перезапустите приложение.".to_owned(),
                })
            }
            Err(error) => {
                let _ = Self::audit_restore_failed(
                    app,
                    &user_dto,
                    &operation_id,
                    preflight.backup_code.as_deref(),
                    &error.code,
                );

                Err(error)
            }
        }
    }

    fn validate_restore_confirmation(payload: &RestoreBackupPayload) -> Result<(), AppErrorDto> {
        if payload.confirmation_phrase.trim() != "ВОССТАНОВИТЬ" {
            return Err(AppErrorDto::validation(
                "Для восстановления нужно ввести подтверждение: ВОССТАНОВИТЬ",
            ));
        }

        if payload.safety_backup_id.trim().is_empty() {
            return Err(AppErrorDto::validation(
                "Restore запрещён: не передан safety backup",
            ));
        }

        if payload.restore_archive_sha256.trim().is_empty() {
            return Err(AppErrorDto::validation(
                "Restore запрещён: не передан SHA-256 restore archive",
            ));
        }

        if payload.safety_archive_sha256.trim().is_empty() {
            return Err(AppErrorDto::validation(
                "Restore запрещён: не передан SHA-256 safety backup",
            ));
        }

        Ok(())
    }

    fn verify_safety_backup(
        conn: &rusqlite::Connection,
        safety_backup_id: &str,
        expected_sha256: &str,
    ) -> Result<RestoreSafetyBackupCheck, AppErrorDto> {
        let row = BackupRepository::find_private_by_id(conn, safety_backup_id)?
            .ok_or_else(|| AppErrorDto::validation("Safety backup не найден"))?;

        if row.backup_type != "safety" {
            return Err(AppErrorDto::validation(
                "Переданный backup не является safety backup",
            ));
        }

        if row.status != "created" && row.status != "verified" {
            return Err(AppErrorDto::validation(
                "Safety backup должен иметь статус created или verified",
            ));
        }

        if row.sha256 != expected_sha256 {
            return Err(AppErrorDto::validation(
                "SHA-256 safety backup не совпадает с ожидаемым",
            ));
        }

        let file_path = PathBuf::from(&row.file_path);

        if !file_path.exists() {
            return Err(AppErrorDto::validation("Файл safety backup не найден"));
        }

        let actual_sha256 = BackupArchiveReader::sha256_file(&file_path)?;

        if actual_sha256 != expected_sha256 {
            return Err(AppErrorDto::validation(
                "Файл safety backup изменился после создания",
            ));
        }

        Ok(RestoreSafetyBackupCheck {
            backup_id: row.id,
            backup_code: row.backup_code,
            file_path,
            archive_sha256: actual_sha256,
        })
    }

    fn resolve_restore_target(
        conn: &rusqlite::Connection,
        backup_id: Option<String>,
        file_path: Option<String>,
    ) -> Result<RestoreTarget, AppErrorDto> {
        if let Some(backup_id) = backup_id {
            let row = BackupRepository::find_private_by_id(conn, &backup_id)?
                .ok_or_else(|| AppErrorDto::validation("Backup для восстановления не найден"))?;

            return Ok(RestoreTarget {
                backup_id: Some(row.id),
                backup_code: Some(row.backup_code),
                file_path: PathBuf::from(row.file_path),
            });
        }

        let Some(raw_path) = file_path else {
            return Err(AppErrorDto::validation(
                "Не выбран backup для восстановления",
            ));
        };

        if raw_path.trim().is_empty() {
            return Err(AppErrorDto::validation(
                "Не выбран backup для восстановления",
            ));
        }

        let path = PathBuf::from(raw_path);

        if !path.exists() {
            return Err(AppErrorDto::validation(
                "Backup-файл для восстановления не найден",
            ));
        }

        if path.to_string_lossy().starts_with("\\\\") {
            return Err(AppErrorDto::validation(
                "Restore из сетевого UNC-пути не поддерживается",
            ));
        }

        Ok(RestoreTarget {
            backup_id: None,
            backup_code: None,
            file_path: path,
        })
    }

    fn build_restore_operation_paths(
        app: &AppHandle,
        operation_id: &str,
    ) -> Result<RestoreOperationPaths, AppErrorDto> {
        let app_data_dir = app
            .path()
            .app_data_dir()
            .map_err(|err| AppErrorDto::filesystem(err.to_string()))?;

        let live_database_path = get_database_path(app)?;

        Ok(RestoreOperationPaths {
            operation_id: operation_id.to_owned(),
            staging_dir: app_data_dir.join("restore-staging").join(operation_id),
            rollback_dir: app_data_dir.join("restore-rollback").join(operation_id),
            restore_lock_file: app_data_dir.join("restore.lock"),
            live_database_path: live_database_path.clone(),
            staged_database_path: app_data_dir
                .join("restore-staging")
                .join(operation_id)
                .join("database")
                .join("casegraph.sqlite"),
            live_data_dir: app_data_dir.join("data"),
            staged_data_dir: app_data_dir
                .join("restore-staging")
                .join(operation_id)
                .join("data"),
            live_thumbnails_dir: app_data_dir.join("thumbnails"),
            staged_thumbnails_dir: app_data_dir
                .join("restore-staging")
                .join(operation_id)
                .join("thumbnails"),
            live_exports_dir: app_data_dir.join("exports"),
            staged_exports_dir: app_data_dir
                .join("restore-staging")
                .join(operation_id)
                .join("exports"),
            live_templates_dir: app_data_dir.join("templates"),
            staged_templates_dir: app_data_dir
                .join("restore-staging")
                .join(operation_id)
                .join("templates"),
        })
    }

    fn create_restore_lock(paths: &RestoreOperationPaths) -> Result<(), AppErrorDto> {
        if let Some(parent) = paths.restore_lock_file.parent() {
            std::fs::create_dir_all(parent)
                .map_err(|err| AppErrorDto::filesystem(err.to_string()))?;
        }

        std::fs::write(
            &paths.restore_lock_file,
            format!("restore_operation_id={}", paths.operation_id),
        )
        .map_err(|err| AppErrorDto::filesystem(err.to_string()))?;

        Ok(())
    }

    fn remove_restore_lock_best_effort(paths: &RestoreOperationPaths) {
        let _ = std::fs::remove_file(&paths.restore_lock_file);
    }

    fn execute_restore_with_rollback(
        _app: &AppHandle,
        paths: &RestoreOperationPaths,
        restore_archive_path: &PathBuf,
        _operation_id: &str,
    ) -> Result<(), AppErrorDto> {
        // 1. Clean previous staging/rollback.
        let _ = std::fs::remove_dir_all(&paths.staging_dir);
        let _ = std::fs::remove_dir_all(&paths.rollback_dir);

        std::fs::create_dir_all(&paths.staging_dir)
            .map_err(|err| AppErrorDto::filesystem(err.to_string()))?;
        std::fs::create_dir_all(&paths.rollback_dir)
            .map_err(|err| AppErrorDto::filesystem(err.to_string()))?;

        // 2. Extract archive to staging only.
        BackupArchiveReader::extract_to_restore_staging(restore_archive_path, &paths.staging_dir)?;

        // 3. Validate staged content.
        Self::validate_staged_restore(paths)?;

        // 4. Backup current live paths into rollback dir.
        Self::prepare_rollback_copy(paths)?;

        // 5. Replace live paths.
        let replace_result = Self::replace_live_paths(paths);

        if let Err(error) = replace_result {
            let _ = Self::rollback_live_paths(paths);
            return Err(error);
        }

        // 6. Leave rollback dir for post-mortem.
        Ok(())
    }

    fn validate_staged_restore(paths: &RestoreOperationPaths) -> Result<(), AppErrorDto> {
        if !paths.staged_database_path.exists() {
            return Err(AppErrorDto::validation(
                "В backup отсутствует database/casegraph.sqlite",
            ));
        }

        let metadata_path = paths
            .staging_dir
            .join("metadata")
            .join("backup-metadata.json");
        let manifest_path = paths.staging_dir.join("metadata").join("manifest.json");
        let checksums_path = paths.staging_dir.join("metadata").join("checksums.json");

        for required in [&metadata_path, &manifest_path, &checksums_path] {
            if !required.exists() {
                return Err(AppErrorDto::validation(
                    "Restore staging не содержит обязательные metadata-файлы",
                ));
            }
        }

        Self::validate_staged_sqlite_database(&paths.staged_database_path)?;

        Ok(())
    }

    fn validate_staged_sqlite_database(db_path: &PathBuf) -> Result<(), AppErrorDto> {
        let conn = rusqlite::Connection::open_with_flags(
            db_path,
            rusqlite::OpenFlags::SQLITE_OPEN_READ_ONLY,
        )
        .map_err(|err| AppErrorDto::database(err.to_string()))?;

        let integrity: String = conn
            .query_row("PRAGMA integrity_check", [], |row| row.get(0))
            .map_err(|err| AppErrorDto::database(err.to_string()))?;

        if integrity != "ok" {
            return Err(AppErrorDto::validation(
                "SQLite database из backup не прошла PRAGMA integrity_check",
            ));
        }

        let backup_schema_version: i64 = conn
            .query_row(
                r#"
                SELECT COALESCE(MAX(version), 0)
                FROM schema_migrations
                "#,
                [],
                |row| row.get(0),
            )
            .map_err(|err| AppErrorDto::database(err.to_string()))?;

        let current_schema_version = Self::current_schema_version_from_conn(&conn)?;

        if backup_schema_version > current_schema_version {
            return Err(AppErrorDto::validation(
                "Backup создан на более новой версии схемы БД",
            ));
        }

        Ok(())
    }

    fn current_schema_version_from_conn(conn: &rusqlite::Connection) -> Result<i64, AppErrorDto> {
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

    fn prepare_rollback_copy(paths: &RestoreOperationPaths) -> Result<(), AppErrorDto> {
        std::fs::create_dir_all(&paths.rollback_dir)
            .map_err(|err| AppErrorDto::filesystem(err.to_string()))?;

        Self::copy_file_if_exists(
            &paths.live_database_path,
            &paths.rollback_dir.join("casegraph.sqlite"),
        )?;

        Self::copy_dir_if_exists(&paths.live_data_dir, &paths.rollback_dir.join("data"))?;

        Self::copy_dir_if_exists(
            &paths.live_thumbnails_dir,
            &paths.rollback_dir.join("thumbnails"),
        )?;

        Self::copy_dir_if_exists(&paths.live_exports_dir, &paths.rollback_dir.join("exports"))?;

        Self::copy_dir_if_exists(
            &paths.live_templates_dir,
            &paths.rollback_dir.join("templates"),
        )?;

        Ok(())
    }

    fn replace_live_paths(paths: &RestoreOperationPaths) -> Result<(), AppErrorDto> {
        Self::replace_file(&paths.staged_database_path, &paths.live_database_path)?;

        Self::replace_dir(&paths.staged_data_dir, &paths.live_data_dir)?;

        Self::replace_dir(&paths.staged_thumbnails_dir, &paths.live_thumbnails_dir)?;

        Self::replace_dir(&paths.staged_exports_dir, &paths.live_exports_dir)?;

        Self::replace_dir(&paths.staged_templates_dir, &paths.live_templates_dir)?;

        Ok(())
    }

    fn rollback_live_paths(paths: &RestoreOperationPaths) -> Result<(), AppErrorDto> {
        Self::replace_file(
            &paths.rollback_dir.join("casegraph.sqlite"),
            &paths.live_database_path,
        )?;

        Self::replace_dir(&paths.rollback_dir.join("data"), &paths.live_data_dir)?;

        Self::replace_dir(
            &paths.rollback_dir.join("thumbnails"),
            &paths.live_thumbnails_dir,
        )?;

        Self::replace_dir(&paths.rollback_dir.join("exports"), &paths.live_exports_dir)?;

        Self::replace_dir(
            &paths.rollback_dir.join("templates"),
            &paths.live_templates_dir,
        )?;

        Ok(())
    }

    fn replace_file(source: &PathBuf, target: &PathBuf) -> Result<(), AppErrorDto> {
        if !source.exists() {
            return Ok(());
        }

        if let Some(parent) = target.parent() {
            std::fs::create_dir_all(parent)
                .map_err(|err| AppErrorDto::filesystem(err.to_string()))?;
        }

        if target.exists() {
            std::fs::remove_file(target).map_err(|err| AppErrorDto::filesystem(err.to_string()))?;
        }

        if std::fs::rename(source, target).is_err() {
            std::fs::copy(source, target)
                .map_err(|err| AppErrorDto::filesystem(err.to_string()))?;
            let _ = std::fs::remove_file(source);
        }

        Ok(())
    }

    fn replace_dir(source: &PathBuf, target: &PathBuf) -> Result<(), AppErrorDto> {
        if !source.exists() {
            return Ok(());
        }

        if target.exists() {
            std::fs::remove_dir_all(target)
                .map_err(|err| AppErrorDto::filesystem(err.to_string()))?;
        }

        if std::fs::rename(source, target).is_err() {
            Self::copy_dir_recursive(source, target)?;
            let _ = std::fs::remove_dir_all(source);
        }

        Ok(())
    }

    fn copy_file_if_exists(source: &PathBuf, target: &PathBuf) -> Result<(), AppErrorDto> {
        if !source.exists() {
            return Ok(());
        }

        if let Some(parent) = target.parent() {
            std::fs::create_dir_all(parent)
                .map_err(|err| AppErrorDto::filesystem(err.to_string()))?;
        }

        std::fs::copy(source, target).map_err(|err| AppErrorDto::filesystem(err.to_string()))?;

        Ok(())
    }

    fn copy_dir_if_exists(source: &PathBuf, target: &PathBuf) -> Result<(), AppErrorDto> {
        if !source.exists() {
            return Ok(());
        }

        Self::copy_dir_recursive(source, target)
    }

    fn copy_dir_recursive(source: &PathBuf, target: &PathBuf) -> Result<(), AppErrorDto> {
        std::fs::create_dir_all(target).map_err(|err| AppErrorDto::filesystem(err.to_string()))?;

        for entry in walkdir::WalkDir::new(source) {
            let entry = entry.map_err(|err| AppErrorDto::filesystem(err.to_string()))?;

            let relative = entry
                .path()
                .strip_prefix(source)
                .map_err(|_| AppErrorDto::validation("Ошибка подготовки restore paths"))?;

            let destination = target.join(relative);

            if entry.file_type().is_dir() {
                std::fs::create_dir_all(&destination)
                    .map_err(|err| AppErrorDto::filesystem(err.to_string()))?;
            } else {
                if let Some(parent) = destination.parent() {
                    std::fs::create_dir_all(parent)
                        .map_err(|err| AppErrorDto::filesystem(err.to_string()))?;
                }

                std::fs::copy(entry.path(), &destination)
                    .map_err(|err| AppErrorDto::filesystem(err.to_string()))?;
            }
        }

        Ok(())
    }

    // ── Restore audit helpers ───────────────────────────────────────────────

    fn audit_restore_started(
        app: &AppHandle,
        current_user: &crate::security::session::CurrentUserDto,
        operation_id: &str,
        restore_backup_code: Option<&str>,
        restore_archive_sha256: &str,
        safety_backup_code: &str,
        safety_archive_sha256: &str,
    ) -> Result<(), AppErrorDto> {
        let snapshot = restore_started_snapshot(
            operation_id,
            restore_backup_code,
            restore_archive_sha256,
            safety_backup_code,
            safety_archive_sha256,
        )?;

        let details = restore_started_details("full_restore")?;

        AuditService::write_best_effort(
            app,
            AuditWriteInput::success(current_user, audit_action::backup::RESTORE_STARTED)
                .with_entity("backup", restore_backup_code.unwrap_or("backup"))
                .with_snapshots(None, Some(snapshot))
                .with_details(details)
                .with_entity_type("backup".to_owned()),
        );

        Ok(())
    }

    fn audit_restore_completed_with_conn(
        conn: &rusqlite::Connection,
        current_user: &crate::security::session::CurrentUserDto,
        operation_id: &str,
        restore_backup_code: Option<&str>,
        restore_archive_sha256: &str,
        safety_backup_code: &str,
        safety_archive_sha256: &str,
    ) -> Result<(), AppErrorDto> {
        let snapshot = restore_completed_snapshot(
            operation_id,
            restore_backup_code,
            restore_archive_sha256,
            safety_backup_code,
            safety_archive_sha256,
        )?;

        let details = restore_completed_details(true)?;

        AuditService::write_best_effort_with_conn(
            conn,
            AuditWriteInput::success(current_user, audit_action::backup::RESTORE_COMPLETED)
                .with_entity("backup", restore_backup_code.unwrap_or("backup"))
                .with_snapshots(None, Some(snapshot))
                .with_details(details)
                .with_entity_type("backup".to_owned()),
        );

        Ok(())
    }

    fn audit_restore_failed(
        app: &AppHandle,
        current_user: &crate::security::session::CurrentUserDto,
        operation_id: &str,
        restore_backup_code: Option<&str>,
        error_code: &str,
    ) -> Result<(), AppErrorDto> {
        let snapshot = restore_failed_snapshot(operation_id, restore_backup_code)?;

        let details = restore_failed_details(error_code)?;

        AuditService::write_best_effort(
            app,
            AuditWriteInput::failure(current_user, audit_action::backup::RESTORE_FAILED)
                .with_entity("backup", restore_backup_code.unwrap_or("backup"))
                .with_snapshots(None, Some(snapshot))
                .with_details(details)
                .with_entity_type("backup".to_owned()),
        );

        Ok(())
    }
}
