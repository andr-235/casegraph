use std::path::PathBuf;

#[derive(Debug, Clone)]
pub struct RestoreOperationPaths {
    pub operation_id: String,
    pub staging_dir: PathBuf,
    pub rollback_dir: PathBuf,
    pub restore_lock_file: PathBuf,

    pub live_database_path: PathBuf,
    pub staged_database_path: PathBuf,

    pub live_data_dir: PathBuf,
    pub staged_data_dir: PathBuf,

    pub live_thumbnails_dir: PathBuf,
    pub staged_thumbnails_dir: PathBuf,

    pub live_exports_dir: PathBuf,
    pub staged_exports_dir: PathBuf,

    pub live_templates_dir: PathBuf,
    pub staged_templates_dir: PathBuf,
}

#[derive(Debug, Clone)]
pub struct RestoreSafetyBackupCheck {
    pub backup_id: String,
    pub backup_code: String,
    pub file_path: PathBuf,
    pub archive_sha256: String,
}
