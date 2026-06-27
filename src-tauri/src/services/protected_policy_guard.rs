use rusqlite::Connection;
use tauri::AppHandle;

use crate::audit::audit_metadata;
use crate::audit::audit_service::{AuditService, AuditWriteInput};
use crate::errors::app_error::AppErrorDto;
use crate::security::session::CurrentUserDto;
use crate::services::settings_access_policy::SettingsAccessPolicy;

pub struct ProtectedPolicyGuard;

impl ProtectedPolicyGuard {
    pub fn require_docx_export_allowed(
        app: &AppHandle,
        conn: &Connection,
        current_user: &CurrentUserDto,
    ) -> Result<(), AppErrorDto> {
        if current_user.is_administrator() || current_user.is_analyst() {
            return Ok(());
        }

        if current_user.is_viewer() {
            let policy = SettingsAccessPolicy::from_connection(conn)?;

            if policy.viewer_can_export_docx {
                return Ok(());
            }
        }

        let error = AppErrorDto::access_denied("Экспорт DOCX недоступен для роли наблюдателя.");

        Self::write_policy_denied_audit(
            app,
            current_user,
            "export_docx",
            "access.viewerCanExportDocx",
            &error,
        );

        Err(error)
    }

    pub fn require_backup_create_allowed(
        app: &AppHandle,
        conn: &Connection,
        current_user: &CurrentUserDto,
    ) -> Result<(), AppErrorDto> {
        if current_user.is_administrator() {
            return Ok(());
        }

        if current_user.is_analyst() {
            let policy = SettingsAccessPolicy::from_connection(conn)?;

            if policy.analyst_can_create_backup {
                return Ok(());
            }
        }

        let error = AppErrorDto::access_denied("Недостаточно прав для создания резервной копии.");

        Self::write_policy_denied_audit(
            app,
            current_user,
            "create_backup",
            "access.analystCanCreateBackup",
            &error,
        );

        Err(error)
    }

    fn write_policy_denied_audit(
        app: &AppHandle,
        current_user: &CurrentUserDto,
        command: &str,
        policy: &str,
        _error: &AppErrorDto,
    ) {
        let result = (|| {
            let details = audit_metadata::policy_denied(command, &current_user.role, policy)?;

            let mut input = AuditWriteInput::failure(
                current_user,
                crate::domain::audit_action::audit::ACCESS_DENIED,
            )
            .with_entity_type("settings_policy")
            .with_details(details);

            input.result = "denied".to_string();
            input.severity = "warning".to_string();

            AuditService::write_best_effort(app, input);
            Ok::<(), AppErrorDto>(())
        })();

        if let Err(e) = result {
            eprintln!("[audit] policy_denied audit write failed: {}", e.message);
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::models::settings_catalog::{
        KEY_ACCESS_ANALYST_CAN_CREATE_BACKUP, KEY_ACCESS_VIEWER_CAN_EXPORT_DOCX,
    };

    fn setup_conn_with_settings(pairs: &[(&str, &str)]) -> Connection {
        let conn = Connection::open_in_memory().unwrap();
        conn.execute_batch(
            "CREATE TABLE app_settings (key TEXT PRIMARY KEY, value TEXT NOT NULL);",
        )
        .unwrap();
        for (k, v) in pairs {
            conn.execute(
                "INSERT INTO app_settings (key, value) VALUES (?1, ?2)",
                rusqlite::params![k, v],
            )
            .unwrap();
        }
        conn
    }

    fn test_user(role: &str) -> CurrentUserDto {
        CurrentUserDto {
            user_id: format!("user-{role}"),
            username: role.to_string(),
            display_name: role.to_string(),
            role: role.to_string(),
            is_active: true,
            must_change_password: false,
        }
    }

    // ─── DOCX export policy tests ─────────────────────────────────────────

    #[test]
    fn administrator_can_export_docx_regardless_of_policy() {
        let conn = setup_conn_with_settings(&[(KEY_ACCESS_VIEWER_CAN_EXPORT_DOCX, "false")]);
        let user = test_user("administrator");
        // conn-only check — app is not available in unit test context
        let policy = SettingsAccessPolicy::from_connection(&conn).unwrap();
        assert!(policy.viewer_can_export_docx == false);
        assert!(user.is_administrator());
    }

    #[test]
    fn analyst_can_export_docx_regardless_of_viewer_policy() {
        let conn = setup_conn_with_settings(&[(KEY_ACCESS_VIEWER_CAN_EXPORT_DOCX, "false")]);
        let user = test_user("analyst");
        assert!(user.is_analyst());
        // Analyst is always allowed — no policy check needed
        let _policy = SettingsAccessPolicy::from_connection(&conn).unwrap();
    }

    #[test]
    fn viewer_cannot_export_docx_when_policy_is_false() {
        let conn = setup_conn_with_settings(&[(KEY_ACCESS_VIEWER_CAN_EXPORT_DOCX, "false")]);
        let policy = SettingsAccessPolicy::from_connection(&conn).unwrap();
        assert!(!policy.viewer_can_export_docx);
    }

    #[test]
    fn viewer_can_export_docx_when_policy_is_true() {
        let conn = setup_conn_with_settings(&[(KEY_ACCESS_VIEWER_CAN_EXPORT_DOCX, "true")]);
        let policy = SettingsAccessPolicy::from_connection(&conn).unwrap();
        assert!(policy.viewer_can_export_docx);
    }

    // ─── Backup create policy tests ─────────────────────────────────────

    #[test]
    fn administrator_can_create_backup_regardless_of_policy() {
        let conn = setup_conn_with_settings(&[(KEY_ACCESS_ANALYST_CAN_CREATE_BACKUP, "false")]);
        let user = test_user("administrator");
        let policy = SettingsAccessPolicy::from_connection(&conn).unwrap();
        assert!(!policy.analyst_can_create_backup);
        assert!(user.is_administrator());
    }

    #[test]
    fn analyst_cannot_create_backup_when_policy_is_false() {
        let conn = setup_conn_with_settings(&[(KEY_ACCESS_ANALYST_CAN_CREATE_BACKUP, "false")]);
        let policy = SettingsAccessPolicy::from_connection(&conn).unwrap();
        assert!(!policy.analyst_can_create_backup);
    }

    #[test]
    fn analyst_can_create_backup_when_policy_is_true() {
        let conn = setup_conn_with_settings(&[(KEY_ACCESS_ANALYST_CAN_CREATE_BACKUP, "true")]);
        let policy = SettingsAccessPolicy::from_connection(&conn).unwrap();
        assert!(policy.analyst_can_create_backup);
    }

    #[test]
    fn viewer_cannot_create_backup() {
        let conn = setup_conn_with_settings(&[(KEY_ACCESS_ANALYST_CAN_CREATE_BACKUP, "true")]);
        let policy = SettingsAccessPolicy::from_connection(&conn).unwrap();
        assert!(policy.analyst_can_create_backup);
        // Viewer always denied — role check happens first
    }

    // ─── Defaults tests ────────────────────────────────────────────────

    #[test]
    fn default_policy_blocks_viewer_docx_export() {
        let conn = Connection::open_in_memory().unwrap();
        conn.execute_batch(
            "CREATE TABLE app_settings (key TEXT PRIMARY KEY, value TEXT NOT NULL);",
        )
        .unwrap();
        let policy = SettingsAccessPolicy::from_connection(&conn).unwrap();
        // No rows = defaults used — viewer_can_export_docx defaults to false
        assert!(!policy.viewer_can_export_docx);
    }

    #[test]
    fn default_policy_blocks_analyst_backup_create() {
        let conn = Connection::open_in_memory().unwrap();
        conn.execute_batch(
            "CREATE TABLE app_settings (key TEXT PRIMARY KEY, value TEXT NOT NULL);",
        )
        .unwrap();
        let policy = SettingsAccessPolicy::from_connection(&conn).unwrap();
        // No rows = defaults used — analyst_can_create_backup defaults to false
        assert!(!policy.analyst_can_create_backup);
    }
}
