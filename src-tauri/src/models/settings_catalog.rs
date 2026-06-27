use std::collections::{HashMap, HashSet};

// ─── Setting kind / category ────────────────────────────────────────────────

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum SettingKind {
    Bool,
    Str,
    Path,
    Template,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum SettingCategory {
    Docx,
    Backup,
    Integrity,
    Access,
}

// ─── Definition ─────────────────────────────────────────────────────────────

#[derive(Debug, Clone, Copy)]
pub struct SettingDefinition {
    /// Canonical database key (dot-separated snake_case).
    pub key: &'static str,
    pub category: SettingCategory,
    pub kind: SettingKind,
    pub default_value: &'static str,
    /// True for keys whose values are filesystem paths and must be redacted in audit logs.
    pub is_path_like: bool,
}

// ─── Canonical key constants (must match 004_app_settings.sql) ──────────────

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

// ─── Catalog ─────────────────────────────────────────────────────────────────

pub const SETTINGS_CATALOG: &[SettingDefinition] = &[
    SettingDefinition {
        key: KEY_DOCX_DEFAULT_TEMPLATE,
        category: SettingCategory::Docx,
        kind: SettingKind::Template,
        default_value: "analytical-report",
        is_path_like: false,
    },
    SettingDefinition {
        key: KEY_DOCX_DEFAULT_EXPORT_DIR,
        category: SettingCategory::Docx,
        kind: SettingKind::Path,
        default_value: "",
        is_path_like: true,
    },
    SettingDefinition {
        key: KEY_DOCX_INCLUDE_MATERIALS_TABLE,
        category: SettingCategory::Docx,
        kind: SettingKind::Bool,
        default_value: "true",
        is_path_like: false,
    },
    SettingDefinition {
        key: KEY_DOCX_INCLUDE_SHA256_TABLE,
        category: SettingCategory::Docx,
        kind: SettingKind::Bool,
        default_value: "true",
        is_path_like: false,
    },
    SettingDefinition {
        key: KEY_BACKUP_DEFAULT_DIR,
        category: SettingCategory::Backup,
        kind: SettingKind::Path,
        default_value: "",
        is_path_like: true,
    },
    SettingDefinition {
        key: KEY_BACKUP_SAFETY_BEFORE_RESTORE,
        category: SettingCategory::Backup,
        kind: SettingKind::Bool,
        default_value: "true",
        is_path_like: false,
    },
    SettingDefinition {
        key: KEY_BACKUP_VERIFY_AFTER_CREATE,
        category: SettingCategory::Backup,
        kind: SettingKind::Bool,
        default_value: "true",
        is_path_like: false,
    },
    SettingDefinition {
        key: KEY_INTEGRITY_WARN_BEFORE_DOCX,
        category: SettingCategory::Integrity,
        kind: SettingKind::Bool,
        default_value: "true",
        is_path_like: false,
    },
    SettingDefinition {
        key: KEY_INTEGRITY_WARN_BEFORE_BACKUP,
        category: SettingCategory::Integrity,
        kind: SettingKind::Bool,
        default_value: "true",
        is_path_like: false,
    },
    SettingDefinition {
        key: KEY_ACCESS_VIEWER_CAN_EXPORT_DOCX,
        category: SettingCategory::Access,
        kind: SettingKind::Bool,
        default_value: "false",
        is_path_like: false,
    },
    SettingDefinition {
        key: KEY_ACCESS_ANALYST_CAN_CREATE_BACKUP,
        category: SettingCategory::Access,
        kind: SettingKind::Bool,
        default_value: "false",
        is_path_like: false,
    },
];

// ─── Query helpers ────────────────────────────────────────────────────────────

/// Returns the set of all keys that are allowed to be written to app_settings.
pub fn allowed_setting_keys() -> HashSet<&'static str> {
    SETTINGS_CATALOG.iter().map(|d| d.key).collect()
}

/// Returns all catalog defaults as (key, value) string pairs.
pub fn default_setting_pairs() -> Vec<(String, String)> {
    SETTINGS_CATALOG
        .iter()
        .map(|d| (d.key.to_string(), d.default_value.to_string()))
        .collect()
}

/// Returns all catalog defaults as a HashMap for O(1) lookup.
pub fn default_settings_map() -> HashMap<String, String> {
    SETTINGS_CATALOG
        .iter()
        .map(|d| (d.key.to_string(), d.default_value.to_string()))
        .collect()
}

/// Looks up a single definition by key.
pub fn setting_definition(key: &str) -> Option<&'static SettingDefinition> {
    SETTINGS_CATALOG.iter().find(|d| d.key == key)
}

/// Returns only the keys whose values are filesystem paths (used for audit redaction).
pub fn path_like_keys() -> HashSet<&'static str> {
    SETTINGS_CATALOG
        .iter()
        .filter(|d| d.is_path_like)
        .map(|d| d.key)
        .collect()
}

// ─── Catalog consistency tests ────────────────────────────────────────────────

#[cfg(test)]
mod tests {
    use super::*;
    use crate::services::settings_validator::validate_setting_pair;

    #[test]
    fn catalog_keys_are_unique() {
        let mut seen = HashSet::new();
        for item in SETTINGS_CATALOG {
            assert!(seen.insert(item.key), "duplicate setting key: {}", item.key);
        }
    }

    #[test]
    fn every_catalog_key_has_a_default_pair() {
        let allowed = allowed_setting_keys();
        let pairs: HashSet<String> = default_setting_pairs()
            .into_iter()
            .map(|(k, _)| k)
            .collect();
        for key in &allowed {
            assert!(
                pairs.contains(*key),
                "missing default for setting key: {key}"
            );
        }
    }

    #[test]
    fn every_default_value_passes_validation() {
        for (key, value) in default_setting_pairs() {
            validate_setting_pair(&key, &value)
                .unwrap_or_else(|e| panic!("default value invalid for key '{key}': {e:?}"));
        }
    }

    #[test]
    fn path_like_keys_are_a_subset_of_catalog() {
        let allowed = allowed_setting_keys();
        for key in path_like_keys() {
            assert!(allowed.contains(key), "path-like key not in catalog: {key}");
        }
    }
}
