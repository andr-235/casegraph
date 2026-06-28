use tauri::AppHandle;

use crate::backup::{BackupHistoryItemDto, BackupService};
use crate::errors::app_error::CommandResult;

#[tauri::command]
pub fn get_backup_history(app: AppHandle) -> CommandResult<Vec<BackupHistoryItemDto>> {
    match BackupService::get_backup_history(&app) {
        Ok(data) => CommandResult::ok(data),
        Err(error) => CommandResult::err(error),
    }
}
