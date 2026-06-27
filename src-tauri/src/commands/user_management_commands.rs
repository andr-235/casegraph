use tauri::{AppHandle, State};

use crate::domain::user_management::{
    CreateUserPayload, CreateUserResponse, GetRolesResponse, GetUserByIdPayload,
    GetUserByIdResponse, GetUsersPayload, GetUsersResponse, UpdateUserPayload, UpdateUserResponse,
};
use crate::errors::app_error::CommandResult;
use crate::security::session::SessionState;
use crate::services::user_management_service::UserManagementService;

#[tauri::command]
pub fn get_users(
    app: AppHandle,
    session: State<SessionState>,
    payload: GetUsersPayload,
) -> CommandResult<GetUsersResponse> {
    match UserManagementService::get_users(&app, &session, payload) {
        Ok(response) => CommandResult::ok(response),
        Err(error) => CommandResult::err(error),
    }
}

#[tauri::command]
pub fn get_user_by_id(
    app: AppHandle,
    session: State<SessionState>,
    payload: GetUserByIdPayload,
) -> CommandResult<GetUserByIdResponse> {
    match UserManagementService::get_user_by_id(&app, &session, payload) {
        Ok(response) => CommandResult::ok(response),
        Err(error) => CommandResult::err(error),
    }
}

#[tauri::command]
pub fn update_user(
    app: AppHandle,
    session: State<SessionState>,
    payload: UpdateUserPayload,
) -> CommandResult<UpdateUserResponse> {
    match UserManagementService::update_user(&app, &session, payload) {
        Ok(response) => CommandResult::ok(response),
        Err(error) => CommandResult::err(error),
    }
}

#[tauri::command]
pub fn create_user(
    app: AppHandle,
    session: State<SessionState>,
    payload: CreateUserPayload,
) -> CommandResult<CreateUserResponse> {
    match UserManagementService::create_user(&app, &session, payload) {
        Ok(response) => CommandResult::ok(response),
        Err(error) => CommandResult::err(error),
    }
}

#[tauri::command]
pub fn get_roles(app: AppHandle, session: State<SessionState>) -> CommandResult<GetRolesResponse> {
    match UserManagementService::get_roles(&app, &session) {
        Ok(response) => CommandResult::ok(response),
        Err(error) => CommandResult::err(error),
    }
}
