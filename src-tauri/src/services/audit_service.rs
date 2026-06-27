use rusqlite::Connection;
use serde::Serialize;
use serde_json::Value;

use crate::errors::app_error::AppErrorDto;
use crate::repositories::audit_repository::{AuditRepository, CreateAuditLogRecord};
use crate::security::session::CurrentUserDto;

pub const AUDIT_RESULT_SUCCESS: &str = "success";
pub const AUDIT_SEVERITY_INFO: &str = "info";

pub const ENTITY_TYPE_EVENT: &str = "event";

pub const EVENT_CREATED: &str = "EVENT_CREATED";
pub const EVENT_UPDATED: &str = "EVENT_UPDATED";
pub const EVENT_DELETED: &str = "EVENT_DELETED";
pub const EVENT_REPORT_FLAG_CHANGED: &str = "EVENT_REPORT_FLAG_CHANGED";

pub struct AuditService;

impl AuditService {
    pub fn write_success(
        conn: &Connection,
        user: &CurrentUserDto,
        action: &str,
        entity_type: &str,
        entity_id: Option<&str>,
        case_id: Option<&str>,
        old_value: Option<Value>,
        new_value: Option<Value>,
        technical_details: Option<Value>,
    ) -> Result<(), AppErrorDto> {
        let record = CreateAuditLogRecord {
            user_id: Some(user.user_id.clone()),
            username: user.username.clone(),
            user_role: user.role.clone(),

            action: action.to_string(),
            entity_type: entity_type.to_string(),
            entity_id: entity_id.map(str::to_string),
            case_id: case_id.map(str::to_string),

            result: AUDIT_RESULT_SUCCESS.to_string(),
            severity: AUDIT_SEVERITY_INFO.to_string(),

            old_value: serialize_optional_json(old_value)?,
            new_value: serialize_optional_json(new_value)?,
            technical_details: serialize_optional_json(technical_details)?,

            app_version: env!("CARGO_PKG_VERSION").to_string(),
        };

        AuditRepository::insert(conn, record)
    }
}

fn serialize_optional_json(value: Option<Value>) -> Result<Option<String>, AppErrorDto> {
    match value {
        Some(value) => serde_json::to_string(&value)
            .map(Some)
            .map_err(|err| AppErrorDto::new("ERR_AUDIT_SERIALIZATION", &err.to_string(), None)),
        None => Ok(None),
    }
}

pub fn to_json_value<T: Serialize>(value: &T) -> Result<Value, AppErrorDto> {
    serde_json::to_value(value)
        .map_err(|err| AppErrorDto::new("ERR_AUDIT_SERIALIZATION", &err.to_string(), None))
}
