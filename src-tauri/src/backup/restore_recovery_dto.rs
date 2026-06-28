use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct RestoreRecoveryStatusDto {
    pub recovery_required: bool,
    pub operation_id: Option<String>,
    pub phase: Option<String>,
    pub restore_backup_code: Option<String>,
    pub safety_backup_code: Option<String>,
    pub started_at: Option<String>,
    pub updated_at: Option<String>,
    pub last_error_code: Option<String>,
    pub recommended_action: RestoreRecoveryActionDto,
}

#[derive(Debug, Clone, Serialize)]
#[serde(rename_all = "camelCase")]
pub enum RestoreRecoveryActionDto {
    None,
    CleanupCompletedRestore,
    RollbackInterruptedRestore,
    ManualReviewRequired,
}

#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ResolveRestoreRecoveryPayload {
    pub action: String,
    pub confirmation_phrase: String,
}

#[derive(Debug, Clone, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct ResolveRestoreRecoveryResponse {
    pub resolved: bool,
    pub action: String,
    pub requires_restart: bool,
    pub message: String,
}
