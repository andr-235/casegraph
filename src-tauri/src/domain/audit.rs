use serde::{Deserialize, Serialize};
use serde_json::Value;

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct GetAuditLogsPayload {
    pub action: Option<String>,
    pub result: Option<String>,
    pub severity: Option<String>,
    pub case_id: Option<String>,
    pub entity_type: Option<String>,
    pub user_id: Option<String>,
    pub date_from: Option<String>,
    pub date_to: Option<String>,
    pub page: Option<i64>,
    pub page_size: Option<i64>,
}

#[derive(Debug, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct AuditLogDto {
    pub id: String,

    pub user_id: Option<String>,
    pub username: String,
    pub user_role: String,

    pub action: String,
    pub entity_type: String,
    pub entity_id: Option<String>,
    pub case_id: Option<String>,

    pub result: String,
    pub severity: String,

    pub old_value: Option<String>,
    pub new_value: Option<String>,
    pub technical_details: Option<String>,

    pub app_version: String,
    pub created_at: String,
}

#[derive(Debug, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct GetAuditLogsResponse {
    pub items: Vec<AuditLogDto>,
    pub total: i64,
    pub page: i64,
    pub page_size: i64,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct GetAuditLogByIdPayload {
    pub audit_log_id: String,
}

#[derive(Debug, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct AuditLogDetailsDto {
    pub id: String,

    pub user_id: Option<String>,
    pub username: String,
    pub user_role: String,

    pub action: String,
    pub entity_type: String,
    pub entity_id: Option<String>,
    pub case_id: Option<String>,

    pub result: String,
    pub severity: String,

    pub old_value: Option<Value>,
    pub new_value: Option<Value>,
    pub technical_details: Option<Value>,

    pub app_version: String,
    pub created_at: String,
}

#[derive(Debug, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct GetAuditLogByIdResponse {
    pub item: AuditLogDetailsDto,
}

#[derive(Debug, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct AuditActionOptionDto {
    pub action: String,
    pub count: i64,
}

#[derive(Debug, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct GetAuditActionsResponse {
    pub items: Vec<AuditActionOptionDto>,
}

#[derive(Debug, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct AuditUserOptionDto {
    pub user_id: String,
    pub username: String,
    pub user_role: String,
    pub count: i64,
}

#[derive(Debug, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct GetAuditUsersResponse {
    pub items: Vec<AuditUserOptionDto>,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ExportAuditLogPayload {
    pub action: Option<String>,
    pub result: Option<String>,
    pub severity: Option<String>,
    pub case_id: Option<String>,
    pub entity_type: Option<String>,
    pub user_id: Option<String>,
    pub date_from: Option<String>,
    pub date_to: Option<String>,
}

#[derive(Debug, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct ExportAuditLogResponse {
    pub file_path: String,
    pub exported_count: i64,
    pub format: String,
}

#[derive(Debug)]
pub struct AuditAccessDeniedInput {
    pub command_name: String,
    pub entity_type: String,
    pub entity_id: Option<String>,
    pub description: String,
    pub required_role: String,
}
