use crate::models::settings_keys::*;

#[derive(Debug, Clone)]
pub struct DefaultSettingPair {
    pub key: &'static str,
    pub value: String,
}

pub fn default_settings_pairs() -> Vec<DefaultSettingPair> {
    vec![
        DefaultSettingPair {
            key: KEY_DOCX_DEFAULT_TEMPLATE,
            value: "analytical-report".to_string(),
        },
        DefaultSettingPair {
            key: KEY_DOCX_DEFAULT_EXPORT_DIR,
            value: "".to_string(),
        },
        DefaultSettingPair {
            key: KEY_DOCX_INCLUDE_MATERIALS_TABLE,
            value: "true".to_string(),
        },
        DefaultSettingPair {
            key: KEY_DOCX_INCLUDE_SHA256_TABLE,
            value: "true".to_string(),
        },
        DefaultSettingPair {
            key: KEY_BACKUP_DEFAULT_DIR,
            value: "".to_string(),
        },
        DefaultSettingPair {
            key: KEY_BACKUP_SAFETY_BEFORE_RESTORE,
            value: "true".to_string(),
        },
        DefaultSettingPair {
            key: KEY_BACKUP_VERIFY_AFTER_CREATE,
            value: "true".to_string(),
        },
        DefaultSettingPair {
            key: KEY_INTEGRITY_WARN_BEFORE_DOCX,
            value: "true".to_string(),
        },
        DefaultSettingPair {
            key: KEY_INTEGRITY_WARN_BEFORE_BACKUP,
            value: "true".to_string(),
        },
        DefaultSettingPair {
            key: KEY_ACCESS_VIEWER_CAN_EXPORT_DOCX,
            value: "false".to_string(),
        },
        DefaultSettingPair {
            key: KEY_ACCESS_ANALYST_CAN_CREATE_BACKUP,
            value: "false".to_string(),
        },
    ]
}
