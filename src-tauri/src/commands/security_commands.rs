use tauri::AppHandle;

use crate::errors::app_error::{AppErrorDto, CommandResult};
use crate::security::effective_permission_service::EffectivePermissionService;
use crate::security::effective_permissions::EffectivePermissionsDto;
use crate::security::ProtectedServiceContext;

#[tauri::command]
pub fn get_effective_permissions(app: AppHandle) -> CommandResult<EffectivePermissionsDto> {
    match (|| -> Result<EffectivePermissionsDto, AppErrorDto> {
        let ctx = ProtectedServiceContext::require_authenticated(&app)?;

        let must_change_password = ctx.current_user.must_change_password;

        EffectivePermissionService::get_effective_permissions(
            &ctx.conn,
            &ctx.current_user,
            must_change_password,
        )
    })() {
        Ok(data) => CommandResult::ok(data),
        Err(error) => CommandResult::err(error),
    }
}
