use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct AppSettingsDto {
    pub docx: DocxSettingsDto,
    pub backup: BackupSettingsDto,
    pub integrity: IntegritySettingsDto,
    pub access: AccessSettingsDto,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct DocxSettingsDto {
    pub default_template: String,
    pub default_export_dir: String,
    pub include_materials_table: bool,
    pub include_sha256_table: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct BackupSettingsDto {
    pub default_backup_dir: String,
    pub safety_backup_before_restore: bool,
    pub verify_backup_after_create: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct IntegritySettingsDto {
    pub warn_before_docx_export: bool,
    pub warn_before_backup: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct AccessSettingsDto {
    pub viewer_can_export_docx: bool,
    pub analyst_can_create_backup: bool,
}

impl Default for AppSettingsDto {
    fn default() -> Self {
        Self {
            docx: DocxSettingsDto {
                default_template: "analytical-report".to_string(),
                default_export_dir: "".to_string(),
                include_materials_table: true,
                include_sha256_table: true,
            },
            backup: BackupSettingsDto {
                default_backup_dir: "".to_string(),
                safety_backup_before_restore: true,
                verify_backup_after_create: true,
            },
            integrity: IntegritySettingsDto {
                warn_before_docx_export: true,
                warn_before_backup: true,
            },
            access: AccessSettingsDto {
                viewer_can_export_docx: false,
                analyst_can_create_backup: false,
            },
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct UpdateSettingsPayload {
    pub docx: UpdateDocxSettingsPayload,
    pub backup: UpdateBackupSettingsPayload,
    pub integrity: UpdateIntegritySettingsPayload,
    pub access: UpdateAccessSettingsPayload,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct UpdateDocxSettingsPayload {
    pub default_template: String,
    pub default_export_dir: String,
    pub include_materials_table: bool,
    pub include_sha256_table: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct UpdateBackupSettingsPayload {
    pub default_backup_dir: String,
    pub safety_backup_before_restore: bool,
    pub verify_backup_after_create: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct UpdateIntegritySettingsPayload {
    pub warn_before_docx_export: bool,
    pub warn_before_backup: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct UpdateAccessSettingsPayload {
    pub viewer_can_export_docx: bool,
    pub analyst_can_create_backup: bool,
}
