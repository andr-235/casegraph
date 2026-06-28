use tauri::AppHandle;

use crate::backup::{
    BackupHistoryItemDto, BackupService, CreateBackupPayload, CreateBackupResponse,
    SelectBackupFileResponse, SelectBackupOutputFolderResponse, VerifyBackupPayload,
    VerifyBackupResponse,
};
use crate::errors::app_error::CommandResult;

#[tauri::command]
pub fn get_backup_history(app: AppHandle) -> CommandResult<Vec<BackupHistoryItemDto>> {
    match BackupService::get_backup_history(&app) {
        Ok(data) => CommandResult::ok(data),
        Err(error) => CommandResult::err(error),
    }
}

#[tauri::command]
pub fn choose_backup_folder(app: AppHandle) -> CommandResult<SelectBackupOutputFolderResponse> {
    match BackupService::choose_backup_folder(&app) {
        Ok(data) => CommandResult::ok(data),
        Err(error) => CommandResult::err(error),
    }
}

#[tauri::command]
pub fn create_backup(
    app: AppHandle,
    payload: CreateBackupPayload,
) -> CommandResult<CreateBackupResponse> {
    match BackupService::create_backup(&app, payload) {
        Ok(data) => CommandResult::ok(data),
        Err(error) => CommandResult::err(error),
    }
}

#[tauri::command]
pub fn choose_backup_file(app: AppHandle) -> CommandResult<SelectBackupFileResponse> {
    match BackupService::choose_backup_file(&app) {
        Ok(data) => CommandResult::ok(data),
        Err(error) => CommandResult::err(error),
    }
}

#[tauri::command]
pub fn verify_backup(
    app: AppHandle,
    payload: VerifyBackupPayload,
) -> CommandResult<VerifyBackupResponse> {
    match BackupService::verify_backup(&app, payload) {
        Ok(data) => CommandResult::ok(data),
        Err(error) => CommandResult::err(error),
    }
}
