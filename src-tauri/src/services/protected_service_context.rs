use rusqlite::Connection;
use tauri::AppHandle;

use crate::db::connection::open_connection;
use crate::errors::app_error::AppErrorDto;
use crate::security::session::{CurrentUserDto, SessionState};
use crate::services::protected_service_guard::ProtectedServiceGuard;

pub struct ProtectedServiceContext {
    pub current_user: CurrentUserDto,
    pub conn: Connection,
}

pub fn require_protected_user(
    app: &AppHandle,
    session: &SessionState,
) -> Result<ProtectedServiceContext, AppErrorDto> {
    let current_user = session.require_current_user()?;
    let conn = open_connection(app)?;

    ProtectedServiceGuard::require_password_change_resolved(&conn, &current_user)?;

    Ok(ProtectedServiceContext { current_user, conn })
}

pub fn require_protected_administrator(
    app: &AppHandle,
    session: &SessionState,
) -> Result<ProtectedServiceContext, AppErrorDto> {
    let context = require_protected_user(app, session)?;

    if context.current_user.role != "administrator" {
        return Err(AppErrorDto::access_denied(
            "Действие доступно только администратору",
        ));
    }

    Ok(context)
}

pub fn require_protected_analyst_or_admin(
    app: &AppHandle,
    session: &SessionState,
) -> Result<ProtectedServiceContext, AppErrorDto> {
    let context = require_protected_user(app, session)?;

    let is_allowed = matches!(
        context.current_user.role.as_str(),
        "administrator" | "analyst"
    );

    if !is_allowed {
        return Err(AppErrorDto::access_denied(
            "Действие доступно только администратору или аналитику",
        ));
    }

    Ok(context)
}
