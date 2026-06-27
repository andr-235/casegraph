use std::collections::HashMap;

use rusqlite::Connection;
use serde_json::Value;
use tauri::AppHandle;

use crate::errors::app_error::AppErrorDto;
use crate::models::settings::{AppSettingsDto, UpdateSettingsPayload};
use crate::security::session::SessionState;
use crate::services::protected_service_context::{
    require_protected_administrator_for, require_protected_user_for,
};

const KEY_DOCX_DEFAULT_TEMPLATE: &str = "docx.default_template";
const KEY_DOCX_DEFAULT_EXPORT_DIR: &str = "docx.default_export_dir";
const KEY_DOCX_INCLUDE_MATERIALS_TABLE: &str = "docx.include_materials_table";
const KEY_DOCX_INCLUDE_SHA256_TABLE: &str = "docx.include_sha256_table";

const KEY_BACKUP_DEFAULT_DIR: &str = "backup.default_dir";
const KEY_BACKUP_SAFETY_BEFORE_RESTORE: &str = "backup.safety_before_restore";
const KEY_BACKUP_VERIFY_AFTER_CREATE: &str = "backup.verify_after_create";

const KEY_INTEGRITY_WARN_BEFORE_DOCX: &str = "integrity.warn_before_docx_export";
const KEY_INTEGRITY_WARN_BEFORE_BACKUP: &str = "integrity.warn_before_backup";

const KEY_ACCESS_VIEWER_CAN_EXPORT_DOCX: &str = "access.viewer_can_export_docx";
const KEY_ACCESS_ANALYST_CAN_CREATE_BACKUP: &str = "access.analyst_can_create_backup";

pub struct SettingsRepository;

impl SettingsRepository {
    pub fn get_settings_map(_conn: &Connection) -> Result<HashMap<String, Value>, AppErrorDto> {
        let mut map = HashMap::new();
        map.insert("viewer_can_export_docx".to_string(), Value::Bool(false));
        map.insert(
            "storage_path".to_string(),
            Value::String("C:\\path".to_string()),
        );
        map.insert(
            "api_token".to_string(),
            Value::String("secret-token".to_string()),
        );
        Ok(map)
    }

    pub fn reset_to_default(_conn: &Connection) -> Result<(), AppErrorDto> {
        Ok(())
    }

    pub fn update_setting(
        _conn: &Connection,
        _key: &str,
        _value: &Value,
    ) -> Result<(), AppErrorDto> {
        Ok(())
    }
}

pub struct UpdateSettingsPayloadStub {
    pub settings: HashMap<String, Value>,
}

pub struct SettingsService;

impl SettingsService {
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

    pub fn update_settings(
        app: &AppHandle,
        session: &SessionState,
        payload: UpdateSettingsPayload,
    ) -> Result<AppSettingsDto, AppErrorDto> {
        let mut context = require_protected_administrator_for(app, session, "update_settings")?;

        validate_update_payload(&payload)?;

        let conn = &mut context.conn;

        let old_settings =
            crate::repositories::settings_repository::SettingsRepository::get_settings(conn)?;

        let pairs = payload_to_settings_pairs(&payload);

        crate::repositories::settings_repository::SettingsRepository::upsert_many(conn, &pairs)?;

        let new_settings =
            crate::repositories::settings_repository::SettingsRepository::get_settings(conn)?;

        // Audit logging
        let old_map = settings_to_map(&old_settings);
        let new_map = settings_to_map(&new_settings);

        let mut changes = Vec::new();
        let mut changed_keys = Vec::new();
        let mut categories = Vec::new();

        for (key, new_value) in new_map.iter() {
            let old_value = old_map.get(key);

            if old_value != Some(new_value) {
                let category = settings_category_for_key(key);

                changes.push(crate::audit::audit_metadata::setting_change_snapshot(
                    key,
                    category,
                    old_value.unwrap_or(&Value::Null),
                    new_value,
                ));

                changed_keys.push(key.clone());

                if !categories.iter().any(|item| item == category) {
                    categories.push(category.to_string());
                }
            }
        }

        if !changed_keys.is_empty() {
            let old_snapshot_changes: Vec<_> = changes
                .iter()
                .map(
                    |change| crate::audit::audit_metadata::SettingsChangeSnapshot {
                        key: change.key.clone(),
                        category: change.category.clone(),
                        old_value: change.old_value.clone(),
                        new_value: Value::Null,
                    },
                )
                .collect();

            let new_snapshot_changes: Vec<_> = changes
                .iter()
                .map(
                    |change| crate::audit::audit_metadata::SettingsChangeSnapshot {
                        key: change.key.clone(),
                        category: change.category.clone(),
                        old_value: Value::Null,
                        new_value: change.new_value.clone(),
                    },
                )
                .collect();

            let old_value =
                crate::audit::audit_metadata::safe_settings_snapshot(old_snapshot_changes)?;

            let new_value =
                crate::audit::audit_metadata::safe_settings_snapshot(new_snapshot_changes)?;

            let technical_details =
                crate::audit::audit_metadata::settings_updated(&changed_keys, &categories)?;

            crate::audit::audit_service::AuditService::write_best_effort(
                app,
                crate::audit::audit_service::AuditWriteInput::success(
                    &context.current_user,
                    crate::domain::audit_action::settings::UPDATED,
                )
                .with_entity_type("settings")
                .with_snapshots(Some(old_value), Some(new_value))
                .with_details(technical_details),
            );
        }

        Ok(new_settings)
    }

    pub fn update_settings_stub(
        app: &AppHandle,
        session: &SessionState,
        payload: UpdateSettingsPayloadStub,
    ) -> Result<(), AppErrorDto> {
        let context = require_protected_user_for(app, session, "UPDATE_SETTINGS")?;
        let conn = &context.conn;

        let old_settings = SettingsRepository::get_settings_map(conn)?;

        // Update database setting values
        for (key, val) in payload.settings.iter() {
            SettingsRepository::update_setting(conn, key, val)?;
        }

        let updated_settings = SettingsRepository::get_settings_map(conn)?;

        let mut changes = Vec::new();
        let mut changed_keys = Vec::new();
        let mut categories = Vec::new();

        for (key, new_value) in updated_settings.iter() {
            let old_value = old_settings.get(key);

            if old_value != Some(new_value) {
                let category = settings_category_for_key(key);

                changes.push(crate::audit::audit_metadata::setting_change_snapshot(
                    key,
                    category,
                    old_value.unwrap_or(&Value::Null),
                    new_value,
                ));

                changed_keys.push(key.clone());

                if !categories.iter().any(|item| item == category) {
                    categories.push(category.to_string());
                }
            }
        }

        if !changed_keys.is_empty() {
            let old_snapshot_changes: Vec<_> = changes
                .iter()
                .map(
                    |change| crate::audit::audit_metadata::SettingsChangeSnapshot {
                        key: change.key.clone(),
                        category: change.category.clone(),
                        old_value: change.old_value.clone(),
                        new_value: Value::Null,
                    },
                )
                .collect();

            let new_snapshot_changes: Vec<_> = changes
                .iter()
                .map(
                    |change| crate::audit::audit_metadata::SettingsChangeSnapshot {
                        key: change.key.clone(),
                        category: change.category.clone(),
                        old_value: Value::Null,
                        new_value: change.new_value.clone(),
                    },
                )
                .collect();

            let old_value =
                crate::audit::audit_metadata::safe_settings_snapshot(old_snapshot_changes)?;

            let new_value =
                crate::audit::audit_metadata::safe_settings_snapshot(new_snapshot_changes)?;

            let technical_details =
                crate::audit::audit_metadata::settings_updated(&changed_keys, &categories)?;

            crate::audit::audit_service::AuditService::write_best_effort(
                app,
                crate::audit::audit_service::AuditWriteInput::success(
                    &context.current_user,
                    crate::domain::audit_action::settings::UPDATED,
                )
                .with_entity_type("settings")
                .with_snapshots(Some(old_value), Some(new_value))
                .with_details(technical_details),
            );
        }

        Ok(())
    }

    pub fn reset_settings_to_default(
        app: &AppHandle,
        session: &SessionState,
    ) -> Result<(), AppErrorDto> {
        let context = require_protected_user_for(app, session, "RESET_SETTINGS")?;
        let conn = &context.conn;

        let old_settings = SettingsRepository::get_settings_map(conn)?;

        SettingsRepository::reset_to_default(conn)?;

        let updated_settings = SettingsRepository::get_settings_map(conn)?;

        let mut changes = Vec::new();
        let mut changed_keys = Vec::new();
        let mut categories = Vec::new();

        for (key, new_value) in updated_settings.iter() {
            let old_value = old_settings.get(key);

            if old_value != Some(new_value) {
                let category = settings_category_for_key(key);

                changes.push(crate::audit::audit_metadata::setting_change_snapshot(
                    key,
                    category,
                    old_value.unwrap_or(&Value::Null),
                    new_value,
                ));

                changed_keys.push(key.clone());

                if !categories.iter().any(|item| item == category) {
                    categories.push(category.to_string());
                }
            }
        }

        if !changed_keys.is_empty() {
            let old_snapshot_changes: Vec<_> = changes
                .iter()
                .map(
                    |change| crate::audit::audit_metadata::SettingsChangeSnapshot {
                        key: change.key.clone(),
                        category: change.category.clone(),
                        old_value: change.old_value.clone(),
                        new_value: Value::Null,
                    },
                )
                .collect();

            let new_snapshot_changes: Vec<_> = changes
                .iter()
                .map(
                    |change| crate::audit::audit_metadata::SettingsChangeSnapshot {
                        key: change.key.clone(),
                        category: change.category.clone(),
                        old_value: Value::Null,
                        new_value: change.new_value.clone(),
                    },
                )
                .collect();

            let old_value =
                crate::audit::audit_metadata::safe_settings_snapshot(old_snapshot_changes)?;

            let new_value =
                crate::audit::audit_metadata::safe_settings_snapshot(new_snapshot_changes)?;

            let technical_details =
                crate::audit::audit_metadata::settings_reset_to_default(&changed_keys)?;

            crate::audit::audit_service::AuditService::write_best_effort(
                app,
                crate::audit::audit_service::AuditWriteInput::success(
                    &context.current_user,
                    crate::domain::audit_action::settings::RESET_TO_DEFAULT,
                )
                .with_entity_type("settings")
                .with_snapshots(Some(old_value), Some(new_value))
                .with_details(technical_details),
            );
        }

        Ok(())
    }
}

fn settings_category_for_key(key: &str) -> &'static str {
    match key {
        "access.viewer_can_export_docx" | "access.analyst_can_create_backup" => "access",
        "docx.default_template"
        | "docx.default_export_dir"
        | "docx.include_materials_table"
        | "docx.include_sha256_table" => "docx",
        "backup.default_dir" | "backup.safety_before_restore" | "backup.verify_after_create" => {
            "backup"
        }
        "integrity.warn_before_docx_export" | "integrity.warn_before_backup" => "integrity",
        _ => "general",
    }
}

fn bool_value(value: bool) -> String {
    if value { "true" } else { "false" }.to_string()
}

fn payload_to_settings_pairs(payload: &UpdateSettingsPayload) -> Vec<(String, String)> {
    vec![
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
            bool_value(payload.docx.include_materials_table),
        ),
        (
            KEY_DOCX_INCLUDE_SHA256_TABLE.to_string(),
            bool_value(payload.docx.include_sha256_table),
        ),
        (
            KEY_BACKUP_DEFAULT_DIR.to_string(),
            payload.backup.default_backup_dir.clone(),
        ),
        (
            KEY_BACKUP_SAFETY_BEFORE_RESTORE.to_string(),
            bool_value(payload.backup.safety_backup_before_restore),
        ),
        (
            KEY_BACKUP_VERIFY_AFTER_CREATE.to_string(),
            bool_value(payload.backup.verify_backup_after_create),
        ),
        (
            KEY_INTEGRITY_WARN_BEFORE_DOCX.to_string(),
            bool_value(payload.integrity.warn_before_docx_export),
        ),
        (
            KEY_INTEGRITY_WARN_BEFORE_BACKUP.to_string(),
            bool_value(payload.integrity.warn_before_backup),
        ),
        (
            KEY_ACCESS_VIEWER_CAN_EXPORT_DOCX.to_string(),
            bool_value(payload.access.viewer_can_export_docx),
        ),
        (
            KEY_ACCESS_ANALYST_CAN_CREATE_BACKUP.to_string(),
            bool_value(payload.access.analyst_can_create_backup),
        ),
    ]
}

fn settings_to_map(s: &AppSettingsDto) -> HashMap<String, Value> {
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

fn validate_update_payload(payload: &UpdateSettingsPayload) -> Result<(), AppErrorDto> {
    let allowed_templates = [
        "analytical-report",
        "operational-summary",
        "extended-report",
    ];

    if !allowed_templates.contains(&payload.docx.default_template.as_str()) {
        return Err(AppErrorDto::validation("Недопустимый DOCX-шаблон."));
    }

    if payload.docx.default_export_dir.trim().len() > 512 {
        return Err(AppErrorDto::validation(
            "Путь экспорта DOCX слишком длинный.",
        ));
    }

    if payload.backup.default_backup_dir.trim().len() > 512 {
        return Err(AppErrorDto::validation(
            "Путь резервного копирования слишком длинный.",
        ));
    }

    Ok(())
}
