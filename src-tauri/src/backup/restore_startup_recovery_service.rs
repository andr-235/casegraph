use crate::audit::audit_service::{AuditService, AuditWriteInput};
use crate::backup::{
    ResolveRestoreRecoveryPayload, ResolveRestoreRecoveryResponse, RestoreMaintenanceService,
    RestoreOperationPhase, RestoreRecoveryActionDto, RestoreRecoveryStatusDto,
};
use crate::domain::audit_action;
use crate::errors::app_error::AppErrorDto;
use crate::security::{ProtectedOperation, ProtectedServiceContext};

pub struct RestoreStartupRecoveryService;

impl RestoreStartupRecoveryService {
    pub fn get_status(app: &tauri::AppHandle) -> Result<RestoreRecoveryStatusDto, AppErrorDto> {
        let state = RestoreMaintenanceService::read_state(app)?;
        let lock_exists = RestoreMaintenanceService::lock_path(app)?.exists();

        let Some(state) = state else {
            return Ok(RestoreRecoveryStatusDto {
                recovery_required: lock_exists,
                operation_id: None,
                phase: None,
                restore_backup_code: None,
                safety_backup_code: None,
                started_at: None,
                updated_at: None,
                last_error_code: None,
                recommended_action: if lock_exists {
                    RestoreRecoveryActionDto::ManualReviewRequired
                } else {
                    RestoreRecoveryActionDto::None
                },
            });
        };

        let recommended_action = match state.phase {
            RestoreOperationPhase::CompletedRequiresRestart
            | RestoreOperationPhase::RollbackCompleted => {
                RestoreRecoveryActionDto::CleanupCompletedRestore
            }
            RestoreOperationPhase::Started
            | RestoreOperationPhase::StagingExtracted
            | RestoreOperationPhase::StagingValidated
            | RestoreOperationPhase::RollbackPrepared
            | RestoreOperationPhase::ReplacingLiveData
            | RestoreOperationPhase::LiveDataReplaced
            | RestoreOperationPhase::FailedNeedsRecovery
            | RestoreOperationPhase::RollbackStarted => {
                RestoreRecoveryActionDto::RollbackInterruptedRestore
            }
        };

        Ok(RestoreRecoveryStatusDto {
            recovery_required: true,
            operation_id: Some(state.operation_id),
            phase: Some(format!("{:?}", state.phase)),
            restore_backup_code: state.restore_backup_code,
            safety_backup_code: Some(state.safety_backup_code),
            started_at: Some(state.started_at),
            updated_at: Some(state.updated_at),
            last_error_code: state.last_error_code,
            recommended_action,
        })
    }

    pub fn resolve(
        app: &tauri::AppHandle,
        payload: ResolveRestoreRecoveryPayload,
    ) -> Result<ResolveRestoreRecoveryResponse, AppErrorDto> {
        let ctx =
            ProtectedServiceContext::require_operation(app, ProtectedOperation::BackupRestore)?;

        if payload.confirmation_phrase.trim() != "RECOVERY" {
            return Err(AppErrorDto::validation(
                "Для recovery нужно ввести подтверждение: RECOVERY",
            ));
        }

        let status = Self::get_status(app)?;

        if !status.recovery_required {
            return Ok(ResolveRestoreRecoveryResponse {
                resolved: true,
                action: "none".to_owned(),
                requires_restart: false,
                message: "Restore recovery не требуется.".to_owned(),
            });
        }

        let _ = crate::audit::audit_metadata::restore_recovery_snapshot(
            status.operation_id.as_deref(),
            status.phase.as_deref(),
            status.restore_backup_code.as_deref(),
            status.safety_backup_code.as_deref(),
            status.last_error_code.as_deref(),
        );

        AuditService::write_best_effort(
            app,
            AuditWriteInput::success(
                &ctx.current_user,
                audit_action::backup::RESTORE_RECOVERY_DETECTED,
            )
            .with_entity("restore", status.operation_id.clone().unwrap_or_default())
            .with_snapshots(None, None)
            .with_entity_type("restore".to_owned()),
        );

        match payload.action.as_str() {
            "cleanup_completed_restore" => Self::cleanup_completed_restore(app),
            "rollback_interrupted_restore" => Self::rollback_interrupted_restore(app),
            _ => Err(AppErrorDto::validation(
                "Неизвестное restore recovery действие",
            )),
        }
    }

    fn cleanup_completed_restore(
        app: &tauri::AppHandle,
    ) -> Result<ResolveRestoreRecoveryResponse, AppErrorDto> {
        RestoreMaintenanceService::cleanup_completed_restore(app)?;

        Ok(ResolveRestoreRecoveryResponse {
            resolved: true,
            action: "cleanup_completed_restore".to_owned(),
            requires_restart: false,
            message: "Restore cleanup выполнен. Можно продолжить работу.".to_owned(),
        })
    }

    fn rollback_interrupted_restore(
        app: &tauri::AppHandle,
    ) -> Result<ResolveRestoreRecoveryResponse, AppErrorDto> {
        let state = RestoreMaintenanceService::read_state(app)?
            .ok_or_else(|| AppErrorDto::validation("Restore state не найден"))?;

        RestoreMaintenanceService::update_phase(app, RestoreOperationPhase::RollbackStarted, None)?;

        crate::backup::RestoreService::rollback_by_operation_id(app, &state.operation_id)?;

        RestoreMaintenanceService::update_phase(
            app,
            RestoreOperationPhase::RollbackCompleted,
            None,
        )?;

        RestoreMaintenanceService::cleanup_completed_restore(app)?;

        Ok(ResolveRestoreRecoveryResponse {
            resolved: true,
            action: "rollback_interrupted_restore".to_owned(),
            requires_restart: true,
            message: "Rollback выполнен. Перезапустите приложение.".to_owned(),
        })
    }
}
