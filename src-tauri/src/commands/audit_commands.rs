use tauri::{AppHandle, State};

use crate::db::connection::open_connection;
use crate::domain::audit::{GetAuditLogsPayload, GetAuditLogsResponse};
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
