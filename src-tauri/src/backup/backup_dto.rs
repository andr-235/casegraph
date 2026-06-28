use serde::{Deserialize, Serialize};

use crate::backup::backup_archive_reader::BackupVerificationSummaryDto;

pub use crate::backup::backup_archive_reader::{
    BackupVerificationIssueDto, BackupVerificationIssueSeverity,
};

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

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct SelectBackupFileResponse {
    pub file_path: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct VerifyBackupPayload {
    pub backup_id: Option<String>,
    pub file_path: Option<String>,
}

#[derive(Debug, Clone, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct VerifyBackupResponse {
    pub backup_id: Option<String>,
    pub backup_code: Option<String>,
    pub file_name: String,
    pub archive_sha256: String,
    pub checked_at: String,
    pub is_valid: bool,
    pub summary: BackupVerificationSummaryDto,
    pub issues: Vec<BackupVerificationIssueDto>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct SelectRestoreBackupFileResponse {
    pub file_path: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct RestoreBackupPreflightPayload {
    pub backup_id: Option<String>,
    pub file_path: Option<String>,
}

#[derive(Debug, Clone, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct RestoreBackupPreflightResponse {
    pub backup_id: Option<String>,
    pub backup_code: Option<String>,
    pub file_name: String,
    pub archive_sha256: String,
    pub checked_at: String,
    pub can_restore: bool,
    pub requires_safety_backup: bool,
    pub metadata: RestoreBackupMetadataPreviewDto,
    pub compatibility: RestoreCompatibilityDto,
    pub verification: BackupVerificationSummaryDto,
    pub warnings: Vec<RestorePreflightIssueDto>,
    pub errors: Vec<RestorePreflightIssueDto>,
}

#[derive(Debug, Clone, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct RestoreBackupMetadataPreviewDto {
    pub backup_type: String,
    pub app_version: String,
    pub schema_version: i64,
    pub created_at: String,
    pub created_by: Option<String>,
    pub case_id: Option<String>,
    pub case_code: Option<String>,
    pub file_count: usize,
}

#[derive(Debug, Clone, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct RestoreCompatibilityDto {
    pub app_version_ok: bool,
    pub schema_version_ok: bool,
    pub backup_type_ok: bool,
    pub current_app_version: String,
    pub backup_app_version: String,
    pub current_schema_version: i64,
    pub backup_schema_version: i64,
}

#[derive(Debug, Clone, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct RestorePreflightIssueDto {
    pub code: String,
    pub message: String,
    pub severity: RestorePreflightIssueSeverity,
}

#[derive(Debug, Clone, Serialize)]
#[serde(rename_all = "snake_case")]
pub enum RestorePreflightIssueSeverity {
    Warning,
    Error,
}
