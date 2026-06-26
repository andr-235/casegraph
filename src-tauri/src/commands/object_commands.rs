use tauri::{AppHandle, State};

use crate::domain::objects::{
    CreateObjectPayload, CreateObjectResponse, GetObjectByIdPayload, GetObjectByIdResponse,
    GetObjectsPayload, GetObjectsResponse, LinkObjectToMaterialsPayload,
    LinkObjectToMaterialsResponse, UpdateObjectPayload, UpdateObjectResponse,
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

#[tauri::command]
pub fn get_object_by_id(
    app: AppHandle,
    session: State<SessionState>,
    payload: GetObjectByIdPayload,
) -> CommandResult<GetObjectByIdResponse> {
    match ObjectService::get_object_by_id(&app, &session, payload) {
        Ok(response) => CommandResult::ok(response),
        Err(error) => CommandResult::err(error),
    }
}

#[tauri::command]
pub fn link_object_to_materials(
    app: AppHandle,
    session: State<SessionState>,
    payload: LinkObjectToMaterialsPayload,
) -> CommandResult<LinkObjectToMaterialsResponse> {
    match ObjectService::link_object_to_materials(&app, &session, payload) {
        Ok(response) => CommandResult::ok(response),
        Err(error) => CommandResult::err(error),
    }
}

#[tauri::command]
pub fn update_object(
    app: AppHandle,
    session: State<SessionState>,
    payload: UpdateObjectPayload,
) -> CommandResult<UpdateObjectResponse> {
    match ObjectService::update_object(&app, &session, payload) {
        Ok(response) => CommandResult::ok(response),
        Err(error) => CommandResult::err(error),
    }
}
