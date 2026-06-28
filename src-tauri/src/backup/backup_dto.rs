use serde::Serialize;

#[derive(Debug, Clone, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct BackupHistoryItemDto {
    pub id: String,
    pub backup_code: String,
    pub backup_type: String,
    pub status: String,
    pub file_name: String,
    pub file_size: Option<i64>,
    pub sha256: Option<String>,
    pub case_id: Option<String>,
    pub case_code: Option<String>,
    pub app_version: String,
    pub schema_version: i64,
    pub created_by: String,
    pub created_at: String,
    pub verified_at: Option<String>,
    pub restored_at: Option<String>,
}
