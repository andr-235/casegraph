use serde_json::json;

use crate::audit::audit_error_sanitizer::{sanitize_audit_details, sanitize_error_text};

// ── sanitize_error_text: path token redaction ─────────────────────────────

#[test]
fn sanitizes_windows_absolute_paths_in_strings() {
    let input = "failed to read C:\\Users\\Ivan\\Desktop\\secret.pdf";
    let output = sanitize_error_text(input);

    assert!(output.contains("[redacted:path]"));
    assert!(!output.contains("C:\\Users\\Ivan"));
}

#[test]
fn sanitizes_windows_path_with_forward_slash() {
    let input = "access denied: C:/Users/Admin/AppData/casegraph.sqlite";
    let output = sanitize_error_text(input);

    assert!(output.contains("[redacted:path]"));
    assert!(!output.contains("C:/Users/Admin"));
}

#[test]
fn sanitizes_unc_paths() {
    let input = r"cannot open \\server\share\backup.zip";
    let output = sanitize_error_text(input);

    assert!(output.contains("[redacted:path]"));
    assert!(!output.contains(r"\\server\share"));
}

#[test]
fn sanitizes_unix_home_paths() {
    let input = "failed to read /home/ivan/casegraph/materials/secret.pdf";
    let output = sanitize_error_text(input);

    assert!(output.contains("[redacted:path]"));
    assert!(!output.contains("/home/ivan"));
}

#[test]
fn sanitizes_unix_tmp_paths() {
    let input = "temp file at /tmp/casegraph-restore-abc123/archive.zip could not be created";
    let output = sanitize_error_text(input);

    assert!(output.contains("[redacted:path]"));
    assert!(!output.contains("/tmp/casegraph-restore"));
}

#[test]
fn sanitizes_unix_var_paths() {
    let input = "failed to write /var/data/casegraph/backup";
    let output = sanitize_error_text(input);

    assert!(output.contains("[redacted:path]"));
    assert!(!output.contains("/var/data"));
}

#[test]
fn sanitizes_macos_private_paths() {
    let input = "no such file: /private/var/folders/tmp/casegraph.zip";
    let output = sanitize_error_text(input);

    assert!(output.contains("[redacted:path]"));
    assert!(!output.contains("/private/var"));
}

#[test]
fn keeps_safe_non_path_text() {
    let input = "read error count: 3, mismatch count: 1";
    let output = sanitize_error_text(input);

    // No paths → string unchanged
    assert_eq!(output, input);
}

#[test]
fn handles_empty_string() {
    let output = sanitize_error_text("");
    assert_eq!(output, "");
}

// ── sanitize_audit_details: JSON key-based redaction ─────────────────────

#[test]
fn sanitizes_path_keys_in_json() {
    let value = json!({
        "operation": "restoreBackup",
        "archivePath": "D:\\Backups\\casegraph.zip",
        "errorCode": "ERR_RESTORE_FAILED"
    });

    let sanitized = sanitize_audit_details(value);
    let text = sanitized.to_string();

    assert!(text.contains("[redacted:path]"));
    assert!(!text.contains("D:\\\\Backups"));
    // non-sensitive fields must survive
    assert!(text.contains("restoreBackup"));
    assert!(text.contains("ERR_RESTORE_FAILED"));
}

#[test]
fn sanitizes_stored_path_key_in_json() {
    let value = json!({
        "operation": "importMaterial",
        "storedPath": "C:\\Users\\Ivan\\AppData\\casegraph\\materials\\photo.png",
        "errorCode": "ERR_MATERIAL_IMPORT_FAILED"
    });

    let sanitized = sanitize_audit_details(value);
    let text = sanitized.to_string();

    assert!(text.contains("[redacted:path]"));
    assert!(!text.contains("C:\\\\Users\\\\Ivan"));
}

#[test]
fn sanitizes_backup_path_key_in_json() {
    let value = json!({
        "operation": "createBackup",
        "backupPath": "/home/user/backups/casegraph-2024.zip",
        "errorCode": "ERR_BACKUP_FAILED"
    });

    let sanitized = sanitize_audit_details(value);

    assert_eq!(sanitized["backupPath"], "[redacted:path]");
}

#[test]
fn sanitizes_temp_dir_key_in_json() {
    let value = json!({
        "operation": "restoreBackup",
        "tempDir": "/tmp/casegraph-restore-xyz",
        "errorCode": "ERR_RESTORE_FAILED"
    });

    let sanitized = sanitize_audit_details(value);

    assert_eq!(sanitized["tempDir"], "[redacted:path]");
}

#[test]
fn sanitizes_secret_keys_in_json() {
    let value = json!({
        "operation": "resetUserPassword",
        "temporaryPassword": "Temp-123456",
        "errorCode": "ERR_VALIDATION"
    });

    let sanitized = sanitize_audit_details(value);
    let text = sanitized.to_string();

    assert!(text.contains("[redacted:secret]"));
    assert!(!text.contains("Temp-123456"));
}

#[test]
fn sanitizes_content_keys_in_json() {
    let value = json!({
        "operation": "saveReportDraft",
        "content": "Полный текст справки по делу...",
        "errorCode": "ERR_REPORT_SAVE_FAILED"
    });

    let sanitized = sanitize_audit_details(value);
    let text = sanitized.to_string();

    assert!(text.contains("[redacted:content]"));
    assert!(!text.contains("Полный текст справки"));
}

#[test]
fn sanitizes_zip_bytes_key() {
    let value = json!({
        "operation": "createBackup",
        "zipBytes": "AQIDBA==",
        "errorCode": "ERR_BACKUP_FAILED"
    });

    let sanitized = sanitize_audit_details(value);

    assert_eq!(sanitized["zipBytes"], "[redacted:content]");
}

#[test]
fn sanitizes_nested_secret_in_json() {
    let value = json!({
        "operation": "changePassword",
        "user": {
            "userId": "user-1",
            "newPassword": "s3cr3t",
            "role": "analyst"
        }
    });

    let sanitized = sanitize_audit_details(value);

    assert_eq!(sanitized["user"]["newPassword"], "[redacted:secret]");
    assert_eq!(sanitized["user"]["userId"], "user-1");
    assert_eq!(sanitized["user"]["role"], "analyst");
}

#[test]
fn keeps_safe_operational_context() {
    let value = json!({
        "operation": "runIntegrityCheck",
        "errorCode": "ERR_MATERIAL_READ_FAILED",
        "caseId": "case-1",
        "materialCode": "MAT-001",
        "checkedCount": 10,
        "readErrorCount": 1
    });

    let sanitized = sanitize_audit_details(value);

    assert_eq!(sanitized["operation"], "runIntegrityCheck");
    assert_eq!(sanitized["errorCode"], "ERR_MATERIAL_READ_FAILED");
    assert_eq!(sanitized["materialCode"], "MAT-001");
    assert_eq!(sanitized["checkedCount"], 10);
    assert_eq!(sanitized["readErrorCount"], 1);
}

#[test]
fn sanitizes_string_value_containing_windows_path() {
    let value = json!({
        "operation": "importMaterial",
        "safeReason": "Не удалось прочитать C:\\Users\\Ivan\\Desktop\\photo.png"
    });

    let sanitized = sanitize_audit_details(value);
    let reason = sanitized["safeReason"].as_str().unwrap_or("");

    assert!(reason.contains("[redacted:path]"));
    assert!(!reason.contains("C:\\Users\\Ivan"));
}

#[test]
fn sanitizes_string_value_containing_unix_path() {
    let value = json!({
        "operation": "importMaterial",
        "safeReason": "Файл /home/ivan/materials/photo.jpg недоступен"
    });

    let sanitized = sanitize_audit_details(value);
    let reason = sanitized["safeReason"].as_str().unwrap_or("");

    assert!(reason.contains("[redacted:path]"));
    assert!(!reason.contains("/home/ivan"));
}

// ── audit_error_details builder ───────────────────────────────────────────

#[test]
fn audit_error_details_are_safe() {
    use crate::audit::audit_metadata::audit_error_details;

    let details = audit_error_details(
        "restoreBackup",
        "ERR_RESTORE_FAILED",
        "Не удалось восстановить данные из backup.",
        Some("backup"),
        Some("backup-1"),
        None,
        None,
        Some("BCK-001"),
    )
    .unwrap();

    let text = details.to_string();

    assert!(text.contains("restoreBackup"));
    assert!(text.contains("ERR_RESTORE_FAILED"));
    assert!(text.contains("BCK-001"));
    assert!(!text.contains("archivePath"));
    assert!(!text.contains("C:\\\\"));
    assert!(!text.contains("/home/"));
}

#[test]
fn audit_error_details_omits_none_optional_fields() {
    use crate::audit::audit_metadata::audit_error_details;

    let details = audit_error_details(
        "exportDocx",
        "ERR_DOCX_EXPORT_FAILED",
        "Не удалось сформировать DOCX-документ.",
        Some("report_draft"),
        None,
        None,
        None,
        None,
    )
    .unwrap();

    let obj = details.as_object().unwrap();

    assert!(!obj.contains_key("entityId"));
    assert!(!obj.contains_key("caseId"));
    assert!(!obj.contains_key("materialCode"));
    assert!(!obj.contains_key("backupCode"));
}

// ── safe_audit_error_details builder ─────────────────────────────────────

#[test]
fn safe_audit_error_details_returns_sanitized_safe_details() {
    use crate::audit::audit_metadata::safe_audit_error_details;

    let details = safe_audit_error_details(
        "restoreBackup",
        "ERR_RESTORE_FAILED",
        "Не удалось восстановить данные из backup C:\\\\Users\\\\Ivan\\\\Desktop\\\\backup.zip",
        Some("backup"),
        Some("backup-id"),
        None,
        None,
        Some("BCK-001"),
    )
    .unwrap();

    let text = details.into_value().to_string();

    assert!(text.contains("restoreBackup"));
    assert!(text.contains("ERR_RESTORE_FAILED"));
    assert!(text.contains("BCK-001"));
    assert!(!text.contains("C:\\\\Users"));
    assert!(!text.contains("Desktop"));
    assert!(!text.contains("backup.zip"));
}
