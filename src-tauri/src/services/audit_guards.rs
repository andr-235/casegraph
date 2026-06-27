use tauri::AppHandle;

use crate::errors::app_error::AppErrorDto;
use crate::security::session::CurrentUserDto;
use crate::services::audit_access_denied::access_denied_error;

pub const ROLE_ADMINISTRATOR: &str = "administrator";
pub const ROLE_ANALYST: &str = "analyst";

pub fn current_user_role(current_user: &CurrentUserDto) -> &str {
    current_user.role.as_str()
}

pub fn is_administrator(current_user: &CurrentUserDto) -> bool {
    current_user_role(current_user) == ROLE_ADMINISTRATOR
}

pub fn is_analyst(current_user: &CurrentUserDto) -> bool {
    current_user_role(current_user) == ROLE_ANALYST
}

pub fn is_audit_reader(current_user: &CurrentUserDto) -> bool {
    is_administrator(current_user) || is_analyst(current_user)
}

pub fn audit_user_filter_for_reader(
    current_user: &CurrentUserDto,
    requested_user_id: Option<String>,
) -> Option<String> {
    if is_administrator(current_user) {
        requested_user_id
    } else {
        Some(current_user.user_id.clone())
    }
}

pub fn require_audit_reader(
    app: &AppHandle,
    current_user: &CurrentUserDto,
    command_name: &str,
    message: &str,
) -> Result<(), AppErrorDto> {
    if is_audit_reader(current_user) {
        return Ok(());
    }

    Err(access_denied_error(
        app,
        current_user,
        command_name,
        "audit_log",
        None,
        "administrator|analyst",
        message,
    ))
}

pub fn require_audit_admin(
    app: &AppHandle,
    current_user: &CurrentUserDto,
    command_name: &str,
    message: &str,
) -> Result<(), AppErrorDto> {
    if is_administrator(current_user) {
        return Ok(());
    }

    Err(access_denied_error(
        app,
        current_user,
        command_name,
        "audit_log",
        None,
        "administrator",
        message,
    ))
}

pub fn require_no_user_filter_for_analyst(
    app: &AppHandle,
    current_user: &CurrentUserDto,
    command_name: &str,
    requested_user_id: &Option<String>,
) -> Result<(), AppErrorDto> {
    if is_administrator(current_user) || requested_user_id.is_none() {
        return Ok(());
    }

    Err(access_denied_error(
        app,
        current_user,
        command_name,
        "audit_log",
        None,
        "administrator",
        "Фильтр журнала по пользователю доступен только администратору.",
    ))
}

pub fn require_own_audit_entry_or_admin(
    app: &AppHandle,
    current_user: &CurrentUserDto,
    command_name: &str,
    audit_log_id: &str,
    audit_entry_user_id: Option<&str>,
) -> Result<(), AppErrorDto> {
    if is_administrator(current_user) {
        return Ok(());
    }

    if audit_entry_user_id == Some(current_user.user_id.as_str()) {
        return Ok(());
    }

    Err(access_denied_error(
        app,
        current_user,
        command_name,
        "audit_log",
        Some(audit_log_id.to_string()),
        "administrator",
        "Аналитик может просматривать только собственные записи журнала.",
    ))
}
