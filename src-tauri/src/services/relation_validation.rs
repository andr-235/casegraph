use crate::domain::relation_confidence::is_valid_confidence_level;
use crate::domain::relation_type::is_valid_relation_type;
use crate::errors::app_error::AppErrorDto;

const MAX_RELATION_TITLE_LEN: usize = 200;
const MAX_RELATION_BASIS_LEN: usize = 5_000;
const MAX_RELATION_COMMENT_LEN: usize = 2_000;

pub fn normalize_required_id(
    value: &str,
    code: &str,
    message: &str,
) -> Result<String, AppErrorDto> {
    let normalized = value.trim().to_string();

    if normalized.is_empty() {
        return Err(AppErrorDto::new(code, message, None));
    }

    Ok(normalized)
}

pub fn normalize_optional_id(value: &Option<String>) -> Option<String> {
    value
        .as_ref()
        .map(|item| item.trim().to_string())
        .filter(|item| !item.is_empty())
}

pub fn normalize_relation_type(value: &str) -> Result<String, AppErrorDto> {
    let normalized = value.trim().to_string();

    if normalized.is_empty() {
        return Err(AppErrorDto::new(
            "ERR_RELATION_TYPE_REQUIRED",
            "Не выбран тип связи.",
            None,
        ));
    }

    if !is_valid_relation_type(&normalized) {
        return Err(AppErrorDto::new(
            "ERR_RELATION_TYPE_INVALID",
            "Недопустимый тип связи.",
            Some(normalized),
        ));
    }

    Ok(normalized)
}

pub fn normalize_confidence_level(value: &str) -> Result<String, AppErrorDto> {
    let normalized = value.trim().to_string();

    if normalized.is_empty() {
        return Err(AppErrorDto::new(
            "ERR_RELATION_CONFIDENCE_REQUIRED",
            "Не выбран уровень достоверности связи.",
            None,
        ));
    }

    if !is_valid_confidence_level(&normalized) {
        return Err(AppErrorDto::new(
            "ERR_RELATION_CONFIDENCE_INVALID",
            "Недопустимый уровень достоверности связи.",
            Some(normalized),
        ));
    }

    Ok(normalized)
}

pub fn normalize_relation_title(value: &Option<String>) -> Result<Option<String>, AppErrorDto> {
    normalize_optional_text(
        value,
        MAX_RELATION_TITLE_LEN,
        "ERR_RELATION_TITLE_TOO_LONG",
        "Название связи слишком длинное.",
    )
}

pub fn normalize_relation_basis(value: &str) -> Result<String, AppErrorDto> {
    let normalized = value.trim().to_string();

    if normalized.is_empty() {
        return Err(AppErrorDto::new(
            "ERR_RELATION_BASIS_REQUIRED",
            "Укажите основание связи.",
            None,
        ));
    }

    if normalized.chars().count() > MAX_RELATION_BASIS_LEN {
        return Err(AppErrorDto::new(
            "ERR_RELATION_BASIS_TOO_LONG",
            "Основание связи слишком длинное.",
            None,
        ));
    }

    Ok(normalized)
}

pub fn normalize_analyst_comment(value: &Option<String>) -> Result<Option<String>, AppErrorDto> {
    normalize_optional_text(
        value,
        MAX_RELATION_COMMENT_LEN,
        "ERR_RELATION_COMMENT_TOO_LONG",
        "Комментарий аналитика слишком длинный.",
    )
}

pub fn validate_relation_endpoints(
    source_object_id: &str,
    target_object_id: &str,
) -> Result<(), AppErrorDto> {
    if source_object_id == target_object_id {
        return Err(AppErrorDto::new(
            "ERR_RELATION_SELF_LINK",
            "Нельзя создать связь объекта с самим собой.",
            None,
        ));
    }

    Ok(())
}

fn normalize_optional_text(
    value: &Option<String>,
    max_len: usize,
    code: &str,
    message: &str,
) -> Result<Option<String>, AppErrorDto> {
    let Some(raw_value) = value else {
        return Ok(None);
    };

    let normalized = raw_value.trim().to_string();

    if normalized.is_empty() {
        return Ok(None);
    }

    if normalized.chars().count() > max_len {
        return Err(AppErrorDto::new(code, message, None));
    }

    Ok(Some(normalized))
}
