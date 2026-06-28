use rusqlite::Connection;

use crate::audit::audit_metadata;
use crate::models::settings_catalog::{
    KEY_ACCESS_ANALYST_CAN_CREATE_BACKUP, KEY_ACCESS_VIEWER_CAN_EXPORT_DOCX,
};
use crate::security::permission_decision::{PermissionDecision, PermissionDenyReason};
use crate::security::policy_aware_permission_service::PolicyAwarePermissionService;
use crate::security::protected_operation::ProtectedOperation;
use crate::security::session::CurrentUserDto;

// ─── Helpers ────────────────────────────────────────────────────────────

fn setup_conn_with_settings(pairs: &[(&str, &str)]) -> Connection {
    let conn = Connection::open_in_memory().unwrap();
    conn.execute_batch("CREATE TABLE app_settings (key TEXT PRIMARY KEY, value TEXT NOT NULL);")
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

fn set_setting(conn: &Connection, key: &str, value: &str) {
    conn.execute(
        "INSERT OR REPLACE INTO app_settings (key, value) VALUES (?1, ?2)",
        rusqlite::params![key, value],
    )
    .unwrap();
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

fn assert_allowed(conn: &Connection, user: &CurrentUserDto, operation: ProtectedOperation) {
    let decision = PolicyAwarePermissionService::decide(conn, user, operation);
    assert_eq!(decision, PermissionDecision::Allow);
}

fn assert_denied_role(conn: &Connection, user: &CurrentUserDto, operation: ProtectedOperation) {
    let decision = PolicyAwarePermissionService::decide(conn, user, operation);

    assert!(matches!(
        decision,
        PermissionDecision::Deny {
            reason: PermissionDenyReason::RoleDenied,
            ..
        }
    ));
}

fn assert_denied_policy(
    conn: &Connection,
    user: &CurrentUserDto,
    operation: ProtectedOperation,
    expected_policy_key: &'static str,
) {
    let decision = PolicyAwarePermissionService::decide(conn, user, operation);

    assert!(matches!(
        decision,
        PermissionDecision::Deny {
            reason: PermissionDenyReason::PolicyDenied { policy_key },
            ..
        } if policy_key == expected_policy_key
    ));
}

// ─── Admin operations ──────────────────────────────────────────────────

#[test]
fn admin_can_perform_admin_operations() {
    let conn = setup_conn_with_settings(&[]);
    let user = test_user("administrator");

    assert_allowed(&conn, &user, ProtectedOperation::SettingsUpdate);
    assert_allowed(&conn, &user, ProtectedOperation::UserManage);
    assert_allowed(&conn, &user, ProtectedOperation::BackupRestore);
    assert_allowed(&conn, &user, ProtectedOperation::SettingsRead);
    assert_allowed(&conn, &user, ProtectedOperation::AuditLogRead);
}

#[test]
fn admin_can_perform_all_analyst_operations() {
    let conn = setup_conn_with_settings(&[]);
    let user = test_user("administrator");

    assert_allowed(&conn, &user, ProtectedOperation::CaseCreate);
    assert_allowed(&conn, &user, ProtectedOperation::CaseUpdate);
    assert_allowed(&conn, &user, ProtectedOperation::MaterialImport);
    assert_allowed(&conn, &user, ProtectedOperation::MaterialUpdate);
    assert_allowed(&conn, &user, ProtectedOperation::ObjectCreate);
    assert_allowed(&conn, &user, ProtectedOperation::ObjectUpdate);
    assert_allowed(&conn, &user, ProtectedOperation::RelationCreate);
    assert_allowed(&conn, &user, ProtectedOperation::RelationUpdate);
    assert_allowed(&conn, &user, ProtectedOperation::TimelineCreate);
    assert_allowed(&conn, &user, ProtectedOperation::TimelineUpdate);
    assert_allowed(&conn, &user, ProtectedOperation::ReportDraftGenerate);
    assert_allowed(&conn, &user, ProtectedOperation::ReportDraftUpdate);
    assert_allowed(&conn, &user, ProtectedOperation::IntegrityCheckRun);
}

// ─── DOCX export policy ────────────────────────────────────────────────

#[test]
fn admin_can_export_docx_regardless_of_policy() {
    let conn = setup_conn_with_settings(&[(KEY_ACCESS_VIEWER_CAN_EXPORT_DOCX, "false")]);
    let user = test_user("administrator");
    assert_allowed(&conn, &user, ProtectedOperation::DocxExport);
}

#[test]
fn analyst_can_export_docx_regardless_of_viewer_policy() {
    let conn = setup_conn_with_settings(&[(KEY_ACCESS_VIEWER_CAN_EXPORT_DOCX, "false")]);
    let user = test_user("analyst");
    assert_allowed(&conn, &user, ProtectedOperation::DocxExport);
}

#[test]
fn viewer_docx_export_depends_on_policy() {
    let conn = setup_conn_with_settings(&[(KEY_ACCESS_VIEWER_CAN_EXPORT_DOCX, "false")]);
    let user = test_user("viewer");

    assert_denied_policy(
        &conn,
        &user,
        ProtectedOperation::DocxExport,
        KEY_ACCESS_VIEWER_CAN_EXPORT_DOCX,
    );

    set_setting(&conn, KEY_ACCESS_VIEWER_CAN_EXPORT_DOCX, "true");

    assert_allowed(&conn, &user, ProtectedOperation::DocxExport);
}

// ─── Backup create policy ──────────────────────────────────────────────

#[test]
fn admin_can_create_backup_regardless_of_policy() {
    let conn = setup_conn_with_settings(&[(KEY_ACCESS_ANALYST_CAN_CREATE_BACKUP, "false")]);
    let user = test_user("administrator");
    assert_allowed(&conn, &user, ProtectedOperation::BackupCreate);
}

#[test]
fn analyst_backup_create_depends_on_policy() {
    let conn = setup_conn_with_settings(&[(KEY_ACCESS_ANALYST_CAN_CREATE_BACKUP, "false")]);
    let user = test_user("analyst");

    assert_denied_policy(
        &conn,
        &user,
        ProtectedOperation::BackupCreate,
        KEY_ACCESS_ANALYST_CAN_CREATE_BACKUP,
    );

    set_setting(&conn, KEY_ACCESS_ANALYST_CAN_CREATE_BACKUP, "true");

    assert_allowed(&conn, &user, ProtectedOperation::BackupCreate);
}

#[test]
fn viewer_cannot_create_backup_even_when_analyst_policy_enabled() {
    let conn = setup_conn_with_settings(&[(KEY_ACCESS_ANALYST_CAN_CREATE_BACKUP, "true")]);
    let user = test_user("viewer");

    assert_denied_role(&conn, &user, ProtectedOperation::BackupCreate);
}

// ─── Backup restore ────────────────────────────────────────────────────

#[test]
fn only_admin_can_restore_backup() {
    let conn = setup_conn_with_settings(&[]);
    let admin = test_user("administrator");
    let analyst = test_user("analyst");
    let viewer = test_user("viewer");

    assert_allowed(&conn, &admin, ProtectedOperation::BackupRestore);
    assert_denied_role(&conn, &analyst, ProtectedOperation::BackupRestore);
    assert_denied_role(&conn, &viewer, ProtectedOperation::BackupRestore);
}

// ─── Audit log read ────────────────────────────────────────────────────

#[test]
fn admin_and_analyst_can_read_audit_log() {
    let conn = setup_conn_with_settings(&[]);
    let admin = test_user("administrator");
    let analyst = test_user("analyst");
    let viewer = test_user("viewer");

    assert_allowed(&conn, &admin, ProtectedOperation::AuditLogRead);
    assert_allowed(&conn, &analyst, ProtectedOperation::AuditLogRead);
    assert_denied_role(&conn, &viewer, ProtectedOperation::AuditLogRead);
}

// ─── Role checks ───────────────────────────────────────────────────────

#[test]
fn viewer_is_denied_all_analyst_operations() {
    let conn = setup_conn_with_settings(&[]);
    let user = test_user("viewer");

    assert_denied_role(&conn, &user, ProtectedOperation::CaseCreate);
    assert_denied_role(&conn, &user, ProtectedOperation::CaseUpdate);
    assert_denied_role(&conn, &user, ProtectedOperation::MaterialImport);
    assert_denied_role(&conn, &user, ProtectedOperation::MaterialUpdate);
    assert_denied_role(&conn, &user, ProtectedOperation::ObjectCreate);
    assert_denied_role(&conn, &user, ProtectedOperation::ObjectUpdate);
    assert_denied_role(&conn, &user, ProtectedOperation::RelationCreate);
    assert_denied_role(&conn, &user, ProtectedOperation::RelationUpdate);
    assert_denied_role(&conn, &user, ProtectedOperation::TimelineCreate);
    assert_denied_role(&conn, &user, ProtectedOperation::TimelineUpdate);
    assert_denied_role(&conn, &user, ProtectedOperation::ReportDraftGenerate);
    assert_denied_role(&conn, &user, ProtectedOperation::ReportDraftUpdate);
    assert_denied_role(&conn, &user, ProtectedOperation::IntegrityCheckRun);
}

#[test]
fn viewer_is_denied_admin_operations() {
    let conn = setup_conn_with_settings(&[]);
    let user = test_user("viewer");

    assert_denied_role(&conn, &user, ProtectedOperation::SettingsRead);
    assert_denied_role(&conn, &user, ProtectedOperation::SettingsUpdate);
    assert_denied_role(&conn, &user, ProtectedOperation::UserManage);
    assert_denied_role(&conn, &user, ProtectedOperation::BackupRestore);
}

#[test]
fn analyst_is_denied_admin_operations() {
    let conn = setup_conn_with_settings(&[]);
    let user = test_user("analyst");

    assert_denied_role(&conn, &user, ProtectedOperation::SettingsRead);
    assert_denied_role(&conn, &user, ProtectedOperation::SettingsUpdate);
    assert_denied_role(&conn, &user, ProtectedOperation::UserManage);
    assert_denied_role(&conn, &user, ProtectedOperation::BackupRestore);
}

// ─── Audit details safety ──────────────────────────────────────────────

#[test]
fn access_denied_details_contains_only_safe_policy_context() {
    let details = audit_metadata::access_denied_details(
        "backup.create",
        "policy_denied",
        Some(KEY_ACCESS_ANALYST_CAN_CREATE_BACKUP),
    )
    .expect("should build details");

    let value = details.clone_value_for_test();

    assert_eq!(value["operation"], "backup.create");
    assert_eq!(value["reason"], "policy_denied");
    assert_eq!(value["policyKey"], KEY_ACCESS_ANALYST_CAN_CREATE_BACKUP);
}

#[test]
fn access_denied_details_without_policy_key() {
    let details = audit_metadata::access_denied_details("docx.export", "role_denied", None)
        .expect("should build details");

    let value = details.clone_value_for_test();

    assert_eq!(value["operation"], "docx.export");
    assert_eq!(value["reason"], "role_denied");
    assert!(!value.as_object().unwrap().contains_key("policyKey"));
}
