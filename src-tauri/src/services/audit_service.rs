use rusqlite::Connection;
use serde::Serialize;
use serde_json::Value;
use tauri::AppHandle;

use crate::db::connection::open_connection;
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

    pub fn write_success_non_blocking(app: AppHandle, input: AuditSuccessInput) {
        let record = match Self::build_success_record(input) {
            Ok(record) => record,
            Err(_) => {
                eprintln!("[audit] failed to build audit record");
                return;
            }
        };

        tauri::async_runtime::spawn(async move {
            let conn = match open_connection(&app) {
                Ok(conn) => conn,
                Err(_) => {
                    eprintln!("[audit] failed to open database connection");
                    return;
                }
            };

            if AuditRepository::insert(&conn, record).is_err() {
                eprintln!("[audit] failed to insert audit log record");
            }
        });
    }

    fn build_success_record(input: AuditSuccessInput) -> Result<CreateAuditLogRecord, AppErrorDto> {
        Ok(CreateAuditLogRecord {
            user_id: Some(input.user_id),
            username: input.username,
            user_role: input.user_role,

            action: input.action,
            entity_type: input.entity_type,
            entity_id: input.entity_id,
            case_id: input.case_id,

            result: AUDIT_RESULT_SUCCESS.to_string(),
            severity: AUDIT_SEVERITY_INFO.to_string(),

            old_value: serialize_optional_json(input.old_value)?,
            new_value: serialize_optional_json(input.new_value)?,
            technical_details: serialize_optional_json(input.technical_details)?,

            app_version: env!("CARGO_PKG_VERSION").to_string(),
        })
    }
}

#[derive(Debug)]
pub struct AuditSuccessInput {
    pub user_id: String,
    pub username: String,
    pub user_role: String,

    pub action: String,
    pub entity_type: String,
    pub entity_id: Option<String>,
    pub case_id: Option<String>,

    pub old_value: Option<Value>,
    pub new_value: Option<Value>,
    pub technical_details: Option<Value>,
}

impl AuditSuccessInput {
    pub fn new(
        user: &CurrentUserDto,
        action: &str,
        entity_type: &str,
        entity_id: Option<&str>,
        case_id: Option<&str>,
        old_value: Option<Value>,
        new_value: Option<Value>,
        technical_details: Option<Value>,
    ) -> Self {
        Self {
            user_id: user.user_id.clone(),
            username: user.username.clone(),
            user_role: user.role.clone(),

            action: action.to_string(),
            entity_type: entity_type.to_string(),
            entity_id: entity_id.map(str::to_string),
            case_id: case_id.map(str::to_string),

            old_value,
            new_value,
            technical_details,
        }
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
