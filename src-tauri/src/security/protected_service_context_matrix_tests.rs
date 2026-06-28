use crate::security::permission_decision::PermissionDecision;
use crate::security::policy_aware_permission_service::PolicyAwarePermissionService;
use crate::security::protected_operation::ProtectedOperation;
use crate::security::session::CurrentUserDto;

use rusqlite::Connection;

// ─── Helpers ──────────────────────────────────────────────────────────────

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

// ─── Permission decision layer: inactive / password-change do not affect ──

#[test]
fn permission_decision_ignores_inactive_user() {
    let conn = setup_conn();
    let user = test_user("viewer", false, false);
    // The permission layer (decide) does not check is_active — context layer does.
    let decision = PolicyAwarePermissionService::decide(&conn, &user, ProtectedOperation::CaseRead);
    assert_eq!(decision, PermissionDecision::Allow);
}

#[test]
fn permission_decision_ignores_must_change_password() {
    let conn = setup_conn();
    let user = test_user("analyst", true, true);
    // The permission layer (decide) does not check must_change_password — context layer does.
    let decision = PolicyAwarePermissionService::decide(&conn, &user, ProtectedOperation::CaseRead);
    assert_eq!(decision, PermissionDecision::Allow);
}

#[test]
fn inactive_user_still_denied_by_role() {
    let conn = setup_conn();
    let user = test_user("viewer", false, false);
    let decision =
        PolicyAwarePermissionService::decide(&conn, &user, ProtectedOperation::SettingsUpdate);
    assert!(!decision.allowed());
}

#[test]
fn password_change_user_still_denied_by_role() {
    let conn = setup_conn();
    let user = test_user("viewer", true, true);
    let decision =
        PolicyAwarePermissionService::decide(&conn, &user, ProtectedOperation::CaseCreate);
    assert!(!decision.allowed());
}

// ─── any_authenticated applies regardless of inactive/password flags ──────

#[test]
fn any_authenticated_allows_inactive_user() {
    let conn = setup_conn();
    let user = test_user("analyst", false, false);
    let decision = PolicyAwarePermissionService::decide(&conn, &user, ProtectedOperation::CaseRead);
    assert_eq!(decision, PermissionDecision::Allow);
}

#[test]
fn any_authenticated_allows_password_change_user() {
    let conn = setup_conn();
    let user = test_user("analyst", true, true);
    let decision = PolicyAwarePermissionService::decide(&conn, &user, ProtectedOperation::CaseRead);
    assert_eq!(decision, PermissionDecision::Allow);
}
