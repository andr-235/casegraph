use crate::backup::{
    ResolveRestoreRecoveryPayload, ResolveRestoreRecoveryResponse, RestoreRecoveryStatusDto,
    RestoreStartupRecoveryService,
};
use crate::errors::app_error::CommandResult;

#[tauri::command]
pub fn get_restore_recovery_status(
    app: tauri::AppHandle,
) -> CommandResult<RestoreRecoveryStatusDto> {
    CommandResult::from_result(RestoreStartupRecoveryService::get_status(&app))
}

#[tauri::command]
pub fn resolve_restore_recovery(
    app: tauri::AppHandle,
    payload: ResolveRestoreRecoveryPayload,
) -> CommandResult<ResolveRestoreRecoveryResponse> {
    CommandResult::from_result(RestoreStartupRecoveryService::resolve(&app, payload))
}
