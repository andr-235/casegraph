use std::collections::BTreeMap;

use serde::Serialize;

#[derive(Debug, Clone, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct EffectivePermissionsDto {
    pub role: String,
    pub must_change_password: bool,
    pub operations: BTreeMap<String, bool>,
    pub policy_flags: EffectivePolicyFlagsDto,
}

#[derive(Debug, Clone, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct EffectivePolicyFlagsDto {
    pub viewer_can_export_docx: bool,
    pub analyst_can_create_backup: bool,
}
