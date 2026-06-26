use tauri::{AppHandle, State};

use crate::domain::objects::{
    CreateObjectPayload, CreateObjectResponse, GetObjectsPayload, GetObjectsResponse,
};
use crate::errors::app_error::CommandResult;
use crate::security::session::SessionState;
use crate::services::object_service::ObjectService;

#[tauri::command]
pub fn create_object(
    app: AppHandle,
    session: State<SessionState>,
    payload: CreateObjectPayload,
) -> CommandResult<CreateObjectResponse> {
    match ObjectService::create_object(&app, &session, payload) {
        Ok(response) => CommandResult::ok(response),
        Err(error) => CommandResult::err(error),
    }
}

#[tauri::command]
pub fn get_objects(
    app: AppHandle,
    session: State<SessionState>,
    payload: GetObjectsPayload,
) -> CommandResult<GetObjectsResponse> {
    match ObjectService::get_objects(&app, &session, payload) {
        Ok(response) => CommandResult::ok(response),
        Err(error) => CommandResult::err(error),
    }
}
