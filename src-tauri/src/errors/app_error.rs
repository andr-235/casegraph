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
