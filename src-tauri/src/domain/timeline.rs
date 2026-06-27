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

    pub query: Option<String>,
    pub event_type: Option<String>,
    pub object_id: Option<String>,
    pub material_id: Option<String>,
    pub date_from: Option<String>,
    pub date_to: Option<String>,
    pub include_in_report: Option<bool>,
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

#[derive(Debug, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct EventLinkedObjectDto {
    pub id: String,
    pub object_id: String,
    pub object_code: String,
    pub object_type: String,
    pub title: String,
    pub link_note: String,
}

#[derive(Debug, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct EventLinkedMaterialDto {
    pub id: String,
    pub material_id: String,
    pub material_code: String,
    pub title: String,
    pub material_type: String,
    pub link_note: String,
}

#[derive(Debug, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct EventDetailsDto {
    pub event_item: TimelineEventDto,
    pub linked_objects: Vec<EventLinkedObjectDto>,
    pub linked_materials: Vec<EventLinkedMaterialDto>,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct GetEventByIdPayload {
    pub case_id: String,
    pub event_id: String,
}

#[derive(Debug, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct GetEventByIdResponse {
    pub event_details: EventDetailsDto,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct UpdateEventPayload {
    pub case_id: String,
    pub event_id: String,

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
pub struct UpdateEventResponse {
    pub event_details: EventDetailsDto,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct SoftDeleteEventPayload {
    pub case_id: String,
    pub event_id: String,
}

#[derive(Debug, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct SoftDeleteEventResponse {
    pub event_id: String,
}
