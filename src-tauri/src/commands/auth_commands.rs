use tauri::{AppHandle, State};

use crate::errors::app_error::CommandResult;
use crate::security::session::{CurrentUserDto, SessionState};
use crate::services::auth_service::{
    AuthService, CreateFirstAdminPayload, CreateFirstAdminResponse, LoginPayload, LoginResponse,
};

#[tauri::command]
pub fn create_first_admin(
    app: AppHandle,
    payload: CreateFirstAdminPayload,
) -> CommandResult<CreateFirstAdminResponse> {
    match AuthService::create_first_admin(&app, payload) {
        Ok(response) => CommandResult::ok(response),
        Err(error) => CommandResult::err(error),
    }
}

#[tauri::command]
pub fn login(
    app: AppHandle,
    session: State<'_, SessionState>,
    payload: LoginPayload,
) -> CommandResult<LoginResponse> {
    match AuthService::login(&app, &session, payload) {
        Ok(response) => CommandResult::ok(response),
        Err(error) => CommandResult::err(error),
    }
}

#[tauri::command]
pub fn get_current_user(session: State<'_, SessionState>) -> CommandResult<Option<CurrentUserDto>> {
    CommandResult::ok(AuthService::get_current_user(&session))
}

#[tauri::command]
pub fn logout(session: State<'_, SessionState>) -> CommandResult<bool> {
    CommandResult::ok(AuthService::logout(&session))
}
