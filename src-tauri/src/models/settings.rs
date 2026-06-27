use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct AppSettingsDto {
    pub storage_path: Option<String>,
    pub docx_default_template: String,
    pub backup_default_path: Option<String>,
    pub integrity_check_on_startup: bool,
    pub viewer_can_export_docx: bool,
    pub analyst_can_create_backup: bool,
    pub audit_strict_mode: bool,
}

impl Default for AppSettingsDto {
    fn default() -> Self {
        Self {
            storage_path: None,
            docx_default_template: "analytical-report".to_string(),
            backup_default_path: None,
            integrity_check_on_startup: false,
            viewer_can_export_docx: false,
            analyst_can_create_backup: false,
            audit_strict_mode: true,
        }
    }
}
