use crate::domain::object_type::is_valid_object_type;
use crate::errors::app_error::AppErrorDto;

const MAX_OBJECT_TITLE_LEN: usize = 200;
const MAX_OBJECT_VALUE_LEN: usize = 500;
const MAX_OBJECT_DESCRIPTION_LEN: usize = 5_000;
const MAX_OBJECT_CONFIDENCE_NOTE_LEN: usize = 2_000;
const MAX_OBJECT_LINK_REASON_LEN: usize = 2_000;

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

pub fn normalize_object_type(value: &str) -> Result<String, AppErrorDto> {
    let normalized = value.trim().to_lowercase();

    if normalized.is_empty() {
        return Err(AppErrorDto::new(
            "ERR_OBJECT_TYPE_REQUIRED",
            "Не выбран тип объекта.",
            None,
        ));
    }

    if !is_valid_object_type(&normalized) {
        return Err(AppErrorDto::new(
            "ERR_OBJECT_TYPE_INVALID",
            "Недопустимый тип объекта.",
            None,
        ));
    }

    Ok(normalized)
}

pub fn normalize_object_title(value: &str) -> Result<String, AppErrorDto> {
    let normalized = value.trim().to_string();

    if normalized.is_empty() {
        return Err(AppErrorDto::new(
            "ERR_OBJECT_TITLE_REQUIRED",
            "Название объекта обязательно.",
            None,
        ));
    }

    if normalized.chars().count() > MAX_OBJECT_TITLE_LEN {
        return Err(AppErrorDto::new(
            "ERR_OBJECT_TITLE_TOO_LONG",
            "Название объекта слишком длинное.",
            None,
        ));
    }

    Ok(normalized)
}

pub fn normalize_optional_value(value: Option<String>) -> Result<Option<String>, AppErrorDto> {
    let Some(value) = value else {
        return Ok(None);
    };

    let normalized = value.trim().to_string();

    if normalized.is_empty() {
        return Ok(None);
    }

    if normalized.chars().count() > MAX_OBJECT_VALUE_LEN {
        return Err(AppErrorDto::new(
            "ERR_OBJECT_VALUE_TOO_LONG",
            "Значение объекта слишком длинное.",
            None,
        ));
    }

    Ok(Some(normalized))
}

pub fn normalize_object_description(value: Option<String>) -> Result<String, AppErrorDto> {
    let normalized = value.unwrap_or_default().trim().to_string();

    if normalized.chars().count() > MAX_OBJECT_DESCRIPTION_LEN {
        return Err(AppErrorDto::new(
            "ERR_OBJECT_DESCRIPTION_TOO_LONG",
            "Описание объекта слишком длинное.",
            None,
        ));
    }

    Ok(normalized)
}

pub fn normalize_confidence_note(value: Option<String>) -> Result<String, AppErrorDto> {
    let normalized = value.unwrap_or_default().trim().to_string();

    if normalized.chars().count() > MAX_OBJECT_CONFIDENCE_NOTE_LEN {
        return Err(AppErrorDto::new(
            "ERR_OBJECT_CONFIDENCE_NOTE_TOO_LONG",
            "Примечание по достоверности слишком длинное.",
            None,
        ));
    }

    Ok(normalized)
}

pub fn normalize_link_reason(value: Option<String>) -> Result<String, AppErrorDto> {
    let normalized = value.unwrap_or_default().trim().to_string();

    if normalized.chars().count() > MAX_OBJECT_LINK_REASON_LEN {
        return Err(AppErrorDto::new(
            "ERR_OBJECT_LINK_REASON_TOO_LONG",
            "Основание связи объекта с материалом слишком длинное.",
            None,
        ));
    }

    Ok(normalized)
}

pub fn normalize_unique_ids(values: Vec<String>) -> Vec<String> {
    let mut normalized_values = values
        .into_iter()
        .map(|value| value.trim().to_string())
        .filter(|value| !value.is_empty())
        .collect::<Vec<_>>();

    normalized_values.sort();
    normalized_values.dedup();

    normalized_values
}
