use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct TimelineEventDto {
    pub id: String,
    pub case_id: String,
    pub event_code: String,
    pub event_type: String,
    pub title: String,
    pub description: String,
    pub event_date: String,
    pub event_time: Option<String>,
    pub date_precision: String,
    pub period_start: Option<String>,
    pub period_end: Option<String>,
    pub source_note: String,
    pub analyst_comment: String,
    pub include_in_report: bool,
    pub linked_object_count: i64,
    pub linked_material_count: i64,
    pub created_by_user_id: String,
    pub created_at: String,
    pub updated_at: String,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct GetTimelinePayload {
    pub case_id: String,
}

#[derive(Debug, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct GetTimelineResponse {
    pub items: Vec<TimelineEventDto>,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CreateEventPayload {
    pub case_id: String,
    pub event_type: String,
    pub title: String,
    pub description: String,
    pub event_date: String,
    pub event_time: Option<String>,
    pub date_precision: String,
    pub period_start: Option<String>,
    pub period_end: Option<String>,
    pub source_note: String,
    pub analyst_comment: String,
    pub include_in_report: bool,
    pub object_ids: Vec<String>,
    pub material_ids: Vec<String>,
    pub link_note: String,
}

#[derive(Debug, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct CreateEventResponse {
    pub event_item: TimelineEventDto,
}
