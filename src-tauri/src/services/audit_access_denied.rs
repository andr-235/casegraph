use tauri::AppHandle;

use crate::db::connection::open_connection;
use crate::domain::audit::AuditAccessDeniedInput;
use crate::errors::app_error::AppErrorDto;
use crate::repositories::audit_repository::AuditRepository;
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
    write_access_denied_best_effort(
        app,
        current_user,
        AuditAccessDeniedInput {
            command_name: command_name.to_string(),
            entity_type: entity_type.to_string(),
            entity_id,
            description: message.to_string(),
            required_role: required_role.to_string(),
        },
    );

    AppErrorDto::access_denied(message)
}

fn write_access_denied_best_effort(
    app: &AppHandle,
    current_user: &CurrentUserDto,
    input: AuditAccessDeniedInput,
) {
    let conn = match open_connection(app) {
        Ok(conn) => conn,
        Err(error) => {
            eprintln!(
                "[audit] access_denied write skipped: failed to open db: {:?}",
                error
            );
            return;
        }
    };

    if let Err(error) = AuditRepository::insert_access_denied(&conn, current_user, &input) {
        eprintln!("[audit] access_denied write failed: {:?}", error);
    }
}
