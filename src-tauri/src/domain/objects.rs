use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct ObjectListItemDto {
    pub id: String,
    pub case_id: String,
    pub object_code: String,
    pub object_type: String,
    pub title: String,
    pub value: Option<String>,
    pub description: String,
    pub is_key: bool,
    pub include_in_report: bool,
    pub linked_material_count: i64,
    pub relation_count: i64,
    pub created_at: String,
    pub updated_at: String,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CreateObjectPayload {
    pub case_id: String,
    pub object_type: String,
    pub title: String,
    pub value: Option<String>,
    pub description: Option<String>,
    pub is_key: Option<bool>,
    pub confidence_note: Option<String>,
    pub include_in_report: Option<bool>,
}

#[derive(Debug, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct CreateObjectResponse {
    pub object_item: ObjectListItemDto,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct GetObjectsPayload {
    pub case_id: String,
}

#[derive(Debug, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct GetObjectsResponse {
    pub items: Vec<ObjectListItemDto>,
}
