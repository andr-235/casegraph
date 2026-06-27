use tauri::{AppHandle, State};

use crate::errors::app_error::CommandResult;
use crate::models::settings::{AppSettingsDto, UpdateSettingsPayload};
use crate::security::session::SessionState;
use crate::services::settings_service::SettingsService;

#[tauri::command]
pub fn get_settings(app: AppHandle, session: State<SessionState>) -> CommandResult<AppSettingsDto> {
    match SettingsService::get_settings(&app, &session) {
        Ok(settings) => CommandResult::ok(settings),
        Err(error) => CommandResult::err(error),
    }
}

#[tauri::command]
pub fn update_settings(
    app: AppHandle,
    session: State<SessionState>,
    payload: UpdateSettingsPayload,
) -> CommandResult<AppSettingsDto> {
    match SettingsService::update_settings(&app, &session, payload) {
        Ok(settings) => CommandResult::ok(settings),
        Err(error) => CommandResult::err(error),
    }
}
