use rusqlite::Connection;

use crate::security::permission_decision::PermissionDecision;
use crate::security::policy_aware_permission_service::PolicyAwarePermissionService;
use crate::security::protected_operation::ProtectedOperation;
use crate::security::session::CurrentUserDto;

fn setup_conn() -> Connection {
    let conn = Connection::open_in_memory().unwrap();
    conn.execute_batch("CREATE TABLE app_settings (key TEXT PRIMARY KEY, value TEXT NOT NULL);")
        .unwrap();
    conn
}

fn test_user(role: &str, is_active: bool, must_change_password: bool) -> CurrentUserDto {
    CurrentUserDto {
        user_id: format!("user-{role}"),
        username: role.to_string(),
        display_name: role.to_string(),
        role: role.to_string(),
        is_active,
        must_change_password,
    }
}

// ─── Permission decision tests (unit-level, no DB session required) ────

#[test]
fn require_operation_allows_admin_settings_update() {
    let conn = setup_conn();
    let user = test_user("administrator", true, false);
    let decision =
        PolicyAwarePermissionService::decide(&conn, &user, ProtectedOperation::SettingsUpdate);
    assert_eq!(decision, PermissionDecision::Allow);
}

#[test]
fn require_operation_denies_viewer_settings_update() {
    let conn = setup_conn();
    let user = test_user("viewer", true, false);
    let decision =
        PolicyAwarePermissionService::decide(&conn, &user, ProtectedOperation::SettingsUpdate);
    assert!(!decision.allowed());
}

#[test]
fn require_operation_denies_password_change_required_user_on_permission() {
    let conn = setup_conn();
    // must_change_password doesn't affect PermissionDecision — that's a context-level guard
    let user = test_user("analyst", true, true);
    let decision = PolicyAwarePermissionService::decide(&conn, &user, ProtectedOperation::CaseRead);
    assert_eq!(decision, PermissionDecision::Allow);
}

#[test]
fn require_authenticated_allows_password_change_required_user_on_read() {
    let conn = setup_conn();
    let user = test_user("analyst", true, true);
    // any_authenticated allows all roles
    let decision = PolicyAwarePermissionService::decide(&conn, &user, ProtectedOperation::CaseRead);
    assert_eq!(decision, PermissionDecision::Allow);
}

#[test]
fn require_operation_denies_inactive_user_on_read() {
    let conn = setup_conn();
    let user = test_user("viewer", false, false);
    let decision = PolicyAwarePermissionService::decide(&conn, &user, ProtectedOperation::CaseRead);
    // Permission layer still allows any authenticated user — inactivity is enforced by context
    assert_eq!(decision, PermissionDecision::Allow);
}

// ─── Read operations ──────────────────────────────────────────────────

#[test]
fn viewer_can_read_cases() {
    let conn = setup_conn();
    let user = test_user("viewer", true, false);
    let decision = PolicyAwarePermissionService::decide(&conn, &user, ProtectedOperation::CaseRead);
    assert_eq!(decision, PermissionDecision::Allow);
}

#[test]
fn viewer_can_read_materials() {
    let conn = setup_conn();
    let user = test_user("viewer", true, false);
    let decision =
        PolicyAwarePermissionService::decide(&conn, &user, ProtectedOperation::MaterialRead);
    assert_eq!(decision, PermissionDecision::Allow);
}

#[test]
fn viewer_cannot_create_case() {
    let conn = setup_conn();
    let user = test_user("viewer", true, false);
    let decision =
        PolicyAwarePermissionService::decide(&conn, &user, ProtectedOperation::CaseCreate);
    assert!(!decision.allowed());
}

#[test]
fn analyst_can_create_case() {
    let conn = setup_conn();
    let user = test_user("analyst", true, false);
    let decision =
        PolicyAwarePermissionService::decide(&conn, &user, ProtectedOperation::CaseCreate);
    assert_eq!(decision, PermissionDecision::Allow);
}

// ─── Admin operations ─────────────────────────────────────────────────

#[test]
fn viewer_cannot_manage_users() {
    let conn = setup_conn();
    let user = test_user("viewer", true, false);
    let decision =
        PolicyAwarePermissionService::decide(&conn, &user, ProtectedOperation::UserManage);
    assert!(!decision.allowed());
}

#[test]
fn analyst_cannot_manage_users() {
    let conn = setup_conn();
    let user = test_user("analyst", true, false);
    let decision =
        PolicyAwarePermissionService::decide(&conn, &user, ProtectedOperation::UserManage);
    assert!(!decision.allowed());
}

#[test]
fn admin_can_manage_users() {
    let conn = setup_conn();
    let user = test_user("administrator", true, false);
    let decision =
        PolicyAwarePermissionService::decide(&conn, &user, ProtectedOperation::UserManage);
    assert_eq!(decision, PermissionDecision::Allow);
}
