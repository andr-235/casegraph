use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct MaterialDto {
    pub id: String,
    pub case_id: String,
    pub material_code: String,
    pub title: String,
    pub material_type: String,
    pub source_name: String,
    pub description: String,
    pub captured_at: Option<String>,
    pub include_in_report: bool,
    pub original_file_name: Option<String>,
    pub original_path: Option<String>,
    pub stored_file_path: Option<String>,
    pub file_size: Option<i64>,
    pub mime_type: Option<String>,
    pub sha256: Option<String>,
    pub integrity_status: String,
    pub created_by_user_id: String,
    pub created_at: String,
    pub updated_at: String,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct GetMaterialsPayload {
    pub case_id: String,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CreateMaterialPayload {
    pub case_id: String,
    pub title: String,
    pub material_type: String,
    pub source_name: Option<String>,
    pub description: Option<String>,
    pub captured_at: Option<String>,
    pub include_in_report: bool,
    pub source_file_path: Option<String>,
}

#[derive(Debug, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct CreateMaterialResponse {
    pub material: MaterialDto,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct UpdateMaterialPayload {
    pub case_id: String,
    pub material_id: String,
    pub title: String,
    pub material_type: String,
    pub source_name: Option<String>,
    pub description: Option<String>,
    pub captured_at: Option<String>,
    pub include_in_report: bool,
}

#[derive(Debug, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct UpdateMaterialResponse {
    pub material: MaterialDto,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct DeleteMaterialPayload {
    pub case_id: String,
    pub material_id: String,
}

#[derive(Debug, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct DeleteMaterialResponse {
    pub material_id: String,
}
