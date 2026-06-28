use std::collections::HashSet;

use rusqlite::Connection;

use crate::audit::audit_metadata;
use crate::security::permission_decision::{PermissionDecision, PermissionDenyReason};
use crate::security::policy_aware_permission_service::PolicyAwarePermissionService;
use crate::security::protected_operation::ProtectedOperation;
use crate::security::session::CurrentUserDto;

// ─── Test model ───────────────────────────────────────────────────────────

#[derive(Debug, Clone, Copy)]
enum TestRole {
    Administrator,
    Analyst,
    Viewer,
}

#[derive(Debug, Clone, Copy)]
struct TestPolicyFlags {
    viewer_can_export_docx: bool,
    analyst_can_create_backup: bool,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum ExpectedPermission {
    Allow,
    DenyRole,
    DenyPolicy { policy_key: &'static str },
}

#[derive(Debug, Clone, Copy)]
struct MatrixRow {
    operation: ProtectedOperation,
    role: TestRole,
    policy: TestPolicyFlags,
    expected: ExpectedPermission,
}

// ─── Helpers ──────────────────────────────────────────────────────────────

fn role_name(role: TestRole) -> &'static str {
    match role {
        TestRole::Administrator => "administrator",
        TestRole::Analyst => "analyst",
        TestRole::Viewer => "viewer",
    }
}

fn current_user(role: TestRole) -> CurrentUserDto {
    CurrentUserDto {
        user_id: format!("test-user-{}", role_name(role)),
        username: format!("{}_user", role_name(role)),
        display_name: format!("{}_user", role_name(role)),
        role: role_name(role).to_string(),
        is_active: true,
        must_change_password: false,
    }
}

fn setup_permission_test_db(policy: TestPolicyFlags) -> Connection {
    let conn = Connection::open_in_memory().expect("open test db");

    conn.execute_batch(
        r#"
        CREATE TABLE app_settings (
            key TEXT PRIMARY KEY NOT NULL,
            value TEXT NOT NULL,
            updated_at TEXT NOT NULL DEFAULT CURRENT_TIMESTAMP
        );
        "#,
    )
    .expect("create app_settings");

    conn.execute(
        "INSERT INTO app_settings (key, value) VALUES (?1, ?2)",
        rusqlite::params![
            "access.viewer_can_export_docx",
            if policy.viewer_can_export_docx {
                "true"
            } else {
                "false"
            },
        ],
    )
    .expect("insert viewerCanExportDocx");

    conn.execute(
        "INSERT INTO app_settings (key, value) VALUES (?1, ?2)",
        rusqlite::params![
            "access.analyst_can_create_backup",
            if policy.analyst_can_create_backup {
                "true"
            } else {
                "false"
            },
        ],
    )
    .expect("insert analystCanCreateBackup");

    conn
}

// ─── Row builders ─────────────────────────────────────────────────────────

fn allow(operation: ProtectedOperation, role: TestRole, policy: TestPolicyFlags) -> Vec<MatrixRow> {
    vec![MatrixRow {
        operation,
        role,
        policy,
        expected: ExpectedPermission::Allow,
    }]
}

fn deny_role(
    operation: ProtectedOperation,
    role: TestRole,
    policy: TestPolicyFlags,
) -> Vec<MatrixRow> {
    vec![MatrixRow {
        operation,
        role,
        policy,
        expected: ExpectedPermission::DenyRole,
    }]
}

fn deny_policy(
    operation: ProtectedOperation,
    role: TestRole,
    policy: TestPolicyFlags,
    policy_key: &'static str,
) -> Vec<MatrixRow> {
    vec![MatrixRow {
        operation,
        role,
        policy,
        expected: ExpectedPermission::DenyPolicy { policy_key },
    }]
}

fn analyst_admin_only(operation: ProtectedOperation, policy: TestPolicyFlags) -> Vec<MatrixRow> {
    vec![
        MatrixRow {
            operation,
            role: TestRole::Administrator,
            policy,
            expected: ExpectedPermission::Allow,
        },
        MatrixRow {
            operation,
            role: TestRole::Analyst,
            policy,
            expected: ExpectedPermission::Allow,
        },
        MatrixRow {
            operation,
            role: TestRole::Viewer,
            policy,
            expected: ExpectedPermission::DenyRole,
        },
    ]
}

// ─── Matrix rows ──────────────────────────────────────────────────────────

fn matrix_rows() -> Vec<MatrixRow> {
    let p00 = TestPolicyFlags {
        viewer_can_export_docx: false,
        analyst_can_create_backup: false,
    };

    let p10 = TestPolicyFlags {
        viewer_can_export_docx: true,
        analyst_can_create_backup: false,
    };

    let p01 = TestPolicyFlags {
        viewer_can_export_docx: false,
        analyst_can_create_backup: true,
    };

    vec![
        // Read operations: all active authenticated roles allowed.
        allow(ProtectedOperation::CaseRead, TestRole::Administrator, p00),
        allow(ProtectedOperation::CaseRead, TestRole::Analyst, p00),
        allow(ProtectedOperation::CaseRead, TestRole::Viewer, p00),
        allow(
            ProtectedOperation::MaterialRead,
            TestRole::Administrator,
            p00,
        ),
        allow(ProtectedOperation::MaterialRead, TestRole::Analyst, p00),
        allow(ProtectedOperation::MaterialRead, TestRole::Viewer, p00),
        allow(ProtectedOperation::ObjectRead, TestRole::Administrator, p00),
        allow(ProtectedOperation::ObjectRead, TestRole::Analyst, p00),
        allow(ProtectedOperation::ObjectRead, TestRole::Viewer, p00),
        allow(
            ProtectedOperation::RelationRead,
            TestRole::Administrator,
            p00,
        ),
        allow(ProtectedOperation::RelationRead, TestRole::Analyst, p00),
        allow(ProtectedOperation::RelationRead, TestRole::Viewer, p00),
        allow(
            ProtectedOperation::TimelineRead,
            TestRole::Administrator,
            p00,
        ),
        allow(ProtectedOperation::TimelineRead, TestRole::Analyst, p00),
        allow(ProtectedOperation::TimelineRead, TestRole::Viewer, p00),
        allow(
            ProtectedOperation::ReportDraftRead,
            TestRole::Administrator,
            p00,
        ),
        allow(ProtectedOperation::ReportDraftRead, TestRole::Analyst, p00),
        allow(ProtectedOperation::ReportDraftRead, TestRole::Viewer, p00),
        allow(ProtectedOperation::BackupRead, TestRole::Administrator, p00),
        allow(ProtectedOperation::BackupRead, TestRole::Analyst, p00),
        allow(ProtectedOperation::BackupRead, TestRole::Viewer, p00),
        allow(
            ProtectedOperation::IntegrityCheckRead,
            TestRole::Administrator,
            p00,
        ),
        allow(
            ProtectedOperation::IntegrityCheckRead,
            TestRole::Analyst,
            p00,
        ),
        allow(
            ProtectedOperation::IntegrityCheckRead,
            TestRole::Viewer,
            p00,
        ),
        // Analyst/admin write operations.
        analyst_admin_only(ProtectedOperation::CaseCreate, p00),
        analyst_admin_only(ProtectedOperation::CaseUpdate, p00),
        analyst_admin_only(ProtectedOperation::MaterialImport, p00),
        analyst_admin_only(ProtectedOperation::MaterialUpdate, p00),
        analyst_admin_only(ProtectedOperation::ObjectCreate, p00),
        analyst_admin_only(ProtectedOperation::ObjectUpdate, p00),
        analyst_admin_only(ProtectedOperation::RelationCreate, p00),
        analyst_admin_only(ProtectedOperation::RelationUpdate, p00),
        analyst_admin_only(ProtectedOperation::TimelineCreate, p00),
        analyst_admin_only(ProtectedOperation::TimelineUpdate, p00),
        analyst_admin_only(ProtectedOperation::ReportDraftGenerate, p00),
        analyst_admin_only(ProtectedOperation::ReportDraftUpdate, p00),
        analyst_admin_only(ProtectedOperation::IntegrityCheckRun, p00),
        // DOCX export.
        allow(ProtectedOperation::DocxExport, TestRole::Administrator, p00),
        allow(ProtectedOperation::DocxExport, TestRole::Analyst, p00),
        deny_policy(
            ProtectedOperation::DocxExport,
            TestRole::Viewer,
            p00,
            "access.viewer_can_export_docx",
        ),
        allow(ProtectedOperation::DocxExport, TestRole::Viewer, p10),
        // Backup create.
        allow(
            ProtectedOperation::BackupCreate,
            TestRole::Administrator,
            p00,
        ),
        deny_policy(
            ProtectedOperation::BackupCreate,
            TestRole::Analyst,
            p00,
            "access.analyst_can_create_backup",
        ),
        allow(ProtectedOperation::BackupCreate, TestRole::Analyst, p01),
        deny_role(ProtectedOperation::BackupCreate, TestRole::Viewer, p01),
        // Backup verify.
        allow(
            ProtectedOperation::BackupVerify,
            TestRole::Administrator,
            p00,
        ),
        allow(ProtectedOperation::BackupVerify, TestRole::Analyst, p00),
        deny_role(ProtectedOperation::BackupVerify, TestRole::Viewer, p00),
        // Restore: admin-only.
        allow(
            ProtectedOperation::BackupRestore,
            TestRole::Administrator,
            p00,
        ),
        deny_role(ProtectedOperation::BackupRestore, TestRole::Analyst, p00),
        deny_role(ProtectedOperation::BackupRestore, TestRole::Viewer, p00),
        // Audit read.
        allow(
            ProtectedOperation::AuditLogRead,
            TestRole::Administrator,
            p00,
        ),
        allow(ProtectedOperation::AuditLogRead, TestRole::Analyst, p00),
        deny_role(ProtectedOperation::AuditLogRead, TestRole::Viewer, p00),
        // Settings.
        allow(
            ProtectedOperation::SettingsRead,
            TestRole::Administrator,
            p00,
        ),
        deny_role(ProtectedOperation::SettingsRead, TestRole::Analyst, p00),
        deny_role(ProtectedOperation::SettingsRead, TestRole::Viewer, p00),
        allow(
            ProtectedOperation::SettingsUpdate,
            TestRole::Administrator,
            p00,
        ),
        deny_role(ProtectedOperation::SettingsUpdate, TestRole::Analyst, p00),
        deny_role(ProtectedOperation::SettingsUpdate, TestRole::Viewer, p00),
        // User management.
        allow(ProtectedOperation::UserManage, TestRole::Administrator, p00),
        deny_role(ProtectedOperation::UserManage, TestRole::Analyst, p00),
        deny_role(ProtectedOperation::UserManage, TestRole::Viewer, p00),
    ]
    .into_iter()
    .flatten()
    .collect()
}

// ─── Assertion helper ─────────────────────────────────────────────────────

fn assert_permission(row: MatrixRow, actual: PermissionDecision) {
    match (row.expected, actual) {
        (ExpectedPermission::Allow, PermissionDecision::Allow) => {}

        (
            ExpectedPermission::DenyRole,
            PermissionDecision::Deny {
                reason: PermissionDenyReason::RoleDenied,
                ..
            },
        ) => {}

        (
            ExpectedPermission::DenyPolicy {
                policy_key: expected_key,
            },
            PermissionDecision::Deny {
                reason: PermissionDenyReason::PolicyDenied { policy_key },
                ..
            },
        ) => {
            assert_eq!(expected_key, policy_key);
        }

        (expected, actual) => {
            panic!(
                "permission mismatch: operation={:?}, role={:?}, expected={:?}, actual={:?}",
                row.operation, row.role, expected, actual
            );
        }
    }
}

// ─── Tests ────────────────────────────────────────────────────────────────

#[test]
fn protected_operation_permission_matrix_is_stable() {
    for row in matrix_rows() {
        let conn = setup_permission_test_db(row.policy);
        let user = current_user(row.role);

        let actual = PolicyAwarePermissionService::decide(&conn, &user, row.operation);

        assert_permission(row, actual);
    }
}

#[test]
fn every_protected_operation_has_matrix_rows() {
    let covered: HashSet<ProtectedOperation> =
        matrix_rows().into_iter().map(|row| row.operation).collect();

    for operation in ProtectedOperation::all() {
        assert!(
            covered.contains(operation),
            "missing permission matrix rows for {:?}",
            operation
        );
    }
}

#[test]
fn every_protected_operation_has_stable_action_name() {
    for operation in ProtectedOperation::all() {
        let action = operation.action_name();

        assert!(!action.is_empty());
        assert!(
            action.contains('.'),
            "operation action name must be namespaced: {:?} -> {}",
            operation,
            action
        );
        assert!(
            action.chars().all(|c| c.is_ascii_lowercase() || c == '.'),
            "operation action name must be lowercase dotted id: {:?} -> {}",
            operation,
            action
        );
    }
}

#[test]
fn policy_deny_audit_details_include_policy_key() {
    let details = audit_metadata::access_denied_details(
        "docx.export",
        "policy_denied",
        Some("access.viewer_can_export_docx"),
    )
    .expect("should build details");

    let value = details.clone_value_for_test();

    assert_eq!(value["operation"], "docx.export");
    assert_eq!(value["reason"], "policy_denied");
    assert_eq!(value["policyKey"], "access.viewer_can_export_docx");
}

#[test]
fn role_deny_audit_details_do_not_include_policy_key() {
    let details = audit_metadata::access_denied_details("backup.create", "role_denied", None)
        .expect("should build details");

    let value = details.clone_value_for_test();

    assert_eq!(value["operation"], "backup.create");
    assert_eq!(value["reason"], "role_denied");
    assert!(!value.as_object().unwrap().contains_key("policyKey"));
}
