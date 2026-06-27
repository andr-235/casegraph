use rusqlite::Connection;
use tauri::AppHandle;

use crate::db::connection::open_connection;
use crate::errors::app_error::AppErrorDto;
use crate::security::session::{CurrentUserDto, SessionState};
use crate::services::protected_access_audit::{
    write_protected_access_denied_best_effort, ProtectedAccessDeniedAudit,
};
use crate::services::protected_service_guard::ProtectedServiceGuard;

pub struct ProtectedServiceContext {
    pub current_user: CurrentUserDto,
    pub conn: Connection,
}

pub fn require_protected_user(
    app: &AppHandle,
    session: &SessionState,
) -> Result<ProtectedServiceContext, AppErrorDto> {
    require_protected_user_for(app, session, "PROTECTED_COMMAND")
}

pub fn require_protected_user_for(
    app: &AppHandle,
    session: &SessionState,
    action: &str,
) -> Result<ProtectedServiceContext, AppErrorDto> {
    let current_user = session.require_current_user()?;
    let conn = open_connection(app)?;

    if let Err(err) = ProtectedServiceGuard::require_password_change_resolved(&conn, &current_user)
    {
        let reason = match err.code.as_str() {
            "ERR_PASSWORD_CHANGE_REQUIRED" => "password_change_required",
            "ERR_ACCESS_DENIED" => "inactive_or_access_denied",
            _ => "protected_guard_denied",
        };

        write_protected_access_denied_best_effort(
            app,
            &current_user,
            ProtectedAccessDeniedAudit::new(action, reason),
        );

        return Err(err);
    }

    Ok(ProtectedServiceContext { current_user, conn })
}

pub fn require_protected_administrator(
    app: &AppHandle,
    session: &SessionState,
) -> Result<ProtectedServiceContext, AppErrorDto> {
    require_protected_administrator_for(app, session, "PROTECTED_ADMIN_COMMAND")
}

pub fn require_protected_administrator_for(
    app: &AppHandle,
    session: &SessionState,
    action: &str,
) -> Result<ProtectedServiceContext, AppErrorDto> {
    let context = require_protected_user_for(app, session, action)?;

    if context.current_user.role != "administrator" {
        write_protected_access_denied_best_effort(
            app,
            &context.current_user,
            ProtectedAccessDeniedAudit::new(action, "role_denied").required_role("administrator"),
        );

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
    require_protected_analyst_or_admin_for(app, session, "PROTECTED_WRITE_COMMAND")
}

pub fn require_protected_analyst_or_admin_for(
    app: &AppHandle,
    session: &SessionState,
    action: &str,
) -> Result<ProtectedServiceContext, AppErrorDto> {
    let context = require_protected_user_for(app, session, action)?;

    let is_allowed = matches!(
        context.current_user.role.as_str(),
        "administrator" | "analyst"
    );

    if !is_allowed {
        write_protected_access_denied_best_effort(
            app,
            &context.current_user,
            ProtectedAccessDeniedAudit::new(action, "role_denied")
                .required_role("administrator|analyst"),
        );

        return Err(AppErrorDto::access_denied(
            "Действие доступно только администратору или аналитику",
        ));
    }

    Ok(context)
}
