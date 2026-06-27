use serde::Serialize;
use serde_json::{json, Value};

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

pub fn user_created(user_id: &str, username: &str, role_code: &str) -> Value {
    json!({
        "userId": user_id,
        "username": username,
        "roleCode": role_code
    })
}

pub fn user_updated(user_id: &str, username: &str, changed_fields: &[&str]) -> Value {
    json!({
        "userId": user_id,
        "username": username,
        "changedFields": changed_fields
    })
}

pub fn user_blocked(user_id: &str, username: &str) -> Value {
    json!({
        "userId": user_id,
        "username": username
    })
}

pub fn user_unblocked(user_id: &str, username: &str) -> Value {
    json!({
        "userId": user_id,
        "username": username
    })
}

pub fn user_password_reset(user_id: &str, username: &str, must_change_password: bool) -> Value {
    json!({
        "userId": user_id,
        "username": username,
        "mustChangePassword": must_change_password
    })
}

pub fn user_password_changed(user_id: &str, username: &str) -> Value {
    json!({
        "userId": user_id,
        "username": username
    })
}

pub fn timeline_event_created(event_id: &str, event_code: &str, case_id: &str) -> Value {
    json!({
        "eventId": event_id,
        "eventCode": event_code,
        "caseId": case_id
    })
}

pub fn timeline_event_updated(event_id: &str, event_code: &str, changed_fields: &[&str]) -> Value {
    json!({
        "eventId": event_id,
        "eventCode": event_code,
        "changedFields": changed_fields
    })
}

pub fn timeline_event_deleted(event_id: &str, event_code: &str) -> Value {
    json!({
        "eventId": event_id,
        "eventCode": event_code
    })
}

pub fn timeline_event_report_include_changed(
    event_id: &str,
    event_code: &str,
    include_in_report: bool,
) -> Value {
    json!({
        "eventId": event_id,
        "eventCode": event_code,
        "includeInReport": include_in_report
    })
}

pub fn access_denied(
    reason: &str,
    command: &str,
    actual_role: Option<&str>,
    required_role: Option<&str>,
) -> Value {
    json!({
        "reason": reason,
        "command": command,
        "actualRole": actual_role,
        "requiredRole": required_role
    })
}

pub fn password_change_required(command: &str, actual_role: Option<&str>) -> Value {
    access_denied("password_change_required", command, actual_role, None)
}

pub fn inactive_user(command: &str, actual_role: Option<&str>) -> Value {
    access_denied("inactive_user", command, actual_role, None)
}

pub fn role_denied(command: &str, actual_role: Option<&str>, required_role: &str) -> Value {
    access_denied("role_denied", command, actual_role, Some(required_role))
}

pub fn audit_log_exported(exported_rows: usize, format: &str, filters_applied: bool) -> Value {
    json!({
        "exportedRows": exported_rows,
        "format": format,
        "filtersApplied": filters_applied
    })
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
}
