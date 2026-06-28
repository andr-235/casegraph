use rusqlite::Connection;
use tauri::{AppHandle, Manager};

use crate::audit::audit_metadata;
use crate::audit::audit_service::{AuditService, AuditWriteInput};
use crate::db::connection::open_connection;
use crate::errors::app_error::AppErrorDto;
use crate::repositories::user_access_repository::UserAccessRepository;
use crate::security::policy_aware_permission_guard::PolicyAwarePermissionGuard;
use crate::security::protected_operation::ProtectedOperation;
use crate::security::session::{CurrentUserDto, SessionState};

pub struct ProtectedServiceContext {
    pub conn: Connection,
    pub current_user: CurrentUserDto,
}

impl ProtectedServiceContext {
    pub fn require_operation(
        app: &AppHandle,
        operation: ProtectedOperation,
    ) -> Result<Self, AppErrorDto> {
        Self::require_restore_not_blocking(app, operation)?;

        let session = app.state::<SessionState>();
        let session_user = session.require_current_user()?;

        let conn = open_connection(app)?;
        let access_flags =
            UserAccessRepository::get_user_access_flags(&conn, &session_user.user_id)?;

        let current_user = CurrentUserDto {
            user_id: access_flags.id,
            username: access_flags.username,
            display_name: session_user.display_name,
            role: access_flags.role_code,
            is_active: access_flags.is_active,
            must_change_password: access_flags.must_change_password,
        };

        Self::require_active_user(app, &current_user, operation)?;
        Self::require_password_change_resolved(app, &current_user, operation)?;

        PolicyAwarePermissionGuard::require(app, &conn, &current_user, operation)?;

        Ok(Self { conn, current_user })
    }

    pub fn require_authenticated(app: &AppHandle) -> Result<Self, AppErrorDto> {
        let session = app.state::<SessionState>();
        let session_user = session.require_current_user()?;

        let conn = open_connection(app)?;
        let access_flags =
            UserAccessRepository::get_user_access_flags(&conn, &session_user.user_id)?;

        let current_user = CurrentUserDto {
            user_id: access_flags.id,
            username: access_flags.username,
            display_name: session_user.display_name,
            role: access_flags.role_code,
            is_active: access_flags.is_active,
            must_change_password: access_flags.must_change_password,
        };

        Self::require_active_user(app, &current_user, ProtectedOperation::CaseRead)?;

        Ok(Self { conn, current_user })
    }

    fn require_active_user(
        app: &AppHandle,
        current_user: &CurrentUserDto,
        operation: ProtectedOperation,
    ) -> Result<(), AppErrorDto> {
        if current_user.is_active {
            return Ok(());
        }

        Self::write_access_denied_best_effort(app, current_user, operation, "inactive_user", None);

        Err(AppErrorDto::access_denied("Учётная запись заблокирована."))
    }

    fn require_password_change_resolved(
        app: &AppHandle,
        current_user: &CurrentUserDto,
        operation: ProtectedOperation,
    ) -> Result<(), AppErrorDto> {
        if !current_user.must_change_password {
            return Ok(());
        }

        Self::write_access_denied_best_effort(
            app,
            current_user,
            operation,
            "password_change_required",
            None,
        );

        Err(AppErrorDto::password_change_required())
    }

    fn require_restore_not_blocking(
        app: &AppHandle,
        operation: ProtectedOperation,
    ) -> Result<(), AppErrorDto> {
        if operation.is_restore_recovery_allowed() {
            return Ok(());
        }

        if crate::backup::RestoreMaintenanceService::is_recovery_required(app)? {
            return Err(AppErrorDto::restore_in_progress(
                "Приложение находится в режиме восстановления после restore",
            ));
        }

        Ok(())
    }

    fn write_access_denied_best_effort(
        app: &AppHandle,
        current_user: &CurrentUserDto,
        operation: ProtectedOperation,
        reason: &'static str,
        policy_key: Option<&'static str>,
    ) {
        let details =
            audit_metadata::access_denied_details(operation.action_name(), reason, policy_key);

        match details {
            Ok(details) => {
                let mut input = AuditWriteInput::failure(
                    current_user,
                    crate::domain::audit_action::audit::ACCESS_DENIED,
                )
                .with_entity_type("operation")
                .with_details(details);

                input.result = "denied".to_string();
                input.severity = "warning".to_string();

                AuditService::write_best_effort(app, input);
            }
            Err(e) => {
                eprintln!(
                    "[audit] failed to build access_denied details: {}",
                    e.message
                );
            }
        }
    }
}
