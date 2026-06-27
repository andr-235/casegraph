use tauri::AppHandle;

use crate::audit::audit_metadata;
use crate::audit::audit_service::{AuditService, AuditWriteInput};
use crate::errors::app_error::AppErrorDto;
use crate::security::session::CurrentUserDto;

#[derive(Debug, Clone)]
pub struct ProtectedAccessDeniedAudit<'a> {
    pub command_name: &'a str,
    pub reason: &'a str,
    pub required_role: Option<&'a str>,
    pub case_id: Option<&'a str>,
    pub entity_type: Option<&'a str>,
    pub entity_id: Option<&'a str>,
}

impl<'a> ProtectedAccessDeniedAudit<'a> {
    pub fn new(command_name: &'a str, reason: &'a str) -> Self {
        Self {
            command_name,
            reason,
            required_role: None,
            case_id: None,
            entity_type: None,
            entity_id: None,
        }
    }

    pub fn required_role(mut self, required_role: &'a str) -> Self {
        self.required_role = Some(required_role);
        self
    }

    pub fn case_id(mut self, case_id: Option<&'a str>) -> Self {
        self.case_id = case_id;
        self
    }

    pub fn entity(mut self, entity_type: &'a str, entity_id: Option<&'a str>) -> Self {
        self.entity_type = Some(entity_type);
        self.entity_id = entity_id;
        self
    }
}

pub fn write_protected_access_denied_best_effort(
    app: &AppHandle,
    current_user: &CurrentUserDto,
    audit: ProtectedAccessDeniedAudit<'_>,
) {
    let result = (|| {
        let required_role_str = audit.required_role.unwrap_or("");
        let entity_type_str = audit.entity_type.unwrap_or("generic");

        let description = format!(
            "Access denied for action '{}'. Reason: {}.",
            audit.command_name, audit.reason
        );

        let technical_details = audit_metadata::access_denied(
            &description,
            audit.command_name,
            Some(&current_user.role),
            Some(required_role_str),
        )?;

        let mut input = AuditWriteInput::failure(
            current_user,
            crate::domain::audit_action::audit::ACCESS_DENIED,
        )
        .with_entity_type(entity_type_str)
        .with_details(technical_details);

        input.result = "denied".to_string(); // Keep EXACT compatibility
        input.severity = "warning".to_string();

        if let Some(case_id) = audit.case_id {
            input = input.with_case_id(case_id);
        }

        if let Some(id) = audit.entity_id {
            input.entity_id = Some(id.to_string());
        }

        AuditService::write_best_effort(app, input);
        Ok::<(), AppErrorDto>(())
    })();

    if let Err(err) = result {
        eprintln!(
            "[audit] write_protected_access_denied failed: {}",
            err.message
        );
    }
}
