use rusqlite::params;
use serde::{Deserialize, Serialize};
use tauri::AppHandle;
use uuid::Uuid;

use crate::db::connection::open_connection;
use crate::db::migrations::has_administrator;
use crate::errors::app_error::{AppErrorDto, CommandResult};
use crate::security::password::hash_password;

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CreateFirstAdminPayload {
    pub username: String,
    pub display_name: String,
    pub password: String,
}

#[derive(Debug, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct CreateFirstAdminResponse {
    pub user_id: String,
    pub username: String,
    pub role: String,
}

#[tauri::command]
pub fn create_first_admin(
    app: AppHandle,
    payload: CreateFirstAdminPayload,
) -> CommandResult<CreateFirstAdminResponse> {
    match create_first_admin_inner(app, payload) {
        Ok(response) => CommandResult::ok(response),
        Err(error) => CommandResult::err(error),
    }
}

fn create_first_admin_inner(
    app: AppHandle,
    payload: CreateFirstAdminPayload,
) -> Result<CreateFirstAdminResponse, AppErrorDto> {
    let username = payload.username.trim().to_lowercase();
    let display_name = payload.display_name.trim().to_string();

    if username.len() < 3 {
        return Err(AppErrorDto::new(
            "ERR_VALIDATION",
            "Логин должен содержать минимум 3 символа.",
            None,
        ));
    }

    if display_name.len() < 2 {
        return Err(AppErrorDto::new(
            "ERR_VALIDATION",
            "Имя пользователя должно содержать минимум 2 символа.",
            None,
        ));
    }

    if payload.password.len() < 8 {
        return Err(AppErrorDto::new(
            "ERR_VALIDATION",
            "Пароль должен содержать минимум 8 символов.",
            None,
        ));
    }

    let conn = open_connection(&app)?;

    if has_administrator(&conn)? {
        return Err(AppErrorDto::new(
            "ERR_ADMIN_ALREADY_EXISTS",
            "Первый администратор уже создан.",
            None,
        ));
    }

    let role_id: String = conn
        .query_row(
            "SELECT id FROM roles WHERE code = 'administrator'",
            [],
            |row| row.get(0),
        )
        .map_err(|err| AppErrorDto::database(err.to_string()))?;

    let user_id = Uuid::new_v4().to_string();
    let password_hash = hash_password(&payload.password)?;

    conn.execute(
        r#"
        INSERT INTO users (
            id,
            username,
            display_name,
            password_hash,
            role_id,
            is_active,
            must_change_password
        )
        VALUES (?1, ?2, ?3, ?4, ?5, 1, 0)
        "#,
        params![user_id, username, display_name, password_hash, role_id],
    )
    .map_err(|err| AppErrorDto::database(err.to_string()))?;

    Ok(CreateFirstAdminResponse {
        user_id,
        username,
        role: "administrator".to_string(),
    })
}
