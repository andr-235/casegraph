use serde_json::Value;

/// A typed wrapper around a JSON snapshot value for audit `old_value`/`new_value`.
///
/// The constructor is restricted to `crate::audit` so business services cannot
/// build an `AuditSafeSnapshot` from arbitrary `serde_json::Value`.  Only
/// `audit_metadata` builders, which produce well-known typed structs, are
/// allowed to create instances.
#[derive(Debug, Clone)]
pub struct AuditSafeSnapshot {
    value: Value,
}

/// A typed wrapper around a JSON details value for audit `technical_details`.
///
/// Same visibility restriction as `AuditSafeSnapshot`: only code inside
/// `crate::audit` can construct this.  The sanitizer is applied at construction
/// time inside the `build_details` helper in `audit_metadata`.
#[derive(Debug, Clone)]
pub struct AuditSafeDetails {
    value: Value,
}

impl AuditSafeSnapshot {
    /// Internal constructor — only accessible within `crate::audit`.
    pub(in crate::audit) fn from_checked_value(value: Value) -> Self {
        Self { value }
    }

    /// Consume the wrapper and return the inner JSON value.
    /// Accessible to the service layer so `AuditService` can serialize it.
    pub(crate) fn into_value(self) -> Value {
        self.value
    }
}

impl AuditSafeDetails {
    /// Internal constructor — only accessible within `crate::audit`.
    pub(in crate::audit) fn from_checked_value(value: Value) -> Self {
        Self { value }
    }

    /// Consume the wrapper and return the inner JSON value.
    /// Accessible to the service layer so `AuditService` can serialize it.
    pub(crate) fn into_value(self) -> Value {
        self.value
    }
}
