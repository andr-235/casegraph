use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "snake_case")]
pub enum RestoreOperationPhase {
    Started,
    StagingExtracted,
    StagingValidated,
    RollbackPrepared,
    ReplacingLiveData,
    LiveDataReplaced,
    CompletedRequiresRestart,
    FailedNeedsRecovery,
    RollbackStarted,
    RollbackCompleted,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct RestoreOperationState {
    pub operation_id: String,
    pub phase: RestoreOperationPhase,

    pub restore_backup_id: Option<String>,
    pub restore_backup_code: Option<String>,
    pub restore_archive_sha256: String,

    pub safety_backup_id: String,
    pub safety_backup_code: String,
    pub safety_archive_sha256: String,

    pub started_at: String,
    pub updated_at: String,

    pub last_error_code: Option<String>,
}
