use std::fs;
use std::path::PathBuf;

use rusqlite::Connection;
use tauri::{AppHandle, Manager};

use crate::errors::app_error::AppErrorDto;

pub fn get_database_path(app: &AppHandle) -> Result<PathBuf, AppErrorDto> {
    let app_data_dir = app
        .path()
        .app_data_dir()
        .map_err(|err| AppErrorDto::filesystem(err.to_string()))?;

    fs::create_dir_all(&app_data_dir).map_err(|err| AppErrorDto::filesystem(err.to_string()))?;

    Ok(app_data_dir.join("casegraph.sqlite"))
}

pub fn open_connection(app: &AppHandle) -> Result<Connection, AppErrorDto> {
    let db_path = get_database_path(app)?;

    Connection::open(db_path).map_err(|err| AppErrorDto::database(err.to_string()))
}
