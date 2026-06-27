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

pub fn old_new<T: Serialize, U: Serialize>(
    old_val: T,
    new_val: U,
) -> (Option<Value>, Option<Value>) {
    (to_value(old_val), to_value(new_val))
}

pub fn old_value<T: Serialize>(value: T) -> Option<Value> {
    to_value(value)
}

pub fn new_value<T: Serialize>(value: T) -> Option<Value> {
    to_value(value)
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
    use serde_json::json;

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
}
