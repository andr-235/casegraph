use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct GetGraphDataPayload {
    pub case_id: String,
}

#[derive(Debug, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct GetGraphDataResponse {
    pub nodes: Vec<GraphNodeDto>,
    pub edges: Vec<GraphEdgeDto>,
}

#[derive(Debug, Clone, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct GraphNodeDto {
    pub id: String,
    pub case_id: String,
    pub object_code: String,
    pub object_type: String,
    pub title: String,
    pub value: Option<String>,
    pub is_key: bool,
    pub include_in_report: bool,
}

#[derive(Debug, Clone, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct GraphEdgeDto {
    pub id: String,
    pub case_id: String,
    pub relation_code: String,
    pub source_object_id: String,
    pub target_object_id: String,
    pub relation_type: String,
    pub title: Option<String>,
    pub basis: String,
    pub confidence_level: String,
    pub supporting_material_id: Option<String>,
    pub include_in_report: bool,
}
