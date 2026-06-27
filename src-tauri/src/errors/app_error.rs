use serde::Serialize;

#[derive(Debug, Serialize)]
pub struct AppErrorDto {
    pub code: String,
    pub message: String,
    pub details: Option<String>,
}

impl AppErrorDto {
    pub fn new(code: &str, message: &str, details: Option<String>) -> Self {
        Self {
            code: code.to_string(),
            message: message.to_string(),
            details,
        }
    }

    pub fn database(details: impl ToString) -> Self {
        Self::new(
            "ERR_DATABASE",
            "Ошибка работы с локальной базой данных.",
            Some(details.to_string()),
        )
    }

    pub fn filesystem(details: impl ToString) -> Self {
        Self::new(
            "ERR_FILESYSTEM",
            "Ошибка работы с локальным хранилищем.",
            Some(details.to_string()),
        )
    }

    pub fn access_denied(message: &str) -> Self {
        Self::new("ERR_ACCESS_DENIED", message, None)
    }

    pub fn unauthorized(message: &str) -> Self {
        Self::new("ERR_UNAUTHORIZED", message, None)
    }

    pub fn validation(message: &str) -> Self {
        Self::new("ERR_VALIDATION", message, None)
    }

    pub fn not_found(message: &str) -> Self {
        Self::new("ERR_NOT_FOUND", message, None)
    }

    pub fn internal(details: impl ToString) -> Self {
        Self::new(
            "ERR_INTERNAL",
            "Внутренняя ошибка приложения.",
            Some(details.to_string()),
        )
    }

    pub fn password_change_required() -> Self {
        Self::new(
            "ERR_PASSWORD_CHANGE_REQUIRED",
            "Необходимо сменить временный пароль",
            Some(
                "Перед выполнением действия пользователь должен сменить временный пароль."
                    .to_string(),
            ),
        )
    }

    /// Returns a sanitized view of this error safe to embed in audit
    /// `technical_details`.  The `details` field — which may contain raw OS
    /// or filesystem error text — is scrubbed for path tokens before use.
    pub fn to_safe_audit_error(&self) -> SafeAuditErrorDto {
        SafeAuditErrorDto {
            code: self.code.clone(),
            message: self.message.clone(),
            details: self
                .details
                .as_deref()
                .map(crate::audit::audit_error_sanitizer::sanitize_error_text),
        }
    }
}

/// A sanitized error representation safe for inclusion in audit records.
///
/// Unlike `AppErrorDto`, the `details` field has all filesystem path tokens
/// replaced with `[redacted:path]`, so it will never leak OS-level paths
/// into the audit log.
#[derive(Debug, Clone, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct SafeAuditErrorDto {
    pub code: String,
    pub message: String,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub details: Option<String>,
}

#[derive(Debug, Serialize)]
#[serde(untagged)]
pub enum CommandResult<T>
where
    T: Serialize,
{
    Ok { ok: bool, data: T },
    Err { ok: bool, error: AppErrorDto },
}

impl<T> CommandResult<T>
where
    T: Serialize,
{
    pub fn ok(data: T) -> Self {
        Self::Ok { ok: true, data }
    }

    pub fn err(error: AppErrorDto) -> Self {
        Self::Err { ok: false, error }
    }
}
