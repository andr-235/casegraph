use tauri::{AppHandle, State};

use crate::db::connection::open_connection;
use crate::domain::audit::{
    GetAuditActionsResponse, GetAuditLogByIdPayload, GetAuditLogByIdResponse, GetAuditLogsPayload,
    GetAuditLogsResponse, GetAuditUsersResponse,
};
use crate::errors::app_error::CommandResult;
use crate::security::session::SessionState;
use crate::services::audit_service::AuditService;

#[tauri::command]
pub fn get_audit_logs(
    app: AppHandle,
    session: State<SessionState>,
    payload: GetAuditLogsPayload,
) -> CommandResult<GetAuditLogsResponse> {
    let current_user = match session.get_current_user() {
        Some(user) => user,
        None => {
            return CommandResult::err(crate::errors::app_error::AppErrorDto::unauthorized(
                "Необходимо войти в систему.",
            ));
        }
    };

    let conn = match open_connection(&app) {
        Ok(conn) => conn,
        Err(error) => return CommandResult::err(error),
    };

    match AuditService::get_audit_logs(&conn, &current_user, payload) {
        Ok(response) => CommandResult::ok(response),
        Err(error) => CommandResult::err(error),
    }
}

#[tauri::command]
pub fn get_audit_log_by_id(
    app: AppHandle,
    session: State<SessionState>,
    payload: GetAuditLogByIdPayload,
) -> CommandResult<GetAuditLogByIdResponse> {
    let current_user = match session.get_current_user() {
        Some(user) => user,
        None => {
            return CommandResult::err(crate::errors::app_error::AppErrorDto::unauthorized(
                "Необходимо войти в систему.",
            ));
        }
    };

    let conn = match open_connection(&app) {
        Ok(conn) => conn,
        Err(error) => return CommandResult::err(error),
    };

    match AuditService::get_audit_log_by_id(&conn, &current_user, payload) {
        Ok(response) => CommandResult::ok(response),
        Err(error) => CommandResult::err(error),
    }
}

#[tauri::command]
pub fn get_audit_actions(
    app: AppHandle,
    session: State<SessionState>,
) -> CommandResult<GetAuditActionsResponse> {
    let current_user = match session.get_current_user() {
        Some(user) => user,
        None => {
            return CommandResult::err(crate::errors::app_error::AppErrorDto::unauthorized(
                "Необходимо войти в систему.",
            ));
        }
    };

    let conn = match open_connection(&app) {
        Ok(conn) => conn,
        Err(error) => return CommandResult::err(error),
    };

    match AuditService::get_audit_actions(&conn, &current_user) {
        Ok(response) => CommandResult::ok(response),
        Err(error) => CommandResult::err(error),
    }
}

#[tauri::command]
pub fn get_audit_users(
    app: AppHandle,
    session: State<SessionState>,
) -> CommandResult<GetAuditUsersResponse> {
    let current_user = match session.get_current_user() {
        Some(user) => user,
        None => {
            return CommandResult::err(crate::errors::app_error::AppErrorDto::unauthorized(
                "Необходимо войти в систему.",
            ));
        }
    };

    let conn = match open_connection(&app) {
        Ok(conn) => conn,
        Err(error) => return CommandResult::err(error),
    };

    match AuditService::get_audit_users(&conn, &current_user) {
        Ok(response) => CommandResult::ok(response),
        Err(error) => CommandResult::err(error),
    }
}
