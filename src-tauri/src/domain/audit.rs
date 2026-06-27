use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct GetAuditLogsPayload {
    pub action: Option<String>,
    pub result: Option<String>,
    pub severity: Option<String>,
    pub case_id: Option<String>,
    pub entity_type: Option<String>,
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
