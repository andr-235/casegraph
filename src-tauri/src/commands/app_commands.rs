use serde::Serialize;
use tauri::AppHandle;

use crate::backup::{RestoreRecoveryStatusDto, RestoreStartupRecoveryService};
use crate::db::connection::open_connection;
use crate::db::migrations::{apply_migrations, has_administrator};
use crate::errors::app_error::{AppErrorDto, CommandResult};

#[derive(Debug, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct InitializeAppResponse {
    pub app_version: String,
    pub has_admin: bool,
    pub database_ready: bool,
    pub offline_mode: bool,
    pub restore_recovery_status: RestoreRecoveryStatusDto,
}

#[tauri::command(rename_all = "camelCase")]
pub fn initialize_app(app: AppHandle) -> CommandResult<InitializeAppResponse> {
    match initialize_app_inner(app) {
        Ok(response) => CommandResult::ok(response),
        Err(error) => CommandResult::err(error),
    }
}

fn initialize_app_inner(app: AppHandle) -> Result<InitializeAppResponse, AppErrorDto> {
    let conn = open_connection(&app)?;

    apply_migrations(&conn)?;

    let has_admin = has_administrator(&conn)?;

    let restore_recovery_status = RestoreStartupRecoveryService::get_status(&app)?;

    Ok(InitializeAppResponse {
        app_version: env!("CARGO_PKG_VERSION").to_string(),
        has_admin,
        database_ready: true,
        offline_mode: true,
        restore_recovery_status,
    })
}
