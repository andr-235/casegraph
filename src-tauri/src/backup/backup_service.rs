use tauri::AppHandle;

use crate::backup::{BackupHistoryItemDto, BackupRepository};
use crate::errors::app_error::AppErrorDto;
use crate::security::{ProtectedOperation, ProtectedServiceContext};

pub struct BackupService;

impl BackupService {
    pub fn get_backup_history(app: &AppHandle) -> Result<Vec<BackupHistoryItemDto>, AppErrorDto> {
        let ctx = ProtectedServiceContext::require_operation(app, ProtectedOperation::BackupRead)?;

        BackupRepository::list_history(&ctx.conn, 100)
    }
}
