use std::path::{Path, PathBuf};

use chrono::Utc;
use tauri::{AppHandle, Manager};
use tauri_plugin_dialog::DialogExt;
use uuid::Uuid;

use crate::audit::audit_metadata::{
    backup_created, backup_verified, safe_backup_snapshot, safe_backup_verification_snapshot,
};
use crate::audit::audit_service::{AuditService, AuditWriteInput};
use crate::backup::{
    BackupArchiveInput, BackupArchiveReader, BackupArchiveWriter, BackupCreateType,
    BackupHistoryItemDto, BackupMetadataService, BackupRepository, CreateBackupPayload,
    CreateBackupResponse, NewBackupHistoryRow, SelectBackupFileResponse,
    SelectBackupOutputFolderResponse, VerifyBackupPayload, VerifyBackupResponse,
};
use crate::db::connection::get_database_path;
use crate::domain::audit_action;
use crate::errors::app_error::AppErrorDto;
use crate::security::{ProtectedOperation, ProtectedServiceContext};

pub struct BackupService;

impl BackupService {
    pub fn get_backup_history(app: &AppHandle) -> Result<Vec<BackupHistoryItemDto>, AppErrorDto> {
        let ctx = ProtectedServiceContext::require_operation(app, ProtectedOperation::BackupRead)?;

        BackupRepository::list_history(&ctx.conn, 100)
    }

    pub fn choose_backup_folder(
        app: &AppHandle,
    ) -> Result<SelectBackupOutputFolderResponse, AppErrorDto> {
        ProtectedServiceContext::require_operation(app, ProtectedOperation::BackupCreate)?;

        let folder = app
            .dialog()
            .file()
            .set_title("Выберите папку для резервной копии")
            .blocking_pick_folder();

        let folder_path = folder.map(|file_path| match file_path {
            tauri_plugin_dialog::FilePath::Path(p) => p.to_string_lossy().to_string(),
            tauri_plugin_dialog::FilePath::Url(u) => {
                if let Ok(p) = u.to_file_path() {
                    p.to_string_lossy().to_string()
                } else {
                    u.path().to_string()
                }
            }
        });

        Ok(SelectBackupOutputFolderResponse { folder_path })
    }

    pub fn create_backup(
        app: &AppHandle,
        payload: CreateBackupPayload,
    ) -> Result<CreateBackupResponse, AppErrorDto> {
        let ctx =
            ProtectedServiceContext::require_operation(app, ProtectedOperation::BackupCreate)?;

        Self::validate_create_payload(&payload)?;

        let backup_id = Uuid::new_v4().to_string();
        let created_at = Utc::now().to_rfc3339();
        let backup_code = Self::next_backup_code(&ctx.conn)?;
        let app_version = env!("CARGO_PKG_VERSION").to_owned();
        let schema_version = Self::current_schema_version(&ctx.conn)?;

        let output_folder = PathBuf::from(&payload.output_folder_path);
        Self::validate_output_folder(&output_folder)?;

        let file_name = format!(
            "casegraph-backup-{}.zip",
            Utc::now().format("%Y-%m-%d-%H-%M")
        );
        let output_file_path = output_folder.join(&file_name);

        let app_data_dir = app
            .path()
            .app_data_dir()
            .map_err(|err| AppErrorDto::filesystem(err.to_string()))?;

        let database_path = get_database_path(app)?;
        let templates_dir = app_data_dir.join("templates");

        let metadata = BackupMetadataService::build_metadata(
            &backup_id,
            &backup_code,
            &created_at,
            &ctx.current_user.user_id,
            &ctx.current_user.username,
            &ctx.current_user.role,
            &app_version,
            schema_version,
        );

        let archive_result = BackupArchiveWriter::create_full_backup(BackupArchiveInput {
            output_file_path: output_file_path.clone(),
            database_path,
            data_dir: app_data_dir,
            templates_dir: Some(templates_dir),
            metadata,
            include_templates: payload.include_templates,
            include_exports: payload.include_exports,
            include_audit_log: payload.include_audit_log,
        })?;

        BackupRepository::insert_history(
            &ctx.conn,
            &NewBackupHistoryRow {
                id: backup_id.clone(),
                backup_code: backup_code.clone(),
                backup_type: "full".to_owned(),
                status: "created".to_owned(),
                file_path: output_file_path.to_string_lossy().to_string(),
                file_name: file_name.clone(),
                file_size: archive_result.file_size,
                sha256: archive_result.archive_sha256.clone(),
                case_id: None,
                case_code: None,
                app_version,
                schema_version,
                created_by: ctx.current_user.user_id.clone(),
                created_at: created_at.clone(),
                metadata_json: archive_result.metadata_json,
            },
        )?;

        Self::audit_backup_created(
            app,
            &ctx.current_user,
            &backup_code,
            archive_result.file_size,
            &archive_result.archive_sha256,
            &created_at,
        );

        Ok(CreateBackupResponse {
            backup_id,
            backup_code,
            file_name,
            file_size: archive_result.file_size,
            sha256: archive_result.archive_sha256,
            created_at,
        })
    }

    fn validate_create_payload(payload: &CreateBackupPayload) -> Result<(), AppErrorDto> {
        if payload.backup_type != BackupCreateType::Full {
            return Err(AppErrorDto::validation(
                "В этом срезе поддерживается только полный backup",
            ));
        }

        if payload.output_folder_path.trim().is_empty() {
            return Err(AppErrorDto::validation("Не выбрана папка для backup"));
        }

        Ok(())
    }

    fn validate_output_folder(path: &Path) -> Result<(), AppErrorDto> {
        if path.to_string_lossy().starts_with("\\\\") {
            return Err(AppErrorDto::validation(
                "Сетевые UNC-пути не поддерживаются в MVP",
            ));
        }

        if !path.exists() {
            std::fs::create_dir_all(path)
                .map_err(|err| AppErrorDto::filesystem(err.to_string()))?;
        }

        if !path.is_dir() {
            return Err(AppErrorDto::validation("Путь backup должен быть папкой"));
        }

        Ok(())
    }

    fn next_backup_code(conn: &rusqlite::Connection) -> Result<String, AppErrorDto> {
        let count: i64 = conn
            .query_row("SELECT COUNT(*) FROM backup_history", [], |row| row.get(0))
            .map_err(|err| AppErrorDto::database(err.to_string()))?;

        Ok(format!("BKP-{:03}", count + 1))
    }

    fn current_schema_version(conn: &rusqlite::Connection) -> Result<i64, AppErrorDto> {
        let version: i64 = conn
            .query_row(
                "SELECT COALESCE(MAX(version), 0) FROM schema_migrations",
                [],
                |row| row.get(0),
            )
            .map_err(|err| AppErrorDto::database(err.to_string()))?;

        Ok(version)
    }

    pub fn choose_backup_file(app: &AppHandle) -> Result<SelectBackupFileResponse, AppErrorDto> {
        ProtectedServiceContext::require_operation(app, ProtectedOperation::BackupVerify)?;

        let file = app
            .dialog()
            .file()
            .set_title("Выберите backup ZIP")
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

        Ok(SelectBackupFileResponse { file_path })
    }

    pub fn verify_backup(
        app: &AppHandle,
        payload: VerifyBackupPayload,
    ) -> Result<VerifyBackupResponse, AppErrorDto> {
        let ctx =
            ProtectedServiceContext::require_operation(app, ProtectedOperation::BackupVerify)?;

        let checked_at = Utc::now().to_rfc3339();

        let history_row = match payload.backup_id.as_deref() {
            Some(backup_id) => BackupRepository::find_private_by_id(&ctx.conn, backup_id)?,
            None => None,
        };

        let file_path = match (&history_row, payload.file_path.as_deref()) {
            (Some(row), _) => PathBuf::from(&row.file_path),
            (None, Some(path)) if !path.trim().is_empty() => PathBuf::from(path),
            _ => return Err(AppErrorDto::validation("Не выбран backup для проверки")),
        };

        if !file_path.exists() {
            if let Some(row) = &history_row {
                BackupRepository::update_verification_result(
                    &ctx.conn,
                    &row.id,
                    "failed",
                    "",
                    &checked_at,
                    "{}",
                )?;
            }

            return Err(AppErrorDto::not_found(
                "Backup-файл не найден по сохранённому пути",
            ));
        }

        Self::validate_backup_file_path(&file_path)?;

        let verification = BackupArchiveReader::verify(&file_path)?;

        let metadata_backup_id = verification
            .metadata
            .as_ref()
            .map(|metadata| metadata.backup_id.clone());

        let backup_id_to_update = history_row
            .as_ref()
            .map(|row| row.id.clone())
            .or(metadata_backup_id);

        let backup_code = history_row
            .as_ref()
            .map(|row| row.backup_code.clone())
            .or_else(|| {
                verification
                    .metadata
                    .as_ref()
                    .map(|metadata| metadata.backup_code.clone())
            });

        let backup_type = history_row
            .as_ref()
            .map(|row| row.backup_type.clone())
            .or_else(|| {
                verification
                    .metadata
                    .as_ref()
                    .map(|metadata| metadata.backup_type.clone())
            })
            .unwrap_or_else(|| "full".to_owned());

        let is_valid = verification.summary.error_count == 0;
        let status = if is_valid { "verified" } else { "failed" };

        let verification_json = serde_json::to_string_pretty(&serde_json::json!({
            "checkedAt": checked_at,
            "isValid": is_valid,
            "summary": verification.summary,
            "issues": verification.issues,
        }))
        .map_err(|err| AppErrorDto::internal(err.to_string()))?;

        if let Some(backup_id) = &backup_id_to_update {
            if BackupRepository::find_private_by_id(&ctx.conn, backup_id)?.is_some() {
                BackupRepository::update_verification_result(
                    &ctx.conn,
                    backup_id,
                    status,
                    &verification.archive_sha256,
                    &checked_at,
                    &verification_json,
                )?;
            }
        }

        let action = if is_valid {
            audit_action::backup::VERIFIED
        } else {
            audit_action::backup::VERIFICATION_FAILED
        };

        Self::audit_backup_verified(
            app,
            &ctx.current_user,
            &action,
            backup_code.as_deref(),
            &backup_type,
            is_valid,
            &verification.archive_sha256,
            verification.summary.error_count,
            &checked_at,
        );

        let file_name = file_path
            .file_name()
            .and_then(|value| value.to_str())
            .unwrap_or("backup.zip")
            .to_owned();

        Ok(VerifyBackupResponse {
            backup_id: backup_id_to_update,
            backup_code,
            file_name,
            archive_sha256: verification.archive_sha256,
            checked_at,
            is_valid,
            summary: verification.summary,
            issues: verification.issues,
        })
    }

    fn validate_backup_file_path(path: &PathBuf) -> Result<(), AppErrorDto> {
        if path.to_string_lossy().starts_with("\\\\") {
            return Err(AppErrorDto::validation(
                "Сетевые UNC-пути не поддерживаются в MVP",
            ));
        }

        if path.extension().and_then(|value| value.to_str()) != Some("zip") {
            return Err(AppErrorDto::validation("Backup должен быть ZIP-архивом"));
        }

        Ok(())
    }

    fn audit_backup_verified(
        app: &AppHandle,
        current_user: &crate::security::session::CurrentUserDto,
        action: &str,
        backup_code: Option<&str>,
        backup_type: &str,
        is_valid: bool,
        archive_sha256: &str,
        error_count: usize,
        checked_at: &str,
    ) {
        let result = (|| {
            let snapshot = safe_backup_verification_snapshot(
                backup_code,
                backup_type,
                is_valid,
                Some(archive_sha256),
                None,
                error_count,
                0,
                Some(checked_at),
            )?;

            let details = backup_verified(
                backup_code.unwrap_or(""),
                backup_type,
                is_valid,
                error_count,
                0,
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
            eprintln!("[backup] audit_backup_verified failed: {}", e.message);
        }
    }

    fn audit_backup_created(
        app: &AppHandle,
        current_user: &crate::security::session::CurrentUserDto,
        backup_code: &str,
        file_size: i64,
        sha256: &str,
        created_at: &str,
    ) {
        let result = (|| {
            let details = backup_created(backup_code, "full", None, Some(file_size))?;

            let snapshot = safe_backup_snapshot(
                Some(backup_code),
                "full",
                "created",
                None,
                Some(env!("CARGO_PKG_VERSION")),
                None,
                Some(file_size),
                Some(sha256),
                None,
                Some(created_at),
                None,
            )?;

            AuditService::write_best_effort(
                app,
                AuditWriteInput::success(
                    current_user,
                    crate::domain::audit_action::backup::CREATED,
                )
                .with_entity("backup", backup_code)
                .with_snapshots(None, Some(snapshot))
                .with_details(details),
            );

            Ok::<(), AppErrorDto>(())
        })();

        if let Err(e) = result {
            eprintln!("[backup] audit_backup_created failed: {}", e.message);
        }
    }
}
