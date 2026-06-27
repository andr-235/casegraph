use tauri::{AppHandle, State};

use crate::errors::app_error::CommandResult;
use crate::models::settings::AppSettingsDto;
use crate::security::session::SessionState;
use crate::services::settings_service::SettingsService;

#[tauri::command]
pub fn get_settings(app: AppHandle, session: State<SessionState>) -> CommandResult<AppSettingsDto> {
    match SettingsService::get_settings(&app, &session) {
        Ok(settings) => CommandResult::ok(settings),
        Err(error) => CommandResult::err(error),
    }
}
