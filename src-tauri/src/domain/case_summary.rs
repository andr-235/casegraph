use serde::{Deserialize, Serialize};

/// Сводка по делу — счётчики для сайдбара
#[derive(Debug, Clone, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct CaseSummaryDto {
    pub object_count: i64,
    pub key_object_count: i64,
    pub material_count: i64,
    pub integrity_issue_count: i64,
    pub relation_count: i64,
    pub event_count: i64,
    pub report_event_count: i64,
    /// Максимальный updated_at среди всех записей, участвующих в сводке
    pub updated_at: String,
}

/// Payload для команды get_case_summary
#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct GetCaseSummaryPayload {
    pub case_id: String,
}
