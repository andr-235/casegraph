// Canonical string keys for the app_settings table.
// Imported by both settings_service and settings_defaults.

pub const KEY_DOCX_DEFAULT_TEMPLATE: &str = "docx.default_template";
pub const KEY_DOCX_DEFAULT_EXPORT_DIR: &str = "docx.default_export_dir";
pub const KEY_DOCX_INCLUDE_MATERIALS_TABLE: &str = "docx.include_materials_table";
pub const KEY_DOCX_INCLUDE_SHA256_TABLE: &str = "docx.include_sha256_table";

pub const KEY_BACKUP_DEFAULT_DIR: &str = "backup.default_dir";
pub const KEY_BACKUP_SAFETY_BEFORE_RESTORE: &str = "backup.safety_before_restore";
pub const KEY_BACKUP_VERIFY_AFTER_CREATE: &str = "backup.verify_after_create";

pub const KEY_INTEGRITY_WARN_BEFORE_DOCX: &str = "integrity.warn_before_docx_export";
pub const KEY_INTEGRITY_WARN_BEFORE_BACKUP: &str = "integrity.warn_before_backup";

pub const KEY_ACCESS_VIEWER_CAN_EXPORT_DOCX: &str = "access.viewer_can_export_docx";
pub const KEY_ACCESS_ANALYST_CAN_CREATE_BACKUP: &str = "access.analyst_can_create_backup";
