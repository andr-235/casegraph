use crate::domain::user_management::{CreateUserPayload, UpdateUserPayload};
use crate::errors::app_error::AppErrorDto;

const MIN_USERNAME_LEN: usize = 3;
const MAX_USERNAME_LEN: usize = 64;
const MIN_DISPLAY_NAME_LEN: usize = 2;
const MAX_DISPLAY_NAME_LEN: usize = 120;
const MIN_PASSWORD_LEN: usize = 8;
const MAX_PASSWORD_LEN: usize = 128;

#[derive(Debug)]
pub struct NormalizedCreateUserInput {
    pub username: String,
    pub display_name: Option<String>,
    pub role_code: String,
    pub password: String,
    pub must_change_password: bool,
}

pub fn normalize_create_user_payload(
    payload: CreateUserPayload,
) -> Result<NormalizedCreateUserInput, AppErrorDto> {
    let username = payload.username.trim().to_lowercase();
    let display_name = payload
        .display_name
        .map(|value| value.trim().to_string())
        .filter(|value| !value.is_empty());

    let role_code = payload.role_code.trim().to_string();
    let password = payload.password;

    validate_username(&username)?;
    validate_display_name(display_name.as_deref())?;
    validate_role_code(&role_code)?;
    validate_password(&password)?;

    Ok(NormalizedCreateUserInput {
        username,
        display_name,
        role_code,
        password,
        must_change_password: payload.must_change_password.unwrap_or(true),
    })
}

#[derive(Debug)]
pub struct NormalizedUpdateUserInput {
    pub user_id: String,
    pub display_name: Option<String>,
    pub role_code: String,
    pub must_change_password: bool,
}

pub fn normalize_update_user_payload(
    payload: UpdateUserPayload,
) -> Result<NormalizedUpdateUserInput, AppErrorDto> {
    let user_id = payload.user_id.trim().to_string();

    if user_id.is_empty() {
        return Err(AppErrorDto::validation("Не указан пользователь"));
    }

    let display_name = payload
        .display_name
        .map(|value| value.trim().to_string())
        .filter(|value| !value.is_empty());

    let role_code = payload.role_code.trim().to_string();

    validate_display_name(display_name.as_deref())?;
    validate_role_code(&role_code)?;

    Ok(NormalizedUpdateUserInput {
        user_id,
        display_name,
        role_code,
        must_change_password: payload.must_change_password,
    })
}

fn validate_username(username: &str) -> Result<(), AppErrorDto> {
    if username.len() < MIN_USERNAME_LEN || username.len() > MAX_USERNAME_LEN {
        return Err(AppErrorDto::validation(
            "Логин должен содержать от 3 до 64 символов",
        ));
    }

    let is_valid = username.chars().all(|ch| {
        ch.is_ascii_lowercase() || ch.is_ascii_digit() || ch == '_' || ch == '-' || ch == '.'
    });

    if !is_valid {
        return Err(AppErrorDto::validation(
            "Логин может содержать латинские буквы, цифры, точку, дефис и подчёркивание",
        ));
    }

    Ok(())
}

fn validate_display_name(display_name: Option<&str>) -> Result<(), AppErrorDto> {
    if let Some(display_name) = display_name {
        if display_name.len() < MIN_DISPLAY_NAME_LEN || display_name.len() > MAX_DISPLAY_NAME_LEN {
            return Err(AppErrorDto::validation(
                "Имя пользователя должно содержать от 2 до 120 символов",
            ));
        }
    }

    Ok(())
}

fn validate_role_code(role_code: &str) -> Result<(), AppErrorDto> {
    let is_valid = matches!(role_code, "administrator" | "analyst" | "viewer");

    if !is_valid {
        return Err(AppErrorDto::validation("Некорректная роль пользователя"));
    }

    Ok(())
}

fn validate_password(password: &str) -> Result<(), AppErrorDto> {
    if password.len() < MIN_PASSWORD_LEN || password.len() > MAX_PASSWORD_LEN {
        return Err(AppErrorDto::validation(
            "Пароль должен содержать от 8 до 128 символов",
        ));
    }

    Ok(())
}
