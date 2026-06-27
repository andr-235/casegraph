use std::collections::HashMap;

use serde_json::Value;
use tauri::AppHandle;

use tauri_plugin_dialog::DialogExt;

use crate::audit::audit_safe_value::AuditSafeDetails;
use crate::errors::app_error::AppErrorDto;
use crate::models::settings::{
    AppSettingsDto, ChooseSettingsDirectoryPayload, ChooseSettingsDirectoryResponse,
    UpdateSettingsPayload,
};
use crate::models::settings_catalog::{setting_definition, SettingCategory};
use crate::models::settings_keys::*;
use crate::security::session::SessionState;
use crate::services::protected_service_context::require_protected_administrator_for;
use crate::services::settings_validator::{validate_path, validate_setting_pair};

pub struct SettingsService;

impl SettingsService {
    // ─── get_settings ────────────────────────────────────────────────────────

    pub fn get_settings(
        app: &AppHandle,
        session: &SessionState,
    ) -> Result<AppSettingsDto, AppErrorDto> {
        let context = require_protected_administrator_for(app, session, "settings.get")?;
        let conn = crate::db::connection::open_connection(app)?;
        let settings =
            crate::repositories::settings_repository::SettingsRepository::get_settings(&conn)?;
        drop(context);
        Ok(settings)
    }

    // ─── update_settings ──────────────────────────────────────────────────────

    pub fn update_settings(
        app: &AppHandle,
        session: &SessionState,
        payload: UpdateSettingsPayload,
    ) -> Result<AppSettingsDto, AppErrorDto> {
        let mut context = require_protected_administrator_for(app, session, "update_settings")?;

        let conn = &mut context.conn;

        // Build pairs and run catalog-driven validation in one pass
        let pairs = payload_to_settings_pairs(&payload)?;

        let old_settings =
            crate::repositories::settings_repository::SettingsRepository::get_settings(conn)?;

        crate::repositories::settings_repository::SettingsRepository::upsert_many(conn, &pairs)?;

        let new_settings =
            crate::repositories::settings_repository::SettingsRepository::get_settings(conn)?;

        // ── Audit ──
        audit_settings_change(
            app,
            &context.current_user,
            &old_settings,
            &new_settings,
            crate::domain::audit_action::settings::UPDATED,
            |keys| crate::audit::audit_metadata::settings_updated(keys, &categories_for_keys(keys)),
        );

        Ok(new_settings)
    }

    // ─── reset_settings_to_defaults ───────────────────────────────────────────

    pub fn reset_settings_to_defaults(
        app: &AppHandle,
        session: &SessionState,
    ) -> Result<AppSettingsDto, AppErrorDto> {
        let mut context =
            require_protected_administrator_for(app, session, "reset_settings_to_defaults")?;
        let conn = &mut context.conn;

        let old_settings =
            crate::repositories::settings_repository::SettingsRepository::get_settings(conn)?;

        // Catalog-driven reset — no separate defaults list to drift
        crate::repositories::settings_repository::SettingsRepository::reset_to_defaults(conn)?;

        let new_settings =
            crate::repositories::settings_repository::SettingsRepository::get_settings(conn)?;

        // ── Audit ──
        audit_settings_change(
            app,
            &context.current_user,
            &old_settings,
            &new_settings,
            crate::domain::audit_action::settings::RESET_TO_DEFAULT,
            |keys| crate::audit::audit_metadata::settings_reset_to_default(keys),
        );

        Ok(new_settings)
    }

    // ─── choose_settings_directory ────────────────────────────────────────────

    pub fn choose_settings_directory(
        app: &AppHandle,
        session: &SessionState,
        _payload: ChooseSettingsDirectoryPayload,
    ) -> Result<ChooseSettingsDirectoryResponse, AppErrorDto> {
        let _context =
            require_protected_administrator_for(app, session, "choose_settings_directory")?;

        let selected_path = pick_folder(app)?;

        // Use the shared validate_path — same rules as update_settings path validation
        if let Some(path) = selected_path.as_deref() {
            validate_path(path)?;
        }

        Ok(ChooseSettingsDirectoryResponse {
            path: selected_path,
        })
    }
}

// ─── Payload → pairs (catalog-driven) ────────────────────────────────────────

fn payload_to_settings_pairs(
    payload: &UpdateSettingsPayload,
) -> Result<Vec<(String, String)>, AppErrorDto> {
    let pairs = vec![
        (
            KEY_DOCX_DEFAULT_TEMPLATE.to_string(),
            payload.docx.default_template.clone(),
        ),
        (
            KEY_DOCX_DEFAULT_EXPORT_DIR.to_string(),
            payload.docx.default_export_dir.clone(),
        ),
        (
            KEY_DOCX_INCLUDE_MATERIALS_TABLE.to_string(),
            bool_str(payload.docx.include_materials_table),
        ),
        (
            KEY_DOCX_INCLUDE_SHA256_TABLE.to_string(),
            bool_str(payload.docx.include_sha256_table),
        ),
        (
            KEY_BACKUP_DEFAULT_DIR.to_string(),
            payload.backup.default_backup_dir.clone(),
        ),
        (
            KEY_BACKUP_SAFETY_BEFORE_RESTORE.to_string(),
            bool_str(payload.backup.safety_backup_before_restore),
        ),
        (
            KEY_BACKUP_VERIFY_AFTER_CREATE.to_string(),
            bool_str(payload.backup.verify_backup_after_create),
        ),
        (
            KEY_INTEGRITY_WARN_BEFORE_DOCX.to_string(),
            bool_str(payload.integrity.warn_before_docx_export),
        ),
        (
            KEY_INTEGRITY_WARN_BEFORE_BACKUP.to_string(),
            bool_str(payload.integrity.warn_before_backup),
        ),
        (
            KEY_ACCESS_VIEWER_CAN_EXPORT_DOCX.to_string(),
            bool_str(payload.access.viewer_can_export_docx),
        ),
        (
            KEY_ACCESS_ANALYST_CAN_CREATE_BACKUP.to_string(),
            bool_str(payload.access.analyst_can_create_backup),
        ),
    ];

    // Validate every pair through the catalog — single validation site
    for (key, value) in &pairs {
        validate_setting_pair(key, value)?;
    }

    Ok(pairs)
}

// ─── Audit helper ─────────────────────────────────────────────────────────────

fn audit_settings_change<F>(
    app: &AppHandle,
    current_user: &crate::security::session::CurrentUserDto,
    old_settings: &AppSettingsDto,
    new_settings: &AppSettingsDto,
    action: &'static str,
    make_details: F,
) where
    F: Fn(&[String]) -> Result<AuditSafeDetails, AppErrorDto>,
{
    let old_map = settings_to_value_map(old_settings);
    let new_map = settings_to_value_map(new_settings);

    let mut changes = Vec::new();
    let mut changed_keys = Vec::new();

    for (key, new_value) in new_map.iter() {
        let old_value = old_map.get(key);
        if old_value != Some(new_value) {
            let category = settings_category_label(key);
            changes.push(crate::audit::audit_metadata::setting_change_snapshot(
                key,
                category,
                old_value.unwrap_or(&Value::Null),
                new_value,
            ));
            changed_keys.push(key.clone());
        }
    }

    if changed_keys.is_empty() {
        return;
    }

    let build = || -> Result<(), AppErrorDto> {
        let old_snaps: Vec<_> = changes
            .iter()
            .map(|c| crate::audit::audit_metadata::SettingsChangeSnapshot {
                key: c.key.clone(),
                category: c.category.clone(),
                old_value: c.old_value.clone(),
                new_value: Value::Null,
            })
            .collect();

        let new_snaps: Vec<_> = changes
            .iter()
            .map(|c| crate::audit::audit_metadata::SettingsChangeSnapshot {
                key: c.key.clone(),
                category: c.category.clone(),
                old_value: Value::Null,
                new_value: c.new_value.clone(),
            })
            .collect();

        let old_snapshot = crate::audit::audit_metadata::safe_settings_snapshot(old_snaps)?;
        let new_snapshot = crate::audit::audit_metadata::safe_settings_snapshot(new_snaps)?;
        let details = make_details(&changed_keys)?;

        crate::audit::audit_service::AuditService::write_best_effort(
            app,
            crate::audit::audit_service::AuditWriteInput::success(current_user, action)
                .with_entity_type("settings")
                .with_snapshots(Some(old_snapshot), Some(new_snapshot))
                .with_details(details),
        );

        Ok(())
    };

    // Audit is best-effort — log and swallow
    if let Err(e) = build() {
        eprintln!("[audit] settings audit failed: {e:?}");
    }
}

// ─── Small utilities ─────────────────────────────────────────────────────────

fn bool_str(value: bool) -> String {
    if value { "true" } else { "false" }.to_string()
}

fn settings_to_value_map(s: &AppSettingsDto) -> HashMap<String, Value> {
    let mut map = HashMap::new();
    map.insert(
        KEY_DOCX_DEFAULT_TEMPLATE.to_string(),
        Value::String(s.docx.default_template.clone()),
    );
    map.insert(
        KEY_DOCX_DEFAULT_EXPORT_DIR.to_string(),
        Value::String(s.docx.default_export_dir.clone()),
    );
    map.insert(
        KEY_DOCX_INCLUDE_MATERIALS_TABLE.to_string(),
        Value::Bool(s.docx.include_materials_table),
    );
    map.insert(
        KEY_DOCX_INCLUDE_SHA256_TABLE.to_string(),
        Value::Bool(s.docx.include_sha256_table),
    );
    map.insert(
        KEY_BACKUP_DEFAULT_DIR.to_string(),
        Value::String(s.backup.default_backup_dir.clone()),
    );
    map.insert(
        KEY_BACKUP_SAFETY_BEFORE_RESTORE.to_string(),
        Value::Bool(s.backup.safety_backup_before_restore),
    );
    map.insert(
        KEY_BACKUP_VERIFY_AFTER_CREATE.to_string(),
        Value::Bool(s.backup.verify_backup_after_create),
    );
    map.insert(
        KEY_INTEGRITY_WARN_BEFORE_DOCX.to_string(),
        Value::Bool(s.integrity.warn_before_docx_export),
    );
    map.insert(
        KEY_INTEGRITY_WARN_BEFORE_BACKUP.to_string(),
        Value::Bool(s.integrity.warn_before_backup),
    );
    map.insert(
        KEY_ACCESS_VIEWER_CAN_EXPORT_DOCX.to_string(),
        Value::Bool(s.access.viewer_can_export_docx),
    );
    map.insert(
        KEY_ACCESS_ANALYST_CAN_CREATE_BACKUP.to_string(),
        Value::Bool(s.access.analyst_can_create_backup),
    );
    map
}

/// Returns the human-readable category label for a setting key, driven by the catalog.
fn settings_category_label(key: &str) -> &'static str {
    match setting_definition(key).map(|d| d.category) {
        Some(SettingCategory::Docx) => "docx",
        Some(SettingCategory::Backup) => "backup",
        Some(SettingCategory::Integrity) => "integrity",
        Some(SettingCategory::Access) => "access",
        None => "general",
    }
}

/// Collect unique category labels for a list of changed keys.
fn categories_for_keys(keys: &[String]) -> Vec<String> {
    let mut seen = std::collections::HashSet::new();
    let mut cats = Vec::new();
    for key in keys {
        let cat = settings_category_label(key);
        if seen.insert(cat) {
            cats.push(cat.to_string());
        }
    }
    cats
}

fn pick_folder(app: &AppHandle) -> Result<Option<String>, AppErrorDto> {
    let selected = app
        .dialog()
        .file()
        .set_title("Выберите папку")
        .blocking_pick_folder();

    let path = selected.map(|file_path| match file_path {
        tauri_plugin_dialog::FilePath::Path(p) => p.to_string_lossy().to_string(),
        tauri_plugin_dialog::FilePath::Url(u) => {
            if let Ok(p) = u.to_file_path() {
                p.to_string_lossy().to_string()
            } else {
                u.path().to_string()
            }
        }
    });

    Ok(path)
}
