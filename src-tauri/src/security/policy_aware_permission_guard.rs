use rusqlite::Connection;
use tauri::AppHandle;

use crate::audit::audit_metadata;
use crate::audit::audit_service::{AuditService, AuditWriteInput};
use crate::errors::app_error::AppErrorDto;
use crate::security::permission_decision::{PermissionDecision, PermissionDenyReason};
use crate::security::policy_aware_permission_service::PolicyAwarePermissionService;
use crate::security::protected_operation::ProtectedOperation;
use crate::security::session::CurrentUserDto;

pub struct PolicyAwarePermissionGuard;

impl PolicyAwarePermissionGuard {
    pub fn require(
        app: &AppHandle,
        conn: &Connection,
        user: &CurrentUserDto,
        operation: ProtectedOperation,
    ) -> Result<(), AppErrorDto> {
        let decision = PolicyAwarePermissionService::decide(conn, user, operation);

        match decision {
            PermissionDecision::Allow => Ok(()),
            PermissionDecision::Deny { reason, message } => {
                Self::write_access_denied_best_effort(app, user, operation, reason);
                Err(AppErrorDto::access_denied(message))
            }
        }
    }

    fn write_access_denied_best_effort(
        app: &AppHandle,
        user: &CurrentUserDto,
        operation: ProtectedOperation,
        reason: PermissionDenyReason,
    ) {
        let details = match reason {
            PermissionDenyReason::RoleDenied => {
                audit_metadata::access_denied_details(operation.action_name(), "role_denied", None)
            }
            PermissionDenyReason::PolicyDenied { policy_key } => {
                audit_metadata::access_denied_details(
                    operation.action_name(),
                    "policy_denied",
                    Some(policy_key),
                )
            }
        };

        match details {
            Ok(details) => {
                let mut input = AuditWriteInput::failure(
                    user,
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
