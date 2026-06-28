use serde::Serialize;
use serde_json::{json, to_value as serde_to_value, Value};

use crate::audit::audit_safe_value::{AuditSafeDetails, AuditSafeSnapshot};
use crate::errors::app_error::AppErrorDto;

pub fn sanitize_sensitive_value(value: Value) -> Value {
    match value {
        Value::Object(map) => {
            let filtered = map
                .into_iter()
                .filter(|(key, _)| {
                    let normalized = key.to_ascii_lowercase();
                    !matches!(
                        normalized.as_str(),
                        "password"
                            | "plainpassword"
                            | "temporarypassword"
                            | "currentpassword"
                            | "newpassword"
                            | "passwordhash"
                            | "password_hash"
                    )
                })
                .map(|(key, val)| (key, sanitize_sensitive_value(val)))
                .collect();

            Value::Object(filtered)
        }
        Value::Array(values) => {
            Value::Array(values.into_iter().map(sanitize_sensitive_value).collect())
        }
        other => other,
    }
}

pub fn to_value<T: Serialize>(value: T) -> Option<Value> {
    serde_json::to_value(value)
        .ok()
        .map(sanitize_sensitive_value)
}

pub fn entity_ref(entity_type: &str, entity_id: &str) -> Value {
    json!({
        "entityType": entity_type,
        "entityId": entity_id
    })
}

pub fn changed_fields(fields: &[&str]) -> Value {
    json!({
        "changedFields": fields
    })
}

pub fn snapshot<T: Serialize>(value: T) -> Option<Value> {
    to_value(value)
}

pub fn old_new<T: Serialize, U: Serialize>(
    old_snapshot: T,
    new_snapshot: U,
) -> (Option<Value>, Option<Value>) {
    (snapshot(old_snapshot), snapshot(new_snapshot))
}

pub fn old_value<T: Serialize>(value: T) -> Option<Value> {
    to_value(value)
}

pub fn new_value<T: Serialize>(value: T) -> Option<Value> {
    to_value(value)
}

// ── Safe snapshot/details builders (compile-time enforced) ─────────────────
//
// Use these helpers to construct `AuditSafeSnapshot` and `AuditSafeDetails`.
// The constructors on those types are restricted to `crate::audit`, so only
// this module (and its siblings) can create safe-typed values.

fn build_snapshot<T: Serialize>(value: T) -> Result<AuditSafeSnapshot, AppErrorDto> {
    let value = serde_to_value(value)
        .map_err(|_| AppErrorDto::internal("Не удалось сформировать audit snapshot."))?;
    Ok(AuditSafeSnapshot::from_checked_value(value))
}

fn build_details<T: Serialize>(value: T) -> Result<AuditSafeDetails, AppErrorDto> {
    use crate::audit::audit_error_sanitizer::sanitize_audit_details;
    let value = serde_to_value(value)
        .map_err(|_| AppErrorDto::internal("Не удалось сформировать audit details."))?;
    let value = sanitize_audit_details(value);
    Ok(AuditSafeDetails::from_checked_value(value))
}

// ── Safe snapshot tuple helpers ────────────────────────────────────────────

pub fn old_new_snapshots(
    old: AuditSafeSnapshot,
    new: AuditSafeSnapshot,
) -> (Option<AuditSafeSnapshot>, Option<AuditSafeSnapshot>) {
    (Some(old), Some(new))
}

pub fn created_snapshot(
    new: AuditSafeSnapshot,
) -> (Option<AuditSafeSnapshot>, Option<AuditSafeSnapshot>) {
    (None, Some(new))
}

pub fn deleted_snapshot(
    old: AuditSafeSnapshot,
) -> (Option<AuditSafeSnapshot>, Option<AuditSafeSnapshot>) {
    (Some(old), None)
}

pub fn no_snapshots() -> (Option<AuditSafeSnapshot>, Option<AuditSafeSnapshot>) {
    (None, None)
}

// ── Typed safe snapshot builders (new service-facing API) ─────────────────
//
// Each `safe_*_snapshot` function builds the corresponding typed struct and
// wraps it in `AuditSafeSnapshot` via `build_snapshot`.  Call sites outside
// `crate::audit` must use these functions; they cannot construct
// `AuditSafeSnapshot` directly.
//
// Note: the structs referenced below are defined later in this file.
// Rust resolves them at compile time regardless of declaration order.

pub fn safe_user_snapshot(
    username: &str,
    display_name: &str,
    role_code: &str,
    is_active: bool,
    must_change_password: bool,
) -> Result<AuditSafeSnapshot, AppErrorDto> {
    build_snapshot(UserAuditSnapshot {
        username,
        display_name,
        role_code,
        is_active,
        must_change_password,
    })
}

pub fn safe_timeline_event_snapshot(
    event_code: &str,
    title: &str,
    description: Option<&str>,
    event_date: &str,
    include_in_report: bool,
) -> Result<AuditSafeSnapshot, AppErrorDto> {
    build_snapshot(TimelineEventAuditSnapshot {
        event_code,
        title,
        description,
        event_date,
        include_in_report,
    })
}

pub fn safe_case_snapshot(
    case_code: &str,
    title: &str,
    subject: Option<&str>,
    status: &str,
    period_start: Option<&str>,
    period_end: Option<&str>,
    description: Option<&str>,
) -> Result<AuditSafeSnapshot, AppErrorDto> {
    build_snapshot(CaseAuditSnapshot {
        case_code,
        title,
        subject,
        status,
        period_start,
        period_end,
        description,
    })
}

pub fn safe_relation_snapshot(
    relation_code: &str,
    source_object_id: &str,
    target_object_id: &str,
    relation_type: &str,
    confidence_level: &str,
    basis: Option<&str>,
    material_id: Option<&str>,
    include_in_report: bool,
) -> Result<AuditSafeSnapshot, AppErrorDto> {
    build_snapshot(RelationAuditSnapshot {
        relation_code,
        source_object_id,
        target_object_id,
        relation_type,
        confidence_level,
        basis,
        material_id,
        include_in_report,
    })
}

pub fn safe_material_snapshot(
    material_code: &str,
    original_file_name: &str,
    material_type: &str,
    file_size_bytes: Option<i64>,
    sha256: Option<&str>,
    integrity_status: Option<&str>,
    description: Option<&str>,
    captured_at: Option<&str>,
    include_in_report: bool,
) -> Result<AuditSafeSnapshot, AppErrorDto> {
    build_snapshot(MaterialAuditSnapshot {
        material_code,
        original_file_name,
        material_type,
        file_size_bytes,
        sha256,
        integrity_status,
        description,
        captured_at,
        include_in_report,
    })
}

pub fn safe_object_snapshot(
    object_code: &str,
    object_type: &str,
    title: &str,
    description: Option<&str>,
    is_key_object: bool,
    include_in_report: Option<bool>,
) -> Result<AuditSafeSnapshot, AppErrorDto> {
    build_snapshot(ObjectAuditSnapshot {
        object_code,
        object_type,
        title,
        description,
        is_key_object,
        include_in_report,
    })
}

#[allow(clippy::too_many_arguments)]
pub fn safe_report_draft_snapshot(
    draft_code: Option<&str>,
    title: &str,
    report_type: &str,
    status: Option<&str>,
    section_count: usize,
    character_count: usize,
    included_materials_count: usize,
    included_objects_count: usize,
    included_relations_count: usize,
    included_events_count: usize,
    exported_at: Option<&str>,
) -> Result<AuditSafeSnapshot, AppErrorDto> {
    build_snapshot(ReportDraftAuditSnapshot {
        draft_code,
        title,
        report_type,
        status,
        section_count,
        character_count,
        included_materials_count,
        included_objects_count,
        included_relations_count,
        included_events_count,
        exported_at,
    })
}

#[allow(clippy::too_many_arguments)]
pub fn safe_backup_snapshot(
    backup_code: Option<&str>,
    backup_type: &str,
    status: &str,
    case_code: Option<&str>,
    app_version: Option<&str>,
    schema_version: Option<&str>,
    archive_size_bytes: Option<i64>,
    archive_sha256: Option<&str>,
    entity_counts: Option<BackupEntityCounts>,
    created_at: Option<&str>,
    completed_at: Option<&str>,
) -> Result<AuditSafeSnapshot, AppErrorDto> {
    build_snapshot(BackupAuditSnapshot {
        backup_code,
        backup_type,
        status,
        case_code,
        app_version,
        schema_version,
        archive_size_bytes,
        archive_sha256,
        entity_counts,
        created_at,
        completed_at,
    })
}

pub fn safe_backup_verification_snapshot(
    backup_code: Option<&str>,
    backup_type: &str,
    is_valid: bool,
    archive_sha256: Option<&str>,
    expected_sha256: Option<&str>,
    errors_count: usize,
    warnings_count: usize,
    checked_at: Option<&str>,
) -> Result<AuditSafeSnapshot, AppErrorDto> {
    build_snapshot(BackupVerificationAuditSnapshot {
        backup_code,
        backup_type,
        is_valid,
        archive_sha256,
        expected_sha256,
        errors_count,
        warnings_count,
        checked_at,
    })
}

#[allow(clippy::too_many_arguments)]
pub fn safe_restore_snapshot(
    backup_code: Option<&str>,
    backup_type: &str,
    status: &str,
    safety_backup_code: Option<&str>,
    app_version: Option<&str>,
    schema_version: Option<&str>,
    restored_counts: Option<BackupEntityCounts>,
    started_at: Option<&str>,
    completed_at: Option<&str>,
) -> Result<AuditSafeSnapshot, AppErrorDto> {
    build_snapshot(RestoreAuditSnapshot {
        backup_code,
        backup_type,
        status,
        safety_backup_code,
        app_version,
        schema_version,
        restored_counts,
        started_at,
        completed_at,
    })
}

pub fn safe_restore_preflight_snapshot(
    backup_code: Option<&str>,
    can_restore: bool,
    backup_type: &str,
    schema_version: i64,
    file_count: usize,
) -> Result<AuditSafeSnapshot, AppErrorDto> {
    build_snapshot(RestorePreflightAuditSnapshot {
        backup_code,
        can_restore,
        backup_type,
        schema_version,
        file_count,
    })
}

pub fn safe_restore_preflight_details(
    operation: &str,
    error_count: usize,
    warning_count: usize,
    verification_error_count: usize,
) -> Result<AuditSafeDetails, AppErrorDto> {
    build_details(RestorePreflightDetails {
        operation,
        error_count,
        warning_count,
        verification_error_count,
    })
}

#[allow(clippy::too_many_arguments)]
pub fn safe_integrity_run_snapshot(
    run_id: &str,
    scope: &str,
    case_code: Option<&str>,
    checked_count: usize,
    ok_count: usize,
    mismatch_count: usize,
    missing_count: usize,
    read_error_count: usize,
    problem_material_codes: Vec<String>,
    started_at: Option<&str>,
    completed_at: Option<&str>,
) -> Result<AuditSafeSnapshot, AppErrorDto> {
    build_snapshot(IntegrityRunAuditSnapshot {
        run_id,
        scope,
        case_code,
        checked_count,
        ok_count,
        mismatch_count,
        missing_count,
        read_error_count,
        problem_material_codes,
        started_at,
        completed_at,
    })
}

pub fn safe_integrity_material_snapshot(
    material_code: &str,
    original_file_name: &str,
    previous_status: Option<&str>,
    current_status: &str,
    expected_sha256: Option<&str>,
    actual_sha256: Option<&str>,
    checked_at: Option<&str>,
) -> Result<AuditSafeSnapshot, AppErrorDto> {
    build_snapshot(IntegrityMaterialAuditSnapshot {
        material_code,
        original_file_name,
        previous_status,
        current_status,
        expected_sha256,
        actual_sha256,
        checked_at,
    })
}

pub fn safe_settings_snapshot(
    changes: Vec<SettingsChangeSnapshot>,
) -> Result<AuditSafeSnapshot, AppErrorDto> {
    build_snapshot(SettingsAuditSnapshot { changes })
}

// ── Typed safe details builder ────────────────────────────────────────────
//
// Replaces raw `json!(...)` in failure audit paths.

pub fn safe_audit_error_details(
    operation: &str,
    error_code: &str,
    safe_reason: &str,
    entity_type: Option<&str>,
    entity_id: Option<&str>,
    case_id: Option<&str>,
    material_code: Option<&str>,
    backup_code: Option<&str>,
) -> Result<AuditSafeDetails, AppErrorDto> {
    build_details(AuditErrorTechnicalDetails {
        operation,
        error_code,
        safe_reason,
        entity_type,
        entity_id,
        case_id,
        material_code,
        backup_code,
    })
}

pub fn push_changed<T: PartialEq>(
    changed: &mut Vec<&str>,
    field: &'static str,
    old_value: &T,
    new_value: &T,
) {
    if old_value != new_value {
        changed.push(field);
    }
}

// SNAPSHOT STANDARD TABLE
// CREATE:
//   old_value = None
//   new_value = snapshot(created_entity)
//   technical_details = operation context
//
// UPDATE:
//   old_value = snapshot(before)
//   new_value = snapshot(after)
//   technical_details = changedFields + operation context
//
// TOGGLE:
//   old_value = snapshot(before)
//   new_value = snapshot(after)
//   technical_details = changed flag + operation context
//
// DELETE / SOFT_DELETE:
//   old_value = snapshot(before)
//   new_value = None
//   technical_details = operation context
//
// ACCESS_DENIED:
//   old_value = None
//   new_value = None
//   technical_details = denial context
//
// PASSWORD_RESET:
//   old_value = safe user snapshot before
//   new_value = safe user snapshot after
//   technical_details = target user + mustChangePassword only

#[derive(Debug, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct UserAuditSnapshot<'a> {
    pub username: &'a str,
    pub display_name: &'a str,
    pub role_code: &'a str,
    pub is_active: bool,
    pub must_change_password: bool,
}

pub fn user_snapshot<'a>(
    username: &'a str,
    display_name: &'a str,
    role_code: &'a str,
    is_active: bool,
    must_change_password: bool,
) -> UserAuditSnapshot<'a> {
    UserAuditSnapshot {
        username,
        display_name,
        role_code,
        is_active,
        must_change_password,
    }
}

#[derive(Debug, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct TimelineEventAuditSnapshot<'a> {
    pub event_code: &'a str,
    pub title: &'a str,
    pub description: Option<&'a str>,
    pub event_date: &'a str,
    pub include_in_report: bool,
}

pub fn timeline_event_snapshot<'a>(
    event_code: &'a str,
    title: &'a str,
    description: Option<&'a str>,
    event_date: &'a str,
    include_in_report: bool,
) -> TimelineEventAuditSnapshot<'a> {
    TimelineEventAuditSnapshot {
        event_code,
        title,
        description,
        event_date,
        include_in_report,
    }
}

#[derive(Debug, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct CaseAuditSnapshot<'a> {
    pub case_code: &'a str,
    pub title: &'a str,
    pub subject: Option<&'a str>,
    pub status: &'a str,
    pub period_start: Option<&'a str>,
    pub period_end: Option<&'a str>,
    pub description: Option<&'a str>,
}

pub fn case_snapshot<'a>(
    case_code: &'a str,
    title: &'a str,
    subject: Option<&'a str>,
    status: &'a str,
    period_start: Option<&'a str>,
    period_end: Option<&'a str>,
    description: Option<&'a str>,
) -> CaseAuditSnapshot<'a> {
    CaseAuditSnapshot {
        case_code,
        title,
        subject,
        status,
        period_start,
        period_end,
        description,
    }
}

#[derive(Debug, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct RelationAuditSnapshot<'a> {
    pub relation_code: &'a str,
    pub source_object_id: &'a str,
    pub target_object_id: &'a str,
    pub relation_type: &'a str,
    pub confidence_level: &'a str,
    pub basis: Option<&'a str>,
    pub material_id: Option<&'a str>,
    pub include_in_report: bool,
}

pub fn relation_snapshot<'a>(
    relation_code: &'a str,
    source_object_id: &'a str,
    target_object_id: &'a str,
    relation_type: &'a str,
    confidence_level: &'a str,
    basis: Option<&'a str>,
    material_id: Option<&'a str>,
    include_in_report: bool,
) -> RelationAuditSnapshot<'a> {
    RelationAuditSnapshot {
        relation_code,
        source_object_id,
        target_object_id,
        relation_type,
        confidence_level,
        basis,
        material_id,
        include_in_report,
    }
}

#[derive(Debug, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct MaterialAuditSnapshot<'a> {
    pub material_code: &'a str,
    pub original_file_name: &'a str,
    pub material_type: &'a str,
    pub file_size_bytes: Option<i64>,
    pub sha256: Option<&'a str>,
    pub integrity_status: Option<&'a str>,
    pub description: Option<&'a str>,
    pub captured_at: Option<&'a str>,
    pub include_in_report: bool,
}

pub fn material_snapshot<'a>(
    material_code: &'a str,
    original_file_name: &'a str,
    material_type: &'a str,
    file_size_bytes: Option<i64>,
    sha256: Option<&'a str>,
    integrity_status: Option<&'a str>,
    description: Option<&'a str>,
    captured_at: Option<&'a str>,
    include_in_report: bool,
) -> MaterialAuditSnapshot<'a> {
    MaterialAuditSnapshot {
        material_code,
        original_file_name,
        material_type,
        file_size_bytes,
        sha256,
        integrity_status,
        description,
        captured_at,
        include_in_report,
    }
}

#[derive(Debug, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct ObjectAuditSnapshot<'a> {
    pub object_code: &'a str,
    pub object_type: &'a str,
    pub title: &'a str,
    pub description: Option<&'a str>,
    pub is_key_object: bool,
    pub include_in_report: Option<bool>,
}

pub fn object_snapshot<'a>(
    object_code: &'a str,
    object_type: &'a str,
    title: &'a str,
    description: Option<&'a str>,
    is_key_object: bool,
    include_in_report: Option<bool>,
) -> ObjectAuditSnapshot<'a> {
    ObjectAuditSnapshot {
        object_code,
        object_type,
        title,
        description,
        is_key_object,
        include_in_report,
    }
}

#[derive(Debug, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct ReportDraftAuditSnapshot<'a> {
    pub draft_code: Option<&'a str>,
    pub title: &'a str,
    pub report_type: &'a str,
    pub status: Option<&'a str>,
    pub section_count: usize,
    pub character_count: usize,
    pub included_materials_count: usize,
    pub included_objects_count: usize,
    pub included_relations_count: usize,
    pub included_events_count: usize,
    pub exported_at: Option<&'a str>,
}

#[allow(clippy::too_many_arguments)]
pub fn report_draft_snapshot<'a>(
    draft_code: Option<&'a str>,
    title: &'a str,
    report_type: &'a str,
    status: Option<&'a str>,
    section_count: usize,
    character_count: usize,
    included_materials_count: usize,
    included_objects_count: usize,
    included_relations_count: usize,
    included_events_count: usize,
    exported_at: Option<&'a str>,
) -> ReportDraftAuditSnapshot<'a> {
    ReportDraftAuditSnapshot {
        draft_code,
        title,
        report_type,
        status,
        section_count,
        character_count,
        included_materials_count,
        included_objects_count,
        included_relations_count,
        included_events_count,
        exported_at,
    }
}

pub struct ReportDraftAuditMetrics {
    pub section_count: usize,
    pub character_count: usize,
    pub included_materials_count: usize,
    pub included_objects_count: usize,
    pub included_relations_count: usize,
    pub included_events_count: usize,
}

pub fn report_draft_metrics_from_content(
    content: &crate::services::report_draft_service::ReportDraftContent,
) -> ReportDraftAuditMetrics {
    let section_count = content.sections.len();

    let character_count = content
        .sections
        .iter()
        .map(|section| section.content.chars().count())
        .sum();

    ReportDraftAuditMetrics {
        section_count,
        character_count,
        included_materials_count: content.materials.len(),
        included_objects_count: content.objects.len(),
        included_relations_count: content.relations.len(),
        included_events_count: content.events.len(),
    }
}

pub fn is_sensitive_setting_key(key: &str) -> bool {
    let normalized = key.to_ascii_lowercase();

    normalized.contains("password")
        || normalized.contains("secret")
        || normalized.contains("token")
        || normalized.contains("key")
        || normalized.contains("credential")
}

/// Returns true for keys whose values are filesystem paths.
/// Driven by the settings catalog's `is_path_like` flag — no separate heuristic list to drift.
pub fn is_path_setting_key(key: &str) -> bool {
    crate::models::settings_catalog::path_like_keys().contains(key)
}

pub fn redact_setting_value(key: &str, value: &Value) -> Value {
    if is_sensitive_setting_key(key) {
        return Value::String("[redacted:secret]".to_string());
    }

    if is_path_setting_key(key) {
        return Value::String("[redacted:path]".to_string());
    }

    value.clone()
}

#[derive(Debug, Serialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct SettingsChangeSnapshot {
    pub key: String,
    pub category: String,
    pub old_value: Value,
    pub new_value: Value,
}

#[derive(Debug, Serialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct SettingsAuditSnapshot {
    pub changes: Vec<SettingsChangeSnapshot>,
}

pub fn settings_snapshot(changes: Vec<SettingsChangeSnapshot>) -> SettingsAuditSnapshot {
    SettingsAuditSnapshot { changes }
}

pub fn setting_change_snapshot(
    key: &str,
    category: &str,
    old_value: &Value,
    new_value: &Value,
) -> SettingsChangeSnapshot {
    SettingsChangeSnapshot {
        key: key.to_string(),
        category: category.to_string(),
        old_value: redact_setting_value(key, old_value),
        new_value: redact_setting_value(key, new_value),
    }
}

pub fn user_created(
    user_id: &str,
    username: &str,
    role_code: &str,
) -> Result<AuditSafeDetails, AppErrorDto> {
    build_details(json!({
        "userId": user_id,
        "username": username,
        "roleCode": role_code
    }))
}

pub fn user_updated(
    user_id: &str,
    username: &str,
    changed_fields: &[&str],
) -> Result<AuditSafeDetails, AppErrorDto> {
    build_details(json!({
        "userId": user_id,
        "username": username,
        "changedFields": changed_fields
    }))
}

pub fn user_blocked(user_id: &str, username: &str) -> Result<AuditSafeDetails, AppErrorDto> {
    build_details(json!({
        "userId": user_id,
        "username": username
    }))
}

pub fn user_unblocked(user_id: &str, username: &str) -> Result<AuditSafeDetails, AppErrorDto> {
    build_details(json!({
        "userId": user_id,
        "username": username
    }))
}

pub fn user_password_reset(
    user_id: &str,
    username: &str,
    must_change_password: bool,
) -> Result<AuditSafeDetails, AppErrorDto> {
    build_details(json!({
        "userId": user_id,
        "username": username,
        "mustChangePassword": must_change_password
    }))
}

pub fn user_password_changed(
    user_id: &str,
    username: &str,
) -> Result<AuditSafeDetails, AppErrorDto> {
    build_details(json!({
        "userId": user_id,
        "username": username
    }))
}

pub fn timeline_event_created(
    event_id: &str,
    event_code: &str,
    case_id: &str,
) -> Result<AuditSafeDetails, AppErrorDto> {
    build_details(json!({
        "eventId": event_id,
        "eventCode": event_code,
        "caseId": case_id
    }))
}

pub fn timeline_event_updated(
    event_id: &str,
    event_code: &str,
    changed_fields: &[&str],
) -> Result<AuditSafeDetails, AppErrorDto> {
    build_details(json!({
        "eventId": event_id,
        "eventCode": event_code,
        "changedFields": changed_fields
    }))
}

pub fn timeline_event_deleted(
    event_id: &str,
    event_code: &str,
) -> Result<AuditSafeDetails, AppErrorDto> {
    build_details(json!({
        "eventId": event_id,
        "eventCode": event_code
    }))
}

pub fn timeline_event_report_include_changed(
    event_id: &str,
    event_code: &str,
    include_in_report: bool,
) -> Result<AuditSafeDetails, AppErrorDto> {
    build_details(json!({
        "eventId": event_id,
        "eventCode": event_code,
        "includeInReport": include_in_report
    }))
}

pub fn case_created(
    case_id: &str,
    case_code: &str,
    title: &str,
) -> Result<AuditSafeDetails, AppErrorDto> {
    build_details(json!({
        "caseId": case_id,
        "caseCode": case_code,
        "title": title
    }))
}

pub fn case_updated(
    case_id: &str,
    case_code: &str,
    changed_fields: &[&str],
) -> Result<AuditSafeDetails, AppErrorDto> {
    build_details(json!({
        "caseId": case_id,
        "caseCode": case_code,
        "changedFields": changed_fields
    }))
}

pub fn case_status_changed(
    case_id: &str,
    case_code: &str,
    old_status: &str,
    new_status: &str,
) -> Result<AuditSafeDetails, AppErrorDto> {
    build_details(json!({
        "caseId": case_id,
        "caseCode": case_code,
        "oldStatus": old_status,
        "newStatus": new_status,
        "changedFields": ["status"]
    }))
}

pub fn relation_created(
    relation_id: &str,
    relation_code: &str,
    source_object_id: &str,
    target_object_id: &str,
) -> Result<AuditSafeDetails, AppErrorDto> {
    build_details(json!({
        "relationId": relation_id,
        "relationCode": relation_code,
        "sourceObjectId": source_object_id,
        "targetObjectId": target_object_id
    }))
}

pub fn relation_updated(
    relation_id: &str,
    relation_code: &str,
    changed_fields: &[&str],
) -> Result<AuditSafeDetails, AppErrorDto> {
    build_details(json!({
        "relationId": relation_id,
        "relationCode": relation_code,
        "changedFields": changed_fields
    }))
}

pub fn relation_report_include_changed(
    relation_id: &str,
    relation_code: &str,
    include_in_report: bool,
) -> Result<AuditSafeDetails, AppErrorDto> {
    build_details(json!({
        "relationId": relation_id,
        "relationCode": relation_code,
        "includeInReport": include_in_report,
        "changedFields": ["includeInReport"]
    }))
}

pub fn relation_deleted(
    relation_id: &str,
    relation_code: &str,
) -> Result<AuditSafeDetails, AppErrorDto> {
    build_details(json!({
        "relationId": relation_id,
        "relationCode": relation_code
    }))
}

pub fn material_imported(
    material_id: &str,
    material_code: &str,
    original_file_name: &str,
) -> Result<AuditSafeDetails, AppErrorDto> {
    build_details(json!({
        "materialId": material_id,
        "materialCode": material_code,
        "originalFileName": original_file_name
    }))
}

pub fn material_updated(
    material_id: &str,
    material_code: &str,
    changed_fields: &[&str],
) -> Result<AuditSafeDetails, AppErrorDto> {
    build_details(json!({
        "materialId": material_id,
        "materialCode": material_code,
        "changedFields": changed_fields
    }))
}

pub fn material_report_include_changed(
    material_id: &str,
    material_code: &str,
    include_in_report: bool,
) -> Result<AuditSafeDetails, AppErrorDto> {
    build_details(json!({
        "materialId": material_id,
        "materialCode": material_code,
        "includeInReport": include_in_report,
        "changedFields": ["includeInReport"]
    }))
}

pub fn material_hash_verified(
    material_id: &str,
    material_code: &str,
    integrity_status: &str,
) -> Result<AuditSafeDetails, AppErrorDto> {
    build_details(json!({
        "materialId": material_id,
        "materialCode": material_code,
        "integrityStatus": integrity_status,
        "changedFields": ["integrityStatus"]
    }))
}

pub fn material_deleted(
    material_id: &str,
    material_code: &str,
) -> Result<AuditSafeDetails, AppErrorDto> {
    build_details(json!({
        "materialId": material_id,
        "materialCode": material_code
    }))
}

pub fn object_created(
    object_id: &str,
    object_code: &str,
    object_type: &str,
) -> Result<AuditSafeDetails, AppErrorDto> {
    build_details(json!({
        "objectId": object_id,
        "objectCode": object_code,
        "objectType": object_type
    }))
}

pub fn object_updated(
    object_id: &str,
    object_code: &str,
    changed_fields: &[&str],
) -> Result<AuditSafeDetails, AppErrorDto> {
    build_details(json!({
        "objectId": object_id,
        "objectCode": object_code,
        "changedFields": changed_fields
    }))
}

pub fn object_material_links_changed(
    object_id: &str,
    object_code: &str,
    material_ids: &[String],
) -> Result<AuditSafeDetails, AppErrorDto> {
    build_details(json!({
        "objectId": object_id,
        "objectCode": object_code,
        "materialIds": material_ids,
        "changedFields": ["materialLinks"]
    }))
}

pub fn object_key_flag_changed(
    object_id: &str,
    object_code: &str,
    is_key_object: bool,
) -> Result<AuditSafeDetails, AppErrorDto> {
    build_details(json!({
        "objectId": object_id,
        "objectCode": object_code,
        "isKeyObject": is_key_object,
        "changedFields": ["isKeyObject"]
    }))
}

pub fn object_deleted(object_id: &str, object_code: &str) -> Result<AuditSafeDetails, AppErrorDto> {
    build_details(json!({
        "objectId": object_id,
        "objectCode": object_code
    }))
}

pub fn report_draft_generated(
    draft_id: &str,
    case_id: &str,
    report_type: &str,
) -> Result<AuditSafeDetails, AppErrorDto> {
    build_details(json!({
        "draftId": draft_id,
        "caseId": case_id,
        "reportType": report_type
    }))
}

pub fn report_draft_updated(
    draft_id: &str,
    case_id: &str,
    changed_fields: &[&str],
) -> Result<AuditSafeDetails, AppErrorDto> {
    build_details(json!({
        "draftId": draft_id,
        "caseId": case_id,
        "changedFields": changed_fields
    }))
}

pub fn report_draft_validated(
    draft_id: &str,
    case_id: &str,
    is_valid: bool,
    warnings_count: usize,
    errors_count: usize,
) -> Result<AuditSafeDetails, AppErrorDto> {
    build_details(json!({
        "draftId": draft_id,
        "caseId": case_id,
        "isValid": is_valid,
        "warningsCount": warnings_count,
        "errorsCount": errors_count
    }))
}

pub fn report_draft_deleted(
    draft_id: &str,
    case_id: &str,
) -> Result<AuditSafeDetails, AppErrorDto> {
    build_details(json!({
        "draftId": draft_id,
        "caseId": case_id
    }))
}

pub fn settings_updated(
    changed_keys: &[String],
    categories: &[String],
) -> Result<AuditSafeDetails, AppErrorDto> {
    build_details(json!({
        "changedKeys": changed_keys,
        "categories": categories,
        "changedFields": changed_keys
    }))
}

pub fn settings_reset_to_default(changed_keys: &[String]) -> Result<AuditSafeDetails, AppErrorDto> {
    build_details(json!({
        "changedKeys": changed_keys,
        "changedFields": changed_keys
    }))
}

pub fn access_denied(
    reason: &str,
    command: &str,
    actual_role: Option<&str>,
    required_role: Option<&str>,
) -> Result<AuditSafeDetails, AppErrorDto> {
    build_details(json!({
        "reason": reason,
        "command": command,
        "actualRole": actual_role,
        "requiredRole": required_role
    }))
}

pub fn password_change_required(
    command: &str,
    actual_role: Option<&str>,
) -> Result<AuditSafeDetails, AppErrorDto> {
    access_denied("password_change_required", command, actual_role, None)
}

pub fn inactive_user(
    command: &str,
    actual_role: Option<&str>,
) -> Result<AuditSafeDetails, AppErrorDto> {
    access_denied("inactive_user", command, actual_role, None)
}

pub fn role_denied(
    command: &str,
    actual_role: Option<&str>,
    required_role: &str,
) -> Result<AuditSafeDetails, AppErrorDto> {
    access_denied("role_denied", command, actual_role, Some(required_role))
}

pub fn access_denied_details(
    operation: &'static str,
    reason: &'static str,
    policy_key: Option<&'static str>,
) -> Result<AuditSafeDetails, AppErrorDto> {
    let value = match policy_key {
        Some(policy_key) => json!({
            "operation": operation,
            "reason": reason,
            "policyKey": policy_key
        }),
        None => json!({
            "operation": operation,
            "reason": reason
        }),
    };

    build_details(value)
}

pub fn policy_denied(
    command: &str,
    actual_role: &str,
    policy: &str,
) -> Result<AuditSafeDetails, AppErrorDto> {
    build_details(json!({
        "reason": "policy_denied",
        "command": command,
        "actualRole": actual_role,
        "policy": policy
    }))
}

pub fn audit_log_exported(
    exported_rows: usize,
    format: &str,
    filters_applied: bool,
) -> Result<AuditSafeDetails, AppErrorDto> {
    build_details(json!({
        "exportedRows": exported_rows,
        "format": format,
        "filtersApplied": filters_applied
    }))
}

#[derive(Debug, Serialize, Clone, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct BackupEntityCounts {
    pub cases: usize,
    pub materials: usize,
    pub objects: usize,
    pub relations: usize,
    pub events: usize,
    pub report_drafts: usize,
    pub audit_logs: usize,
    pub integrity_results: usize,
}

#[derive(Debug, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct BackupAuditSnapshot<'a> {
    pub backup_code: Option<&'a str>,
    pub backup_type: &'a str,
    pub status: &'a str,
    pub case_code: Option<&'a str>,
    pub app_version: Option<&'a str>,
    pub schema_version: Option<&'a str>,
    pub archive_size_bytes: Option<i64>,
    pub archive_sha256: Option<&'a str>,
    pub entity_counts: Option<BackupEntityCounts>,
    pub created_at: Option<&'a str>,
    pub completed_at: Option<&'a str>,
}

#[allow(clippy::too_many_arguments)]
pub fn backup_snapshot<'a>(
    backup_code: Option<&'a str>,
    backup_type: &'a str,
    status: &'a str,
    case_code: Option<&'a str>,
    app_version: Option<&'a str>,
    schema_version: Option<&'a str>,
    archive_size_bytes: Option<i64>,
    archive_sha256: Option<&'a str>,
    entity_counts: Option<BackupEntityCounts>,
    created_at: Option<&'a str>,
    completed_at: Option<&'a str>,
) -> BackupAuditSnapshot<'a> {
    BackupAuditSnapshot {
        backup_code,
        backup_type,
        status,
        case_code,
        app_version,
        schema_version,
        archive_size_bytes,
        archive_sha256,
        entity_counts,
        created_at,
        completed_at,
    }
}

pub fn backup_created(
    backup_id: &str,
    backup_type: &str,
    case_id: Option<&str>,
    archive_size_bytes: Option<i64>,
) -> Result<AuditSafeDetails, AppErrorDto> {
    build_details(json!({
        "backupId": backup_id,
        "backupType": backup_type,
        "caseId": case_id,
        "archiveSizeBytes": archive_size_bytes
    }))
}

pub fn backup_verified(
    backup_id: &str,
    backup_type: &str,
    is_valid: bool,
    errors_count: usize,
    warnings_count: usize,
) -> Result<AuditSafeDetails, AppErrorDto> {
    build_details(json!({
        "backupId": backup_id,
        "backupType": backup_type,
        "isValid": is_valid,
        "errorsCount": errors_count,
        "warningsCount": warnings_count
    }))
}

pub fn backup_restore_started(
    backup_id: &str,
    backup_type: &str,
    safety_backup_id: Option<&str>,
) -> Result<AuditSafeDetails, AppErrorDto> {
    build_details(json!({
        "backupId": backup_id,
        "backupType": backup_type,
        "safetyBackupId": safety_backup_id
    }))
}

pub fn backup_restore_completed(
    backup_id: &str,
    backup_type: &str,
    restored_counts: Option<&BackupEntityCounts>,
) -> Result<AuditSafeDetails, AppErrorDto> {
    build_details(json!({
        "backupId": backup_id,
        "backupType": backup_type,
        "restoredCounts": restored_counts
    }))
}

#[derive(Debug, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct BackupVerificationAuditSnapshot<'a> {
    pub backup_code: Option<&'a str>,
    pub backup_type: &'a str,
    pub is_valid: bool,
    pub archive_sha256: Option<&'a str>,
    pub expected_sha256: Option<&'a str>,
    pub errors_count: usize,
    pub warnings_count: usize,
    pub checked_at: Option<&'a str>,
}

pub fn backup_verification_snapshot<'a>(
    backup_code: Option<&'a str>,
    backup_type: &'a str,
    is_valid: bool,
    archive_sha256: Option<&'a str>,
    expected_sha256: Option<&'a str>,
    errors_count: usize,
    warnings_count: usize,
    checked_at: Option<&'a str>,
) -> BackupVerificationAuditSnapshot<'a> {
    BackupVerificationAuditSnapshot {
        backup_code,
        backup_type,
        is_valid,
        archive_sha256,
        expected_sha256,
        errors_count,
        warnings_count,
        checked_at,
    }
}

#[derive(Debug, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct RestorePreflightAuditSnapshot<'a> {
    pub backup_code: Option<&'a str>,
    pub can_restore: bool,
    pub backup_type: &'a str,
    pub schema_version: i64,
    pub file_count: usize,
}

#[derive(Debug, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct RestorePreflightDetails<'a> {
    pub operation: &'a str,
    pub error_count: usize,
    pub warning_count: usize,
    pub verification_error_count: usize,
}

#[derive(Debug, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct RestoreAuditSnapshot<'a> {
    pub backup_code: Option<&'a str>,
    pub backup_type: &'a str,
    pub status: &'a str,
    pub safety_backup_code: Option<&'a str>,
    pub app_version: Option<&'a str>,
    pub schema_version: Option<&'a str>,
    pub restored_counts: Option<BackupEntityCounts>,
    pub started_at: Option<&'a str>,
    pub completed_at: Option<&'a str>,
}

#[allow(clippy::too_many_arguments)]
pub fn restore_snapshot<'a>(
    backup_code: Option<&'a str>,
    backup_type: &'a str,
    status: &'a str,
    safety_backup_code: Option<&'a str>,
    app_version: Option<&'a str>,
    schema_version: Option<&'a str>,
    restored_counts: Option<BackupEntityCounts>,
    started_at: Option<&'a str>,
    completed_at: Option<&'a str>,
) -> RestoreAuditSnapshot<'a> {
    RestoreAuditSnapshot {
        backup_code,
        backup_type,
        status,
        safety_backup_code,
        app_version,
        schema_version,
        restored_counts,
        started_at,
        completed_at,
    }
}

#[derive(Debug, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct IntegrityRunAuditSnapshot<'a> {
    pub run_id: &'a str,
    pub scope: &'a str,
    pub case_code: Option<&'a str>,
    pub checked_count: usize,
    pub ok_count: usize,
    pub mismatch_count: usize,
    pub missing_count: usize,
    pub read_error_count: usize,
    pub problem_material_codes: Vec<String>,
    pub started_at: Option<&'a str>,
    pub completed_at: Option<&'a str>,
}

#[allow(clippy::too_many_arguments)]
pub fn integrity_run_snapshot<'a>(
    run_id: &'a str,
    scope: &'a str,
    case_code: Option<&'a str>,
    checked_count: usize,
    ok_count: usize,
    mismatch_count: usize,
    missing_count: usize,
    read_error_count: usize,
    problem_material_codes: Vec<String>,
    started_at: Option<&'a str>,
    completed_at: Option<&'a str>,
) -> IntegrityRunAuditSnapshot<'a> {
    IntegrityRunAuditSnapshot {
        run_id,
        scope,
        case_code,
        checked_count,
        ok_count,
        mismatch_count,
        missing_count,
        read_error_count,
        problem_material_codes,
        started_at,
        completed_at,
    }
}

pub type IntegrityResultSummarySnapshot<'a> = IntegrityRunAuditSnapshot<'a>;

pub fn capped_codes(codes: Vec<String>, limit: usize) -> Vec<String> {
    codes.into_iter().take(limit).collect()
}

#[derive(Debug, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct IntegrityMaterialAuditSnapshot<'a> {
    pub material_code: &'a str,
    pub original_file_name: &'a str,
    pub previous_status: Option<&'a str>,
    pub current_status: &'a str,
    pub expected_sha256: Option<&'a str>,
    pub actual_sha256: Option<&'a str>,
    pub checked_at: Option<&'a str>,
}

pub fn integrity_material_snapshot<'a>(
    material_code: &'a str,
    original_file_name: &'a str,
    previous_status: Option<&'a str>,
    current_status: &'a str,
    expected_sha256: Option<&'a str>,
    actual_sha256: Option<&'a str>,
    checked_at: Option<&'a str>,
) -> IntegrityMaterialAuditSnapshot<'a> {
    IntegrityMaterialAuditSnapshot {
        material_code,
        original_file_name,
        previous_status,
        current_status,
        expected_sha256,
        actual_sha256,
        checked_at,
    }
}

pub fn integrity_check_completed(
    run_id: &str,
    scope: &str,
    case_id: Option<&str>,
    checked_count: usize,
    problem_count: usize,
) -> Result<AuditSafeDetails, AppErrorDto> {
    build_details(json!({
        "runId": run_id,
        "scope": scope,
        "caseId": case_id,
        "checkedCount": checked_count,
        "problemCount": problem_count
    }))
}

pub fn integrity_problem_detected(
    run_id: &str,
    scope: &str,
    problem_material_codes: &[String],
    mismatch_count: usize,
    missing_count: usize,
    read_error_count: usize,
) -> Result<AuditSafeDetails, AppErrorDto> {
    build_details(json!({
        "runId": run_id,
        "scope": scope,
        "problemMaterialCodes": problem_material_codes,
        "mismatchCount": mismatch_count,
        "missingCount": missing_count,
        "readErrorCount": read_error_count
    }))
}

pub fn integrity_material_verified(
    material_id: &str,
    material_code: &str,
    previous_status: Option<&str>,
    current_status: &str,
) -> Result<AuditSafeDetails, AppErrorDto> {
    build_details(json!({
        "materialId": material_id,
        "materialCode": material_code,
        "previousStatus": previous_status,
        "currentStatus": current_status,
        "changedFields": ["integrityStatus"]
    }))
}

// ── Safe failure details builder ─────────────────────────────────────────
//
// Use this instead of `json!({ "error": err.to_string() })` in failure
// audit events.  All field values are safe by construction; the result is
// additionally passed through `sanitize_audit_details` as a defence-in-depth
// measure.

#[derive(Debug, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct AuditErrorTechnicalDetails<'a> {
    pub operation: &'a str,
    pub error_code: &'a str,
    pub safe_reason: &'a str,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub entity_type: Option<&'a str>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub entity_id: Option<&'a str>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub case_id: Option<&'a str>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub material_code: Option<&'a str>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub backup_code: Option<&'a str>,
}

/// Build safe `technical_details` for a failure audit event.
///
/// All path / secret / content keys are absent by design.
/// The resulting value is additionally sanitized as a defence-in-depth measure.
///
/// # Deprecated
/// Prefer [`safe_audit_error_details`] which returns `AuditSafeDetails` and
/// enforces the type boundary at the audit service layer.
pub fn audit_error_details(
    operation: &str,
    error_code: &str,
    safe_reason: &str,
    entity_type: Option<&str>,
    entity_id: Option<&str>,
    case_id: Option<&str>,
    material_code: Option<&str>,
    backup_code: Option<&str>,
) -> Result<serde_json::Value, serde_json::Error> {
    // Delegate to the safe builder and unwrap the inner Value.
    // The only error path is serde serialization failure, which we remap.
    safe_audit_error_details(
        operation,
        error_code,
        safe_reason,
        entity_type,
        entity_id,
        case_id,
        material_code,
        backup_code,
    )
    .map(|safe| safe.into_value())
    .map_err(|_| {
        // AppErrorDto is not serde::Error; produce a dummy serde error.
        serde_json::from_str::<serde_json::Value>("!invalid").unwrap_err()
    })
}

// ── Safety backup audit builders ───────────────────────────────────────────

pub fn safe_safety_backup_snapshot(
    safety_backup_code: &str,
    safety_archive_sha256: &str,
    safety_file_size: i64,
    restore_target_backup_code: Option<&str>,
    restore_target_archive_sha256: &str,
) -> Result<AuditSafeSnapshot, AppErrorDto> {
    build_snapshot(serde_json::json!({
        "backupType": "safety",
        "safetyReason": "before_restore",
        "safetyBackupCode": safety_backup_code,
        "safetyArchiveSha256": safety_archive_sha256,
        "safetyFileSize": safety_file_size,
        "restoreTargetBackupCode": restore_target_backup_code,
        "restoreTargetArchiveSha256": restore_target_archive_sha256
    }))
}

pub fn safe_safety_backup_details(
    safety_reason: &str,
    can_continue_to_restore: bool,
) -> Result<AuditSafeDetails, AppErrorDto> {
    build_details(serde_json::json!({
        "operation": "create_restore_safety_backup",
        "safetyReason": safety_reason,
        "canContinueToRestore": can_continue_to_restore
    }))
}

pub fn safe_safety_backup_failed_snapshot(
    restore_target_backup_code: Option<&str>,
) -> Result<AuditSafeSnapshot, AppErrorDto> {
    build_snapshot(serde_json::json!({
        "backupType": "safety",
        "safetyReason": "before_restore",
        "restoreTargetBackupCode": restore_target_backup_code
    }))
}

pub fn safe_safety_backup_failed_details(
    safety_reason: &str,
    error_code: &str,
) -> Result<AuditSafeDetails, AppErrorDto> {
    build_details(serde_json::json!({
        "operation": "create_restore_safety_backup",
        "safetyReason": safety_reason,
        "errorCode": error_code
    }))
}

// ── Restore execution audit builders ────────────────────────────────────────
//
// These builders produce safe audit entries for the restore execution flow.
// They deliberately exclude all filesystem paths, ZIP contents, SQLite dumps,
// and raw error text. Only operation-level metadata is recorded.

pub fn restore_started_snapshot(
    operation_id: &str,
    restore_backup_code: Option<&str>,
    restore_archive_sha256: &str,
    safety_backup_code: &str,
    safety_archive_sha256: &str,
) -> Result<AuditSafeSnapshot, AppErrorDto> {
    build_snapshot(serde_json::json!({
        "restoreOperationId": operation_id,
        "restoreBackupCode": restore_backup_code,
        "restoreArchiveSha256": restore_archive_sha256,
        "safetyBackupCode": safety_backup_code,
        "safetyArchiveSha256": safety_archive_sha256
    }))
}

pub fn restore_started_details(operation: &str) -> Result<AuditSafeDetails, AppErrorDto> {
    build_details(serde_json::json!({
        "operation": operation,
        "phase": "started"
    }))
}

pub fn restore_completed_snapshot(
    operation_id: &str,
    restore_backup_code: Option<&str>,
    restore_archive_sha256: &str,
    safety_backup_code: &str,
    safety_archive_sha256: &str,
) -> Result<AuditSafeSnapshot, AppErrorDto> {
    build_snapshot(serde_json::json!({
        "restoreOperationId": operation_id,
        "restoreBackupCode": restore_backup_code,
        "restoreArchiveSha256": restore_archive_sha256,
        "safetyBackupCode": safety_backup_code,
        "safetyArchiveSha256": safety_archive_sha256,
        "requiresRestart": true
    }))
}

pub fn restore_completed_details(requires_restart: bool) -> Result<AuditSafeDetails, AppErrorDto> {
    build_details(serde_json::json!({
        "operation": "full_restore",
        "phase": "completed",
        "requiresRestart": requires_restart
    }))
}

pub fn restore_failed_snapshot(
    operation_id: &str,
    restore_backup_code: Option<&str>,
) -> Result<AuditSafeSnapshot, AppErrorDto> {
    build_snapshot(serde_json::json!({
        "restoreOperationId": operation_id,
        "restoreBackupCode": restore_backup_code
    }))
}

pub fn restore_failed_details(error_code: &str) -> Result<AuditSafeDetails, AppErrorDto> {
    build_details(serde_json::json!({
        "operation": "full_restore",
        "phase": "failed",
        "errorCode": error_code
    }))
}

// ── Restore recovery audit builders ─────────────────────────────────────────
//
// These builders produce safe audit entries for the restore startup recovery flow.
// They deliberately exclude all filesystem paths, raw io errors, and file content.

pub fn restore_recovery_snapshot(
    operation_id: Option<&str>,
    phase: Option<&str>,
    restore_backup_code: Option<&str>,
    safety_backup_code: Option<&str>,
    last_error_code: Option<&str>,
) -> Result<AuditSafeSnapshot, AppErrorDto> {
    build_snapshot(serde_json::json!({
        "restoreOperationId": operation_id,
        "phase": phase,
        "restoreBackupCode": restore_backup_code,
        "safetyBackupCode": safety_backup_code,
        "lastErrorCode": last_error_code
    }))
}

pub fn restore_recovery_details(action: &str) -> Result<AuditSafeDetails, AppErrorDto> {
    build_details(serde_json::json!({
        "operation": "restore_recovery",
        "action": action
    }))
}

pub fn restore_recovery_resolved_snapshot(action: &str) -> Result<AuditSafeSnapshot, AppErrorDto> {
    build_snapshot(serde_json::json!({
        "resolved": true,
        "action": action
    }))
}

#[cfg(test)]
mod tests {
    use super::*;
    use serde_json::{json, to_value};

    #[test]
    fn sanitize_sensitive_value_removes_password_fields() {
        let value = json!({
            "username": "analyst1",
            "temporaryPassword": "secret",
            "nested": {
                "passwordHash": "hash",
                "role": "analyst"
            }
        });

        let sanitized = sanitize_sensitive_value(value);

        assert_eq!(sanitized["username"], "analyst1");
        assert!(sanitized.get("temporaryPassword").is_none());
        assert!(sanitized["nested"].get("passwordHash").is_none());
        assert_eq!(sanitized["nested"]["role"], "analyst");
    }

    #[test]
    fn user_snapshot_does_not_contain_password_fields() {
        let snapshot_val = user_snapshot("analyst1", "Analyst One", "analyst", true, false);

        let value = to_value(snapshot_val).unwrap();

        assert_eq!(value["username"], "analyst1");
        assert_eq!(value["roleCode"], "analyst");

        assert!(value.get("password").is_none());
        assert!(value.get("passwordHash").is_none());
        assert!(value.get("password_hash").is_none());
        assert!(value.get("temporaryPassword").is_none());
        assert!(value.get("currentPassword").is_none());
        assert!(value.get("newPassword").is_none());
    }

    #[test]
    fn push_changed_adds_only_changed_fields() {
        let mut changed = Vec::new();

        push_changed(&mut changed, "displayName", &"Old", &"New");
        push_changed(&mut changed, "roleCode", &"analyst", &"analyst");

        assert_eq!(changed, vec!["displayName"]);
    }

    #[test]
    fn case_snapshot_contains_only_business_fields() {
        let snapshot_val = case_snapshot(
            "CASE-001",
            "Тестовое дело",
            Some("Иванов И.И."),
            "in_progress",
            Some("2026-01-01"),
            Some("2026-01-15"),
            Some("Описание"),
        );

        let value = to_value(snapshot_val).unwrap();

        assert_eq!(value["caseCode"], "CASE-001");
        assert_eq!(value["title"], "Тестовое дело");
        assert_eq!(value["status"], "in_progress");

        assert!(value.get("createdAt").is_none());
        assert!(value.get("updatedAt").is_none());
        assert!(value.get("createdByUserId").is_none());
    }

    #[test]
    fn relation_snapshot_contains_expected_business_fields() {
        let snapshot_val = relation_snapshot(
            "REL-001",
            "obj-source",
            "obj-target",
            "contact",
            "high",
            Some("Основание связи"),
            Some("mat-001"),
            true,
        );

        let value = to_value(snapshot_val).unwrap();

        assert_eq!(value["relationCode"], "REL-001");
        assert_eq!(value["relationType"], "contact");
        assert_eq!(value["confidenceLevel"], "high");
        assert_eq!(value["includeInReport"], true);

        assert!(value.get("createdAt").is_none());
        assert!(value.get("updatedAt").is_none());
        assert!(value.get("deletedAt").is_none());
    }

    #[test]
    fn material_snapshot_does_not_expose_file_paths() {
        let snapshot = material_snapshot(
            "MAT-001",
            "photo.png",
            "image",
            Some(245760),
            Some("a3f1"),
            Some("ok"),
            Some("Описание"),
            Some("2026-01-15T10:00:00"),
            true,
        );

        let value = to_value(snapshot).unwrap();

        assert_eq!(value["materialCode"], "MAT-001");
        assert_eq!(value["originalFileName"], "photo.png");
        assert_eq!(value["materialType"], "image");
        assert_eq!(value["includeInReport"], true);

        assert!(value.get("originalPath").is_none());
        assert!(value.get("storedPath").is_none());
        assert!(value.get("thumbnailPath").is_none());
        assert!(value.get("filePath").is_none());
    }

    #[test]
    fn object_snapshot_contains_only_business_fields() {
        let snapshot = object_snapshot(
            "P-001",
            "person",
            "Иванов И.И.",
            Some("Описание объекта"),
            true,
            None,
        );

        let value = to_value(snapshot).unwrap();

        assert_eq!(value["objectCode"], "P-001");
        assert_eq!(value["objectType"], "person");
        assert_eq!(value["title"], "Иванов И.И.");
        assert_eq!(value["isKeyObject"], true);

        assert!(value.get("createdAt").is_none());
        assert!(value.get("updatedAt").is_none());
        assert!(value.get("deletedAt").is_none());
        assert!(value.get("graphPosition").is_none());
    }

    #[test]
    fn report_draft_snapshot_does_not_expose_full_content() {
        let snapshot = report_draft_snapshot(
            Some("RPT-001"),
            "Аналитическая справка",
            "analytical_report",
            Some("draft"),
            8,
            12000,
            4,
            6,
            3,
            5,
            None,
        );

        let value = to_value(snapshot).unwrap();

        assert_eq!(value["draftCode"], "RPT-001");
        assert_eq!(value["title"], "Аналитическая справка");
        assert_eq!(value["sectionCount"], 8);
        assert_eq!(value["characterCount"], 12000);

        assert!(value.get("content").is_none());
        assert!(value.get("sections").is_none());
        assert!(value.get("body").is_none());
        assert!(value.get("html").is_none());
        assert!(value.get("markdown").is_none());
    }

    #[test]
    fn setting_change_snapshot_redacts_sensitive_values() {
        let old_value = Value::String("old-secret".to_string());
        let new_value = Value::String("new-secret".to_string());

        let change = setting_change_snapshot("api_token", "integration", &old_value, &new_value);

        let value = to_value(change).unwrap();

        assert_eq!(value["oldValue"], "[redacted:secret]");
        assert_eq!(value["newValue"], "[redacted:secret]");
    }

    #[test]
    fn setting_change_snapshot_redacts_paths() {
        // Keys must be real catalog keys with is_path_like = true
        // (backup.default_dir and docx.default_export_dir)
        let old_value = Value::String("C:\\Users\\Admin\\Documents\\CaseGraph".to_string());
        let new_value = Value::String("D:\\CaseGraphData".to_string());

        let change =
            setting_change_snapshot("backup.default_dir", "backup", &old_value, &new_value);

        let value = to_value(change).unwrap();

        assert_eq!(value["oldValue"], "[redacted:path]");
        assert_eq!(value["newValue"], "[redacted:path]");
    }

    #[test]
    fn backup_snapshot_does_not_expose_paths_or_file_list() {
        let snapshot = backup_snapshot(
            Some("BCK-001"),
            "full",
            "completed",
            None,
            Some("0.1.0"),
            Some("202606270001"),
            Some(1024),
            Some("abc123"),
            None,
            Some("2026-06-27T10:00:00"),
            Some("2026-06-27T10:01:00"),
        );

        let value = serde_json::to_value(snapshot).unwrap();

        assert_eq!(value["backupCode"], "BCK-001");
        assert_eq!(value["backupType"], "full");

        assert!(value.get("backupPath").is_none());
        assert!(value.get("archivePath").is_none());
        assert!(value.get("storageRoot").is_none());
        assert!(value.get("fileList").is_none());
        assert!(value.get("zipBytes").is_none());
        assert!(value.get("sqliteDump").is_none());
    }

    #[test]
    fn integrity_run_snapshot_contains_summary_not_paths() {
        let snapshot = integrity_run_snapshot(
            "run-1",
            "case",
            Some("CASE-001"),
            10,
            8,
            1,
            1,
            0,
            vec!["MAT-001".to_string(), "MAT-002".to_string()],
            Some("2026-06-27T10:00:00"),
            Some("2026-06-27T10:01:00"),
        );

        let value = serde_json::to_value(snapshot).unwrap();

        assert_eq!(value["checkedCount"], 10);
        assert_eq!(value["mismatchCount"], 1);
        assert_eq!(value["missingCount"], 1);

        assert!(value.get("originalPath").is_none());
        assert!(value.get("storedPath").is_none());
        assert!(value.get("absolutePath").is_none());
        assert!(value.get("fileContent").is_none());
    }

    #[test]
    fn capped_codes_limits_problem_material_codes() {
        let codes = (0..100)
            .map(|index| format!("MAT-{index:03}"))
            .collect::<Vec<_>>();

        let capped = capped_codes(codes, 50);

        assert_eq!(capped.len(), 50);
        assert_eq!(capped[0], "MAT-000");
        assert_eq!(capped[49], "MAT-049");
    }
}
