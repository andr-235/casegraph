use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct RelationObjectDto {
    pub id: String,
    pub object_code: String,
    pub object_type: String,
    pub title: String,
    pub value: Option<String>,
    pub is_key: bool,
}

#[derive(Debug, Clone, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct RelationMaterialDto {
    pub id: String,
    pub material_code: String,
    pub title: String,
    pub material_type: String,
    pub integrity_status: String,
}

#[derive(Debug, Clone, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct RelationListItemDto {
    pub id: String,
    pub case_id: String,
    pub relation_code: String,
    pub relation_type: String,
    pub title: Option<String>,
    pub basis: String,
    pub confidence_level: String,
    pub source_object: RelationObjectDto,
    pub target_object: RelationObjectDto,
    pub supporting_material: Option<RelationMaterialDto>,
    pub include_in_report: bool,
    pub created_at: String,
    pub updated_at: String,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CreateRelationPayload {
    pub case_id: String,
    pub source_object_id: String,
    pub target_object_id: String,
    pub relation_type: String,
    pub title: Option<String>,
    pub basis: String,
    pub confidence_level: String,
    pub supporting_material_id: Option<String>,
    pub analyst_comment: Option<String>,
    pub include_in_report: Option<bool>,
}

#[derive(Debug, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct CreateRelationResponse {
    pub relation_item: RelationListItemDto,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct GetRelationsPayload {
    pub case_id: String,
}

#[derive(Debug, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct GetRelationsResponse {
    pub items: Vec<RelationListItemDto>,
}

#[derive(Debug, Clone, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct RelationDetailsDto {
    pub id: String,
    pub case_id: String,
    pub relation_code: String,
    pub source_object: RelationObjectDto,
    pub target_object: RelationObjectDto,
    pub relation_type: String,
    pub title: Option<String>,
    pub basis: String,
    pub confidence_level: String,
    pub supporting_material: Option<RelationMaterialDto>,
    pub analyst_comment: Option<String>,
    pub include_in_report: bool,
    pub created_at: String,
    pub updated_at: String,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct GetRelationByIdPayload {
    pub case_id: String,
    pub relation_id: String,
}

#[derive(Debug, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct GetRelationByIdResponse {
    pub relation: RelationDetailsDto,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct UpdateRelationPayload {
    pub case_id: String,
    pub relation_id: String,
    pub relation_type: String,
    pub title: Option<String>,
    pub basis: String,
    pub confidence_level: String,
    pub supporting_material_id: Option<String>,
    pub analyst_comment: Option<String>,
    pub include_in_report: bool,
}

#[derive(Debug, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct UpdateRelationResponse {
    pub relation: RelationDetailsDto,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct SoftDeleteRelationPayload {
    pub case_id: String,
    pub relation_id: String,
}

#[derive(Debug, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct SoftDeleteRelationResponse {
    pub relation_id: String,
}
