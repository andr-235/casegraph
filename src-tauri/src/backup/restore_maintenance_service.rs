use std::path::PathBuf;

use chrono::Utc;
use tauri::Manager;

use crate::backup::{RestoreOperationPhase, RestoreOperationState};
use crate::errors::app_error::AppErrorDto;

pub struct RestoreMaintenanceService;

impl RestoreMaintenanceService {
    pub fn restore_root_dir(app: &tauri::AppHandle) -> Result<PathBuf, AppErrorDto> {
        let dir = app
            .path()
            .app_data_dir()
            .map_err(|_| AppErrorDto::internal("Не удалось определить app data directory"))?
            .join("restore");

        Ok(dir)
    }

    pub fn state_path(app: &tauri::AppHandle) -> Result<PathBuf, AppErrorDto> {
        Ok(Self::restore_root_dir(app)?.join("restore-operation-state.json"))
    }

    pub fn lock_path(app: &tauri::AppHandle) -> Result<PathBuf, AppErrorDto> {
        Ok(Self::restore_root_dir(app)?.join("restore.lock"))
    }

    pub fn staging_dir(app: &tauri::AppHandle, operation_id: &str) -> Result<PathBuf, AppErrorDto> {
        Ok(Self::restore_root_dir(app)?
            .join("staging")
            .join(operation_id))
    }

    pub fn rollback_dir(
        app: &tauri::AppHandle,
        operation_id: &str,
    ) -> Result<PathBuf, AppErrorDto> {
        Ok(Self::restore_root_dir(app)?
            .join("rollback")
            .join(operation_id))
    }

    pub fn write_state(
        app: &tauri::AppHandle,
        state: &RestoreOperationState,
    ) -> Result<(), AppErrorDto> {
        let path = Self::state_path(app)?;

        if let Some(parent) = path.parent() {
            std::fs::create_dir_all(parent)
                .map_err(|err| AppErrorDto::filesystem(err.to_string()))?;
        }

        let json = serde_json::to_string_pretty(state)
            .map_err(|_| AppErrorDto::internal("Не удалось сериализовать restore state"))?;

        std::fs::write(path, json).map_err(|err| AppErrorDto::filesystem(err.to_string()))?;
        Ok(())
    }

    pub fn read_state(
        app: &tauri::AppHandle,
    ) -> Result<Option<RestoreOperationState>, AppErrorDto> {
        let path = Self::state_path(app)?;

        if !path.exists() {
            return Ok(None);
        }

        let raw = std::fs::read_to_string(path)
            .map_err(|err| AppErrorDto::filesystem(err.to_string()))?;

        let state = serde_json::from_str::<RestoreOperationState>(&raw)
            .map_err(|_| AppErrorDto::validation("Restore state повреждён"))?;

        Ok(Some(state))
    }

    pub fn create_lock(app: &tauri::AppHandle, operation_id: &str) -> Result<(), AppErrorDto> {
        let path = Self::lock_path(app)?;

        if let Some(parent) = path.parent() {
            std::fs::create_dir_all(parent)
                .map_err(|err| AppErrorDto::filesystem(err.to_string()))?;
        }

        std::fs::write(path, operation_id)
            .map_err(|err| AppErrorDto::filesystem(err.to_string()))?;
        Ok(())
    }

    pub fn remove_lock_best_effort(app: &tauri::AppHandle) {
        if let Ok(path) = Self::lock_path(app) {
            let _ = std::fs::remove_file(path);
        }
    }

    pub fn update_phase(
        app: &tauri::AppHandle,
        phase: RestoreOperationPhase,
        error_code: Option<String>,
    ) -> Result<(), AppErrorDto> {
        let Some(mut state) = Self::read_state(app)? else {
            return Err(AppErrorDto::validation("Restore state не найден"));
        };

        state.phase = phase;
        state.updated_at = Utc::now().to_rfc3339();
        state.last_error_code = error_code;

        Self::write_state(app, &state)
    }

    pub fn is_recovery_required(app: &tauri::AppHandle) -> Result<bool, AppErrorDto> {
        let lock_exists = Self::lock_path(app)?.exists();
        let state = Self::read_state(app)?;

        Ok(lock_exists || state.is_some())
    }

    pub fn cleanup_completed_restore(app: &tauri::AppHandle) -> Result<(), AppErrorDto> {
        let Some(state) = Self::read_state(app)? else {
            Self::remove_lock_best_effort(app);
            return Ok(());
        };

        if state.phase != RestoreOperationPhase::CompletedRequiresRestart
            && state.phase != RestoreOperationPhase::RollbackCompleted
        {
            return Err(AppErrorDto::validation(
                "Restore cleanup запрещён: операция не находится в завершённом состоянии",
            ));
        }

        let staging = Self::staging_dir(app, &state.operation_id)?;
        let _ = std::fs::remove_dir_all(staging);

        Self::remove_lock_best_effort(app);

        let state_path = Self::state_path(app)?;
        let _ = std::fs::remove_file(state_path);

        Ok(())
    }
}
