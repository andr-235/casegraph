use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CreateBackupPayload {
    pub backup_type: BackupCreateType,
    pub output_folder_path: String,
    pub include_exports: bool,
    pub include_audit_log: bool,
    pub include_templates: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "snake_case")]
pub enum BackupCreateType {
    Full,
}

#[derive(Debug, Clone, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct CreateBackupResponse {
    pub backup_id: String,
    pub backup_code: String,
    pub file_name: String,
    pub file_size: i64,
    pub sha256: String,
    pub created_at: String,
}

#[derive(Debug, Clone, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct SelectBackupOutputFolderResponse {
    pub folder_path: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct BackupMetadataDto {
    pub backup_id: String,
    pub backup_code: String,
    pub backup_type: String,
    pub created_at: String,
    pub created_by_user_id: String,
    pub created_by_username: String,
    pub created_by_role: String,
    pub app_version: String,
    pub schema_version: i64,
    pub archive_sha256: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct BackupManifestItemDto {
    pub path: String,
    pub kind: String,
    pub required: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct BackupManifestDto {
    pub files: Vec<BackupManifestItemDto>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct BackupChecksumItemDto {
    pub path: String,
    pub sha256: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct BackupChecksumsDto {
    pub algorithm: String,
    pub items: Vec<BackupChecksumItemDto>,
}

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
