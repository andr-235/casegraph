use std::path::{Path, PathBuf};

use chrono::Utc;
use tauri::{AppHandle, Manager};
use tauri_plugin_dialog::DialogExt;
use uuid::Uuid;

use crate::audit::audit_metadata::{backup_created, safe_backup_snapshot};
use crate::audit::audit_service::{AuditService, AuditWriteInput};
use crate::backup::{
    BackupArchiveInput, BackupArchiveWriter, BackupCreateType, BackupHistoryItemDto,
    BackupMetadataService, BackupRepository, CreateBackupPayload, CreateBackupResponse,
    NewBackupHistoryRow, SelectBackupOutputFolderResponse,
};
use crate::db::connection::get_database_path;
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
