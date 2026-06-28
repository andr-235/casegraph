// Legacy wrappers — delegate to security::ProtectedServiceContext
//
// These functions are kept for backward compatibility during the migration.
// New code should call `ProtectedServiceContext::require_operation(app, op)` directly.

use tauri::AppHandle;

use crate::errors::app_error::AppErrorDto;
use crate::security::protected_operation::ProtectedOperation;
use crate::security::session::SessionState;
use crate::security::ProtectedServiceContext;

#[deprecated(note = "Use ProtectedServiceContext::require_operation(app, ProtectedOperation::...)")]
pub fn require_protected_user(
    app: &AppHandle,
    _session: &SessionState,
) -> Result<ProtectedServiceContext, AppErrorDto> {
    ProtectedServiceContext::require_operation(app, ProtectedOperation::CaseRead)
}

#[deprecated(note = "Use ProtectedServiceContext::require_operation(app, ProtectedOperation::...)")]
pub fn require_protected_user_for(
    app: &AppHandle,
    _session: &SessionState,
    _action: &str,
) -> Result<ProtectedServiceContext, AppErrorDto> {
    ProtectedServiceContext::require_operation(app, ProtectedOperation::CaseRead)
}

#[deprecated(note = "Use ProtectedServiceContext::require_operation(app, ProtectedOperation::...)")]
pub fn require_protected_administrator(
    app: &AppHandle,
    _session: &SessionState,
) -> Result<ProtectedServiceContext, AppErrorDto> {
    ProtectedServiceContext::require_operation(app, ProtectedOperation::UserManage)
}

#[deprecated(note = "Use ProtectedServiceContext::require_operation(app, ProtectedOperation::...)")]
pub fn require_protected_administrator_for(
    app: &AppHandle,
    _session: &SessionState,
    _action: &str,
) -> Result<ProtectedServiceContext, AppErrorDto> {
    ProtectedServiceContext::require_operation(app, ProtectedOperation::UserManage)
}

#[deprecated(note = "Use ProtectedServiceContext::require_operation(app, ProtectedOperation::...)")]
pub fn require_protected_analyst_or_admin(
    app: &AppHandle,
    _session: &SessionState,
) -> Result<ProtectedServiceContext, AppErrorDto> {
    ProtectedServiceContext::require_operation(app, ProtectedOperation::CaseRead)
}

#[deprecated(note = "Use ProtectedServiceContext::require_operation(app, ProtectedOperation::...)")]
pub fn require_protected_analyst_or_admin_for(
    app: &AppHandle,
    _session: &SessionState,
    _action: &str,
) -> Result<ProtectedServiceContext, AppErrorDto> {
    ProtectedServiceContext::require_operation(app, ProtectedOperation::CaseRead)
}
