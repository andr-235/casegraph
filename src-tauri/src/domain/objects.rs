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

#[derive(Debug, Clone, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct ObjectDetailsDto {
    pub id: String,
    pub case_id: String,
    pub object_code: String,
    pub object_type: String,
    pub title: String,
    pub value: Option<String>,
    pub description: String,
    pub is_key: bool,
    pub confidence_note: String,
    pub include_in_report: bool,
    pub linked_material_count: i64,
    pub relation_count: i64,
    pub created_at: String,
    pub updated_at: String,
    pub linked_materials: Vec<LinkedObjectMaterialDto>,
    pub relations: Vec<ObjectRelationSummaryDto>,
}

#[derive(Debug, Clone, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct LinkedObjectMaterialDto {
    pub id: String,
    pub material_code: String,
    pub title: String,
    pub material_type: String,
    pub hash_status: String,
    pub link_reason: String,
}

#[derive(Debug, Clone, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct ObjectRelationSummaryDto {
    pub relation_id: String,
    pub relation_code: String,
    pub relation_type: String,
    pub counterpart_object_id: String,
    pub counterpart_object_code: String,
    pub counterpart_title: String,
    pub confidence_level: String,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct GetObjectByIdPayload {
    pub case_id: String,
    pub object_id: String,
}

#[derive(Debug, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct GetObjectByIdResponse {
    pub object_item: ObjectDetailsDto,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct UpdateObjectPayload {
    pub case_id: String,
    pub object_id: String,
    pub title: String,
    pub value: Option<String>,
    pub description: Option<String>,
    pub is_key: bool,
    pub confidence_note: Option<String>,
    pub include_in_report: bool,
}

#[derive(Debug, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct UpdateObjectResponse {
    pub object_item: ObjectDetailsDto,
}
