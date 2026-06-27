use std::fs;
use std::path::PathBuf;
use std::time::{SystemTime, UNIX_EPOCH};

use rusqlite::Connection;
use serde::Serialize;
use serde_json::Value;
use tauri::{AppHandle, Manager};

use crate::db::connection::open_connection;
use crate::domain::audit::{
    AuditLogDetailsDto, AuditLogDto, ExportAuditLogPayload, ExportAuditLogResponse,
    GetAuditActionsResponse, GetAuditLogByIdPayload, GetAuditLogByIdResponse, GetAuditLogsPayload,
    GetAuditLogsResponse, GetAuditUsersResponse,
};
use crate::errors::app_error::AppErrorDto;
use crate::repositories::audit_repository::{
    AuditLogFilters, AuditRepository, CreateAuditLogRecord,
};
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

    pub fn get_audit_logs(
        conn: &Connection,
        current_user: &CurrentUserDto,
        payload: GetAuditLogsPayload,
    ) -> Result<GetAuditLogsResponse, AppErrorDto> {
        let user_role = current_user.role.as_str();

        if user_role == "viewer" {
            return Err(AppErrorDto::access_denied(
                "Недостаточно прав для просмотра журнала действий.",
            ));
        }

        let page = payload.page.unwrap_or(1).max(1);
        let page_size = payload.page_size.unwrap_or(50).clamp(10, 200);
        let offset = (page - 1) * page_size;

        let requested_user_id = normalize_optional_filter(payload.user_id);

        let user_id_filter = if user_role == "administrator" {
            requested_user_id
        } else {
            Some(current_user.user_id.clone())
        };

        let filters = AuditLogFilters {
            action: normalize_optional_filter(payload.action),
            result: normalize_optional_filter(payload.result),
            severity: normalize_optional_filter(payload.severity),
            case_id: normalize_optional_filter(payload.case_id),
            entity_type: normalize_optional_filter(payload.entity_type),
            date_from: normalize_optional_filter(payload.date_from),
            date_to: normalize_optional_filter(payload.date_to),
            user_id: user_id_filter,
            limit: page_size,
            offset,
        };

        let total = AuditRepository::count_audit_logs(conn, &filters)?;
        let items = AuditRepository::get_audit_logs(conn, &filters)?;

        Ok(GetAuditLogsResponse {
            items,
            total,
            page,
            page_size,
        })
    }

    pub fn get_audit_actions(
        conn: &Connection,
        current_user: &CurrentUserDto,
    ) -> Result<GetAuditActionsResponse, AppErrorDto> {
        let user_role = current_user.role.as_str();

        if user_role == "viewer" {
            return Err(AppErrorDto::access_denied(
                "Недостаточно прав для просмотра журнала действий.",
            ));
        }

        let restricted_user_id = if user_role == "administrator" {
            None
        } else {
            Some(current_user.user_id.as_str())
        };

        let items = AuditRepository::get_audit_actions(conn, restricted_user_id)?;

        Ok(GetAuditActionsResponse { items })
    }

    pub fn get_audit_users(
        conn: &Connection,
        current_user: &CurrentUserDto,
    ) -> Result<GetAuditUsersResponse, AppErrorDto> {
        let user_role = current_user.role.as_str();

        if user_role != "administrator" {
            return Err(AppErrorDto::access_denied(
                "Фильтр по пользователям доступен только администратору.",
            ));
        }

        let items = AuditRepository::get_audit_users(conn)?;

        Ok(GetAuditUsersResponse { items })
    }

    pub fn get_audit_log_by_id(
        conn: &Connection,
        current_user: &CurrentUserDto,
        payload: GetAuditLogByIdPayload,
    ) -> Result<GetAuditLogByIdResponse, AppErrorDto> {
        let user_role = current_user.role.as_str();

        if user_role == "viewer" {
            return Err(AppErrorDto::access_denied(
                "Недостаточно прав для просмотра журнала действий.",
            ));
        }

        let audit_log_id = payload.audit_log_id.trim();

        if audit_log_id.is_empty() {
            return Err(AppErrorDto::validation(
                "Не указан идентификатор записи журнала.",
            ));
        }

        let item = AuditRepository::get_audit_log_by_id(conn, audit_log_id)?
            .ok_or_else(|| AppErrorDto::not_found("Запись журнала не найдена."))?;

        if user_role != "administrator"
            && item.user_id.as_deref() != Some(current_user.user_id.as_str())
        {
            return Err(AppErrorDto::access_denied(
                "Недостаточно прав для просмотра этой записи журнала.",
            ));
        }

        Ok(GetAuditLogByIdResponse {
            item: AuditLogDetailsDto {
                id: item.id,
                user_id: item.user_id,
                username: item.username,
                user_role: item.user_role,
                action: item.action,
                entity_type: item.entity_type,
                entity_id: item.entity_id,
                case_id: item.case_id,
                result: item.result,
                severity: item.severity,
                old_value: parse_optional_audit_json(item.old_value),
                new_value: parse_optional_audit_json(item.new_value),
                technical_details: parse_optional_audit_json(item.technical_details),
                app_version: item.app_version,
                created_at: item.created_at,
            },
        })
    }

    pub fn export_audit_log(
        app: &AppHandle,
        current_user: &CurrentUserDto,
        payload: ExportAuditLogPayload,
    ) -> Result<ExportAuditLogResponse, AppErrorDto> {
        let user_role = current_user.role.as_str();

        if user_role != "administrator" {
            return Err(AppErrorDto::access_denied(
                "Экспорт журнала действий доступен только администратору.",
            ));
        }

        let conn = open_connection(app)?;

        let filters = AuditLogFilters {
            action: normalize_optional_filter(payload.action),
            result: normalize_optional_filter(payload.result),
            severity: normalize_optional_filter(payload.severity),
            case_id: normalize_optional_filter(payload.case_id),
            entity_type: normalize_optional_filter(payload.entity_type),
            date_from: normalize_optional_filter(payload.date_from),
            date_to: normalize_optional_filter(payload.date_to),
            user_id: normalize_optional_filter(payload.user_id),
            limit: i64::MAX,
            offset: 0,
        };

        let items = AuditRepository::export_audit_logs(&conn, &filters)?;

        if items.is_empty() {
            return Err(AppErrorDto::validation(
                "Не найдено записей для экспорта с указанными фильтрами.",
            ));
        }

        let csv_content = build_audit_log_csv(&items);
        let export_dir = resolve_audit_export_dir(app)?;
        let timestamp = unix_timestamp_for_filename()?;
        let file_name = format!("audit-log-{}.csv", timestamp);
        let file_path = export_dir.join(&file_name);

        fs::write(&file_path, &csv_content)
            .map_err(|err| AppErrorDto::filesystem(err.to_string()))?;

        let exported_count = items.len() as i64;

        Self::write_audit_export_event_best_effort(app.clone(), current_user, exported_count);

        Ok(ExportAuditLogResponse {
            file_path: file_path.to_string_lossy().to_string(),
            exported_count,
            format: "csv".to_string(),
        })
    }

    fn write_audit_export_event_best_effort(
        app: AppHandle,
        user: &CurrentUserDto,
        exported_count: i64,
    ) {
        let input = AuditSuccessInput::new(
            user,
            "AUDIT_LOG_EXPORTED",
            "audit",
            None,
            None,
            None,
            to_json_value(&serde_json::json!({ "exported_count": exported_count })).ok(),
            None,
        );

        Self::write_success_non_blocking(app, input);
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

fn csv_cell(value: &str) -> String {
    let escaped = value.replace('"', "\"\"");
    format!("\"{}\"", escaped)
}

fn csv_optional_cell(value: Option<&str>) -> String {
    csv_cell(value.unwrap_or(""))
}

fn build_audit_log_csv(items: &[AuditLogDto]) -> String {
    let mut output = String::new();

    output.push_str(
        "id,created_at,user_id,username,user_role,action,entity_type,entity_id,case_id,result,severity,old_value,new_value,technical_details,app_version\n",
    );

    for item in items {
        let row = [
            csv_cell(&item.id),
            csv_cell(&item.created_at),
            csv_optional_cell(item.user_id.as_deref()),
            csv_cell(&item.username),
            csv_cell(&item.user_role),
            csv_cell(&item.action),
            csv_cell(&item.entity_type),
            csv_optional_cell(item.entity_id.as_deref()),
            csv_optional_cell(item.case_id.as_deref()),
            csv_cell(&item.result),
            csv_cell(&item.severity),
            csv_optional_cell(item.old_value.as_deref()),
            csv_optional_cell(item.new_value.as_deref()),
            csv_optional_cell(item.technical_details.as_deref()),
            csv_cell(&item.app_version),
        ];

        output.push_str(&row.join(","));
        output.push('\n');
    }

    output
}

fn resolve_audit_export_dir(app: &AppHandle) -> Result<PathBuf, AppErrorDto> {
    let mut export_dir = app
        .path()
        .app_data_dir()
        .map_err(|err| AppErrorDto::internal(err.to_string()))?;

    export_dir.push("exports");
    export_dir.push("audit-log");

    fs::create_dir_all(&export_dir)
        .map_err(|err| AppErrorDto::filesystem(err.to_string()))?;

    Ok(export_dir)
}

fn unix_timestamp_for_filename() -> Result<u64, AppErrorDto> {
    SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .map(|duration| duration.as_secs())
        .map_err(|err| AppErrorDto::internal(err.to_string()))
}

fn parse_optional_audit_json(raw: Option<String>) -> Option<Value> {
    raw.map(|value| serde_json::from_str::<Value>(&value).unwrap_or(Value::String(value)))
}

fn normalize_optional_filter(value: Option<String>) -> Option<String> {
    value
        .map(|item| item.trim().to_string())
        .filter(|item| !item.is_empty())
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
