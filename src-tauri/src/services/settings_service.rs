use std::collections::HashMap;

use rusqlite::Connection;
use serde_json::Value;
use tauri::AppHandle;

use crate::errors::app_error::AppErrorDto;
use crate::security::session::SessionState;
use crate::services::protected_service_context::require_protected_user_for;

pub struct SettingsRepository;

impl SettingsRepository {
    pub fn get_settings_map(_conn: &Connection) -> Result<HashMap<String, Value>, AppErrorDto> {
        // Stub implementation returning mock settings
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

pub struct UpdateSettingsPayload {
    pub settings: HashMap<String, Value>,
}

pub struct SettingsService;

impl SettingsService {
    pub fn update_settings(
        app: &AppHandle,
        session: &SessionState,
        payload: UpdateSettingsPayload,
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

            let old_value = crate::audit::audit_metadata::snapshot(
                crate::audit::audit_metadata::settings_snapshot(old_snapshot_changes),
            );

            let new_value = crate::audit::audit_metadata::snapshot(
                crate::audit::audit_metadata::settings_snapshot(new_snapshot_changes),
            );

            let input = crate::services::audit_service::AuditSuccessInput::new(
                &context.current_user,
                crate::domain::audit_action::settings::UPDATED,
                "settings",
                None,
                None,
                old_value,
                new_value,
                crate::audit::audit_metadata::settings_updated(&changed_keys, &categories),
            );

            crate::services::audit_service::AuditService::write_success_non_blocking(
                app.clone(),
                input,
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

            let old_value = crate::audit::audit_metadata::snapshot(
                crate::audit::audit_metadata::settings_snapshot(old_snapshot_changes),
            );

            let new_value = crate::audit::audit_metadata::snapshot(
                crate::audit::audit_metadata::settings_snapshot(new_snapshot_changes),
            );

            let input = crate::services::audit_service::AuditSuccessInput::new(
                &context.current_user,
                crate::domain::audit_action::settings::RESET_TO_DEFAULT,
                "settings",
                None,
                None,
                old_value,
                new_value,
                crate::audit::audit_metadata::settings_reset_to_default(&changed_keys),
            );

            crate::services::audit_service::AuditService::write_success_non_blocking(
                app.clone(),
                input,
            );
        }

        Ok(())
    }
}

fn settings_category_for_key(key: &str) -> &'static str {
    match key {
        "viewer_can_export_docx" | "analyst_can_create_backup" => "access",
        "storage_path" | "backup_path" | "export_path" => "storage",
        "docx_template" | "default_report_type" => "docx",
        "integrity_check_before_export" => "integrity",
        "audit_strict_mode" => "audit",
        _ => "general",
    }
}
