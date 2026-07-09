use serde::{Deserialize, Serialize};

use crate::domain::case_summary::CaseSummaryDto;
use crate::domain::cases::CaseDto;

/// Краткое представление объекта
#[derive(Debug, Clone, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct ObjectPreviewDto {
    pub id: String,
    pub object_code: String,
    pub object_type: String,
    pub title: String,
    pub is_key: bool,
}

/// Запись в ленте последней активности
#[derive(Debug, Clone, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct ActivityItemDto {
    pub entity_type: String,
    pub entity_id: String,
    pub code: String,
    pub title: String,
    pub timestamp: String,
    pub action: String,
}

/// Карточка дела с метриками
#[derive(Debug, Clone, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct CaseOverviewDto {
    pub case_item: CaseDto,
    pub summary: CaseSummaryDto,
    pub key_objects: Vec<ObjectPreviewDto>,
    pub recent_activity: Vec<ActivityItemDto>,
}

/// Payload для команды get_case_overview
#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct GetCaseOverviewPayload {
    pub case_id: String,
}
