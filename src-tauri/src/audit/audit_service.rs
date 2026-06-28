use std::fs;
use std::path::PathBuf;
use std::time::{SystemTime, UNIX_EPOCH};

use serde_json::Value;
use tauri::{AppHandle, Manager};

use crate::audit::audit_error_sanitizer::sanitize_audit_details;
use crate::audit::audit_safe_value::{AuditSafeDetails, AuditSafeSnapshot};
use crate::db::connection::open_connection;
use crate::domain::audit::{
    AuditLogDetailsDto, AuditLogDto, ExportAuditLogPayload, ExportAuditLogResponse,
    GetAuditActionsResponse, GetAuditLogByIdPayload, GetAuditLogByIdResponse, GetAuditLogsPayload,
    GetAuditLogsResponse, GetAuditUsersResponse,
};
use crate::domain::audit_action;
use crate::errors::app_error::AppErrorDto;
use crate::security::session::{CurrentUserDto, SessionState};
use crate::security::ProtectedOperation;
use crate::security::ProtectedServiceContext;
use crate::services::audit_guards::{
    audit_user_filter_for_reader, require_audit_admin, require_audit_reader,
    require_no_user_filter_for_analyst, require_own_audit_entry_or_admin,
};

use super::audit_repository::{AuditInsertRow, AuditLogFilters, AuditRepository};

pub const AUDIT_RESULT_SUCCESS: &str = "success";
pub const AUDIT_SEVERITY_INFO: &str = "info";

pub const ENTITY_TYPE_EVENT: &str = "event";

pub const EVENT_CREATED: &str = audit_action::timeline::EVENT_CREATED;
pub const EVENT_UPDATED: &str = audit_action::timeline::EVENT_UPDATED;
pub const EVENT_DELETED: &str = audit_action::timeline::EVENT_DELETED;
pub const EVENT_REPORT_FLAG_CHANGED: &str = audit_action::timeline::EVENT_REPORT_INCLUDE_CHANGED;

pub struct AuditService;

#[derive(Debug)]
pub struct AuditWriteInput {
    pub action: String,
    pub result: String,
    pub severity: String,

    pub user_id: String,
    pub username: String,
    pub user_role: String,

    pub case_id: Option<String>,
    pub entity_type: String,
    pub entity_id: Option<String>,

    pub old_value: Option<AuditSafeSnapshot>,
    pub new_value: Option<AuditSafeSnapshot>,
    pub technical_details: Option<AuditSafeDetails>,
}

impl AuditWriteInput {
    pub fn success(user: &CurrentUserDto, action: impl Into<String>) -> Self {
        Self {
            action: action.into(),
            result: AUDIT_RESULT_SUCCESS.to_string(),
            severity: AUDIT_SEVERITY_INFO.to_string(),
            user_id: user.user_id.clone(),
            username: user.username.clone(),
            user_role: user.role.clone(),
            case_id: None,
            entity_type: String::new(),
            entity_id: None,
            old_value: None,
            new_value: None,
            technical_details: None,
        }
    }

    pub fn failure(user: &CurrentUserDto, action: impl Into<String>) -> Self {
        Self {
            action: action.into(),
            result: "failure".to_string(),
            severity: "warning".to_string(),
            user_id: user.user_id.clone(),
            username: user.username.clone(),
            user_role: user.role.clone(),
            case_id: None,
            entity_type: String::new(),
            entity_id: None,
            old_value: None,
            new_value: None,
            technical_details: None,
        }
    }

    pub fn with_case_id(mut self, case_id: impl Into<String>) -> Self {
        self.case_id = Some(case_id.into());
        self
    }

    pub fn with_case_id_if_some(mut self, case_id: Option<String>) -> Self {
        self.case_id = case_id;
        self
    }

    pub fn with_entity(
        mut self,
        entity_type: impl Into<String>,
        entity_id: impl Into<String>,
    ) -> Self {
        self.entity_type = entity_type.into();
        self.entity_id = Some(entity_id.into());
        self
    }

    pub fn with_entity_if_some(
        mut self,
        entity_type: Option<String>,
        entity_id: Option<String>,
    ) -> Self {
        self.entity_type = entity_type.unwrap_or_default();
        self.entity_id = entity_id;
        self
    }

    pub fn with_entity_type(mut self, entity_type: impl Into<String>) -> Self {
        self.entity_type = entity_type.into();
        self
    }

    pub fn with_snapshots(
        mut self,
        old_value: Option<AuditSafeSnapshot>,
        new_value: Option<AuditSafeSnapshot>,
    ) -> Self {
        self.old_value = old_value;
        self.new_value = new_value;
        self
    }

    pub fn with_details(mut self, details: AuditSafeDetails) -> Self {
        self.technical_details = Some(details);
        self
    }
}

impl AuditService {
    /// Write an audit record in a best-effort, non-blocking way.
    ///
    /// Accepts the new typed `AuditWriteInput`. Safe wrappers are converted
    /// to `Value` only here — this is the only place in the codebase allowed
    /// to call `into_value()` on them.
    pub fn write_best_effort(app: &AppHandle, input: AuditWriteInput) {
        let app = app.clone();

        let old_value = input.old_value.map(|v| v.into_value());
        let new_value = input.new_value.map(|v| v.into_value());
        let technical_details = input
            .technical_details
            .map(|v| sanitize_audit_details(v.into_value()));

        let record = AuditInsertRow {
            user_id: Some(input.user_id),
            username: input.username,
            user_role: input.user_role,
            action: input.action,
            entity_type: input.entity_type,
            entity_id: input.entity_id,
            case_id: input.case_id,
            result: input.result,
            severity: input.severity,
            old_value: old_value.map(|v| serde_json::to_string(&v).unwrap_or_default()),
            new_value: new_value.map(|v| serde_json::to_string(&v).unwrap_or_default()),
            technical_details: technical_details
                .map(|v| serde_json::to_string(&v).unwrap_or_default()),
            app_version: env!("CARGO_PKG_VERSION").to_string(),
        };

        tauri::async_runtime::spawn(async move {
            let conn = match open_connection(&app) {
                Ok(conn) => conn,
                Err(_) => {
                    eprintln!("[audit] write_best_effort: failed to open db");
                    return;
                }
            };
            if let Err(err) = AuditRepository::insert(&conn, record) {
                eprintln!("[audit] write_best_effort failed: {}", err.message);
            }
        });
    }

    pub fn get_audit_logs(
        app: &AppHandle,
        _session: &SessionState,
        payload: GetAuditLogsPayload,
    ) -> Result<GetAuditLogsResponse, AppErrorDto> {
        let context =
            ProtectedServiceContext::require_operation(app, ProtectedOperation::AuditLogRead)?;
        let current_user = &context.current_user;
        let conn = &context.conn;

        require_audit_reader(
            app,
            current_user,
            "get_audit_logs",
            "Недостаточно прав для просмотра журнала действий.",
        )?;

        require_no_user_filter_for_analyst(app, current_user, "get_audit_logs", &payload.user_id)?;

        let page = payload.page.unwrap_or(1).max(1);
        let page_size = payload.page_size.unwrap_or(50).clamp(10, 200);
        let offset = (page - 1) * page_size;

        let requested_user_id = normalize_optional_filter(payload.user_id);
        let user_id_filter = audit_user_filter_for_reader(current_user, requested_user_id);

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
        app: &AppHandle,
        _session: &SessionState,
    ) -> Result<GetAuditActionsResponse, AppErrorDto> {
        let context =
            ProtectedServiceContext::require_operation(app, ProtectedOperation::AuditLogRead)?;
        let current_user = &context.current_user;
        let conn = &context.conn;

        require_audit_reader(
            app,
            current_user,
            "get_audit_actions",
            "Недостаточно прав для получения словаря действий журнала.",
        )?;

        let user_id_filter = audit_user_filter_for_reader(current_user, None);
        let items = AuditRepository::get_audit_actions(conn, user_id_filter.as_deref())?;

        Ok(GetAuditActionsResponse { items })
    }

    pub fn get_audit_users(
        app: &AppHandle,
        _session: &SessionState,
    ) -> Result<GetAuditUsersResponse, AppErrorDto> {
        let context =
            ProtectedServiceContext::require_operation(app, ProtectedOperation::UserManage)?;
        let current_user = &context.current_user;
        let conn = &context.conn;

        require_audit_admin(
            app,
            current_user,
            "get_audit_users",
            "Словарь пользователей журнала доступен только администратору.",
        )?;

        let items = AuditRepository::get_audit_users(conn)?;

        Ok(GetAuditUsersResponse { items })
    }

    pub fn get_audit_log_by_id(
        app: &AppHandle,
        _session: &SessionState,
        payload: GetAuditLogByIdPayload,
    ) -> Result<GetAuditLogByIdResponse, AppErrorDto> {
        let context =
            ProtectedServiceContext::require_operation(app, ProtectedOperation::AuditLogRead)?;
        let current_user = &context.current_user;
        let conn = &context.conn;

        require_audit_reader(
            app,
            current_user,
            "get_audit_log_by_id",
            "Недостаточно прав для просмотра события журнала.",
        )?;

        let audit_log_id = payload.audit_log_id.trim();

        if audit_log_id.is_empty() {
            return Err(AppErrorDto::validation(
                "Не указан идентификатор записи журнала.",
            ));
        }

        let item = AuditRepository::get_audit_log_by_id(conn, audit_log_id)?
            .ok_or_else(|| AppErrorDto::not_found("Запись журнала не найдена."))?;

        require_own_audit_entry_or_admin(
            app,
            current_user,
            "get_audit_log_by_id",
            &payload.audit_log_id,
            item.user_id.as_deref(),
        )?;

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
        _session: &SessionState,
        payload: ExportAuditLogPayload,
    ) -> Result<ExportAuditLogResponse, AppErrorDto> {
        let context =
            ProtectedServiceContext::require_operation(app, ProtectedOperation::UserManage)?;
        let current_user = &context.current_user;
        let conn = &context.conn;

        require_audit_admin(
            app,
            current_user,
            "export_audit_log",
            "Экспорт журнала действий доступен только администратору.",
        )?;

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
        let result = (|| {
            let technical_details = crate::audit::audit_metadata::audit_log_exported(
                exported_count as usize,
                "csv",
                false,
            )?;

            Self::write_best_effort(
                &app,
                AuditWriteInput::success(user, audit_action::audit::LOG_EXPORTED)
                    .with_entity_type("audit")
                    .with_details(technical_details),
            );
            Ok::<(), AppErrorDto>(())
        })();

        if let Err(err) = result {
            eprintln!("[audit] write_audit_export_event failed: {}", err.message);
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

    fs::create_dir_all(&export_dir).map_err(|err| AppErrorDto::filesystem(err.to_string()))?;

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
