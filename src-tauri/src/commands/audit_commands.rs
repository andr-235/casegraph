use tauri::{AppHandle, State};

use crate::audit::audit_service::AuditService;
use crate::domain::audit::{
    ExportAuditLogPayload, ExportAuditLogResponse, GetAuditActionsResponse, GetAuditLogByIdPayload,
    GetAuditLogByIdResponse, GetAuditLogsPayload, GetAuditLogsResponse, GetAuditUsersResponse,
};
use crate::errors::app_error::CommandResult;
use crate::security::session::SessionState;

#[tauri::command]
pub fn get_audit_logs(
    app: AppHandle,
    session: State<'_, SessionState>,
    payload: GetAuditLogsPayload,
) -> CommandResult<GetAuditLogsResponse> {
    match AuditService::get_audit_logs(&app, &session, payload) {
        Ok(response) => CommandResult::ok(response),
        Err(error) => CommandResult::err(error),
    }
}

#[tauri::command]
pub fn get_audit_log_by_id(
    app: AppHandle,
    session: State<'_, SessionState>,
    payload: GetAuditLogByIdPayload,
) -> CommandResult<GetAuditLogByIdResponse> {
    match AuditService::get_audit_log_by_id(&app, &session, payload) {
        Ok(response) => CommandResult::ok(response),
        Err(error) => CommandResult::err(error),
    }
}

#[tauri::command]
pub fn get_audit_actions(
    app: AppHandle,
    session: State<'_, SessionState>,
) -> CommandResult<GetAuditActionsResponse> {
    match AuditService::get_audit_actions(&app, &session) {
        Ok(response) => CommandResult::ok(response),
        Err(error) => CommandResult::err(error),
    }
}

#[tauri::command]
pub fn get_audit_users(
    app: AppHandle,
    session: State<'_, SessionState>,
) -> CommandResult<GetAuditUsersResponse> {
    match AuditService::get_audit_users(&app, &session) {
        Ok(response) => CommandResult::ok(response),
        Err(error) => CommandResult::err(error),
    }
}

#[tauri::command]
pub fn export_audit_log(
    app: AppHandle,
    session: State<'_, SessionState>,
    payload: ExportAuditLogPayload,
) -> CommandResult<ExportAuditLogResponse> {
    match AuditService::export_audit_log(&app, &session, payload) {
        Ok(response) => CommandResult::ok(response),
        Err(error) => CommandResult::err(error),
    }
}
