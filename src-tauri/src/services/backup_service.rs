use std::time::{SystemTime, UNIX_EPOCH};

use serde::Serialize;
use tauri::AppHandle;

use crate::errors::app_error::AppErrorDto;
use crate::security::session::{CurrentUserDto, SessionState};
use crate::security::ProtectedOperation;
use crate::security::ProtectedServiceContext;

fn now_epoch_secs() -> String {
    SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .map(|d| d.as_secs().to_string())
        .unwrap_or_else(|_| "0".to_string())
}

#[derive(Debug, Clone, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct CreateBackupPayload {
    pub comment: Option<String>,
    pub include_audit_logs: Option<bool>,
}

#[derive(Debug, Clone, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct CreateBackupResponse {
    pub backup_id: String,
    pub file_name: String,
    pub file_size_bytes: i64,
    pub created_at: String,
}

#[derive(Debug, Clone, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct RestoreBackupPayload {
    pub backup_id: String,
}

#[derive(Debug, Clone, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct RestoreBackupResponse {
    pub success: bool,
    pub restored_at: String,
}

pub struct BackupService;

impl BackupService {
    pub fn create_backup(
        app: &AppHandle,
        _session: &SessionState,
        _payload: CreateBackupPayload,
    ) -> Result<CreateBackupResponse, AppErrorDto> {
        let context =
            ProtectedServiceContext::require_operation(app, ProtectedOperation::BackupCreate)?;
        let _conn = &context.conn;

        // Backup generation logic will be implemented in a later slice.
        // For now, enforce policy and return a stub response.
        let backup_id = uuid::Uuid::new_v4().to_string();
        let now = now_epoch_secs();

        let response = CreateBackupResponse {
            backup_id: backup_id.clone(),
            file_name: format!("casegraph-backup-{backup_id}.sqlite"),
            file_size_bytes: 0,
            created_at: now,
        };

        // ── Audit ──
        Self::audit_backup_created(app, &context.current_user, &response);

        Ok(response)
    }

    pub fn create_case_backup(
        app: &AppHandle,
        _session: &SessionState,
        case_id: String,
    ) -> Result<CreateBackupResponse, AppErrorDto> {
        let context =
            ProtectedServiceContext::require_operation(app, ProtectedOperation::BackupCreate)?;
        let _conn = &context.conn;

        let backup_id = uuid::Uuid::new_v4().to_string();
        let now = now_epoch_secs();

        let response = CreateBackupResponse {
            backup_id: backup_id.clone(),
            file_name: format!("casegraph-case-backup-{case_id}-{backup_id}.sqlite"),
            file_size_bytes: 0,
            created_at: now,
        };

        // ── Audit ──
        Self::audit_case_backup_created(app, &context.current_user, &case_id, &response);

        Ok(response)
    }

    pub fn restore_backup(
        app: &AppHandle,
        _session: &SessionState,
        _payload: RestoreBackupPayload,
    ) -> Result<RestoreBackupResponse, AppErrorDto> {
        let context =
            ProtectedServiceContext::require_operation(app, ProtectedOperation::BackupRestore)?;
        let _conn = &context.conn;

        let now = now_epoch_secs();

        Ok(RestoreBackupResponse {
            success: true,
            restored_at: now,
        })
    }

    pub fn create_safety_backup_before_restore(
        app: &AppHandle,
        _session: &SessionState,
    ) -> Result<CreateBackupResponse, AppErrorDto> {
        // Safety backup before restore — internal operation, administrator-only
        let context =
            ProtectedServiceContext::require_operation(app, ProtectedOperation::BackupRestore)?;
        let _conn = &context.conn;

        let backup_id = uuid::Uuid::new_v4().to_string();
        let now = now_epoch_secs();

        let response = CreateBackupResponse {
            backup_id: backup_id.clone(),
            file_name: format!("casegraph-safety-{backup_id}.sqlite"),
            file_size_bytes: 0,
            created_at: now,
        };

        Ok(response)
    }

    // ─── Audit helpers ─────────────────────────────────────────────────

    fn audit_backup_created(
        app: &AppHandle,
        current_user: &CurrentUserDto,
        response: &CreateBackupResponse,
    ) {
        let result = (|| {
            let details = crate::audit::audit_metadata::backup_created(
                &response.backup_id,
                "full",
                None,
                Some(response.file_size_bytes),
            )?;

            let snapshot = crate::audit::audit_metadata::safe_backup_snapshot(
                Some(&response.backup_id),
                "full",
                "created",
                None,
                None,
                None,
                Some(response.file_size_bytes),
                None,
                None,
                Some(&response.created_at),
                None,
            )?;

            crate::audit::audit_service::AuditService::write_best_effort(
                app,
                crate::audit::audit_service::AuditWriteInput::success(
                    current_user,
                    crate::domain::audit_action::backup::CREATED,
                )
                .with_entity("backup", response.backup_id.clone())
                .with_snapshots(None, Some(snapshot))
                .with_details(details),
            );
            Ok::<(), AppErrorDto>(())
        })();

        if let Err(e) = result {
            eprintln!("[audit] backup audit failed: {}", e.message);
        }
    }

    fn audit_case_backup_created(
        app: &AppHandle,
        current_user: &CurrentUserDto,
        case_id: &str,
        response: &CreateBackupResponse,
    ) {
        let result = (|| {
            let details = crate::audit::audit_metadata::backup_created(
                &response.backup_id,
                "case",
                Some(case_id),
                Some(response.file_size_bytes),
            )?;

            let snapshot = crate::audit::audit_metadata::safe_backup_snapshot(
                Some(&response.backup_id),
                "case",
                "created",
                Some(case_id),
                None,
                None,
                Some(response.file_size_bytes),
                None,
                None,
                Some(&response.created_at),
                None,
            )?;

            crate::audit::audit_service::AuditService::write_best_effort(
                app,
                crate::audit::audit_service::AuditWriteInput::success(
                    current_user,
                    crate::domain::audit_action::backup::CASE_CREATED,
                )
                .with_case_id(case_id)
                .with_entity("backup", response.backup_id.clone())
                .with_snapshots(None, Some(snapshot))
                .with_details(details),
            );
            Ok::<(), AppErrorDto>(())
        })();

        if let Err(e) = result {
            eprintln!("[audit] case backup audit failed: {}", e.message);
        }
    }
}
