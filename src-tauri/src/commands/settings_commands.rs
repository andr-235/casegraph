use tauri::{AppHandle, State};

use crate::errors::app_error::CommandResult;
use crate::models::settings::{
    AppSettingsDto, ChooseSettingsDirectoryPayload, ChooseSettingsDirectoryResponse,
    UpdateSettingsPayload,
};
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

#[tauri::command]
pub fn choose_settings_directory(
    app: AppHandle,
    session: State<SessionState>,
    payload: ChooseSettingsDirectoryPayload,
) -> CommandResult<ChooseSettingsDirectoryResponse> {
    match SettingsService::choose_settings_directory(&app, &session, payload) {
        Ok(response) => CommandResult::ok(response),
        Err(error) => CommandResult::err(error),
    }
}

#[tauri::command]
pub fn reset_settings_to_defaults(
    app: AppHandle,
    session: State<SessionState>,
) -> CommandResult<AppSettingsDto> {
    match SettingsService::reset_settings_to_defaults(&app, &session) {
        Ok(settings) => CommandResult::ok(settings),
        Err(error) => CommandResult::err(error),
    }
}
