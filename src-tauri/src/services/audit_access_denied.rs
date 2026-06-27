use tauri::AppHandle;

use crate::audit::audit_metadata;
use crate::audit::audit_service::{AuditService, AuditWriteInput};
use crate::errors::app_error::AppErrorDto;
use crate::security::session::CurrentUserDto;

pub fn access_denied_error(
    app: &AppHandle,
    current_user: &CurrentUserDto,
    command_name: &str,
    entity_type: &str,
    entity_id: Option<String>,
    required_role: &str,
    message: &str,
) -> AppErrorDto {
    let result = (|| {
        let technical_details = audit_metadata::access_denied(
            message,
            command_name,
            Some(&current_user.role),
            Some(required_role),
        )?;

        let mut input = AuditWriteInput::failure(
            current_user,
            crate::domain::audit_action::audit::ACCESS_DENIED,
        )
        .with_entity_type(entity_type)
        .with_details(technical_details);

        input.result = "denied".to_string(); // Keep EXACT compatibility
        input.severity = "warning".to_string();

        if let Some(id) = entity_id {
            input.entity_id = Some(id);
        }

        AuditService::write_best_effort(app, input);
        Ok::<(), AppErrorDto>(())
    })();

    if let Err(err) = result {
        eprintln!("[audit] access_denied_error write failed: {}", err.message);
    }

    AppErrorDto::access_denied(message)
}
