use tauri::AppHandle;

use crate::db::connection::open_connection;
use crate::domain::audit::AuditAccessDeniedInput;
use crate::repositories::audit_repository::AuditRepository;
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
        let conn = open_connection(app)?;

        let required_role_str = audit.required_role.unwrap_or("");
        let entity_type_str = audit.entity_type.unwrap_or("generic");
        let entity_id_str = audit.entity_id.map(|s| s.to_string());

        let description = format!(
            "Access denied for action '{}'. Reason: {}.",
            audit.command_name, audit.reason
        );

        let input = AuditAccessDeniedInput {
            command_name: audit.command_name.to_string(),
            entity_type: entity_type_str.to_string(),
            entity_id: entity_id_str,
            description,
            required_role: required_role_str.to_string(),
        };

        AuditRepository::insert_access_denied(&conn, current_user, &input)
    })();

    if let Err(err) = result {
        eprintln!("audit access denied write failed: {:?}", err);
    }
}
