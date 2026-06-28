use std::collections::BTreeMap;

use rusqlite::Connection;

use crate::errors::app_error::AppErrorDto;
use crate::security::effective_permissions::{EffectivePermissionsDto, EffectivePolicyFlagsDto};
use crate::security::permission_decision::PermissionDecision;
use crate::security::policy_aware_permission_service::PolicyAwarePermissionService;
use crate::security::protected_operation::ProtectedOperation;
use crate::security::session::CurrentUserDto;
use crate::services::settings_access_policy::SettingsAccessPolicy;

pub struct EffectivePermissionService;

impl EffectivePermissionService {
    pub fn get_effective_permissions(
        conn: &Connection,
        current_user: &CurrentUserDto,
        must_change_password: bool,
    ) -> Result<EffectivePermissionsDto, AppErrorDto> {
        let access_policy = SettingsAccessPolicy::from_connection(conn)?;

        let mut operations = BTreeMap::new();

        for operation in ProtectedOperation::all() {
            let allowed = if must_change_password {
                false
            } else {
                matches!(
                    PolicyAwarePermissionService::decide(conn, current_user, *operation),
                    PermissionDecision::Allow
                )
            };

            operations.insert(operation.key().to_string(), allowed);
        }

        Ok(EffectivePermissionsDto {
            role: current_user.role.clone(),
            must_change_password,
            operations,
            policy_flags: EffectivePolicyFlagsDto {
                viewer_can_export_docx: access_policy.viewer_can_export_docx,
                analyst_can_create_backup: access_policy.analyst_can_create_backup,
            },
        })
    }
}
