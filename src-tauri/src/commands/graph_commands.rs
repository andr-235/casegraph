use tauri::{AppHandle, State};

use crate::domain::graph::{GetGraphDataPayload, GetGraphDataResponse};
use crate::errors::app_error::CommandResult;
use crate::security::session::SessionState;
use crate::services::graph_service::GraphService;

#[tauri::command]
pub fn get_graph_data(
    app: AppHandle,
    session: State<SessionState>,
    payload: GetGraphDataPayload,
) -> CommandResult<GetGraphDataResponse> {
    match GraphService::get_graph_data(&app, &session, payload) {
        Ok(response) => CommandResult::ok(response),
        Err(error) => CommandResult::err(error),
    }
}
