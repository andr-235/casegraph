use crate::domain::event_date_precision::{
    is_valid_date_precision, DATE_PRECISION_PERIOD,
};
use crate::domain::event_type::is_valid_event_type;
use crate::errors::app_error::AppErrorDto;

const MAX_EVENT_TITLE_LEN: usize = 200;
const MAX_EVENT_DESCRIPTION_LEN: usize = 5_000;
const MAX_EVENT_SOURCE_NOTE_LEN: usize = 2_000;
const MAX_EVENT_ANALYST_COMMENT_LEN: usize = 2_000;
const MAX_EVENT_LINK_NOTE_LEN: usize = 2_000;

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

pub fn normalize_event_type(value: &str) -> Result<String, AppErrorDto> {
    let normalized = value.trim().to_string();

    if !is_valid_event_type(&normalized) {
        return Err(AppErrorDto::new(
            "ERR_INVALID_EVENT_TYPE",
            "Недопустимый тип события",
            None,
        ));
    }

    Ok(normalized)
}

pub fn normalize_date_precision(value: &str) -> Result<String, AppErrorDto> {
    let normalized = value.trim().to_string();

    if !is_valid_date_precision(&normalized) {
        return Err(AppErrorDto::new(
            "ERR_INVALID_DATE_PRECISION",
            "Недопустимая точность даты события",
            None,
        ));
    }

    Ok(normalized)
}

pub fn normalize_required_text(
    value: &str,
    max_len: usize,
    code: &str,
    empty_message: &str,
    length_message: &str,
) -> Result<String, AppErrorDto> {
    let normalized = value.trim().to_string();

    if normalized.is_empty() {
        return Err(AppErrorDto::new(code, empty_message, None));
    }

    if normalized.chars().count() > max_len {
        return Err(AppErrorDto::new(code, length_message, None));
    }

    Ok(normalized)
}

pub fn normalize_optional_text(
    value: &str,
    max_len: usize,
    code: &str,
    length_message: &str,
) -> Result<String, AppErrorDto> {
    let normalized = value.trim().to_string();

    if normalized.chars().count() > max_len {
        return Err(AppErrorDto::new(code, length_message, None));
    }

    Ok(normalized)
}

pub fn normalize_event_title(value: &str) -> Result<String, AppErrorDto> {
    normalize_required_text(
        value,
        MAX_EVENT_TITLE_LEN,
        "ERR_INVALID_EVENT_TITLE",
        "Название события обязательно",
        "Название события слишком длинное",
    )
}

pub fn normalize_event_description(value: &str) -> Result<String, AppErrorDto> {
    normalize_optional_text(
        value,
        MAX_EVENT_DESCRIPTION_LEN,
        "ERR_INVALID_EVENT_DESCRIPTION",
        "Описание события слишком длинное",
    )
}

pub fn normalize_event_source_note(value: &str) -> Result<String, AppErrorDto> {
    normalize_optional_text(
        value,
        MAX_EVENT_SOURCE_NOTE_LEN,
        "ERR_INVALID_EVENT_SOURCE_NOTE",
        "Основание события слишком длинное",
    )
}

pub fn normalize_event_analyst_comment(value: &str) -> Result<String, AppErrorDto> {
    normalize_optional_text(
        value,
        MAX_EVENT_ANALYST_COMMENT_LEN,
        "ERR_INVALID_EVENT_ANALYST_COMMENT",
        "Комментарий аналитика слишком длинный",
    )
}

pub fn normalize_event_link_note(value: &str) -> Result<String, AppErrorDto> {
    normalize_optional_text(
        value,
        MAX_EVENT_LINK_NOTE_LEN,
        "ERR_INVALID_EVENT_LINK_NOTE",
        "Комментарий связи события слишком длинный",
    )
}

pub fn normalize_event_date(value: &str) -> Result<String, AppErrorDto> {
    let normalized = value.trim().to_string();

    if normalized.is_empty() {
        return Err(AppErrorDto::new(
            "ERR_INVALID_EVENT_DATE",
            "Дата события обязательна",
            None,
        ));
    }

    Ok(normalized)
}

pub fn validate_period(
    date_precision: &str,
    period_start: &Option<String>,
    period_end: &Option<String>,
) -> Result<(), AppErrorDto> {
    if date_precision != DATE_PRECISION_PERIOD {
        return Ok(());
    }

    let start = period_start.as_deref().unwrap_or("").trim();
    let end = period_end.as_deref().unwrap_or("").trim();

    if start.is_empty() || end.is_empty() {
        return Err(AppErrorDto::new(
            "ERR_INVALID_EVENT_PERIOD",
            "Для периода нужно указать начало и конец",
            None,
        ));
    }

    if start > end {
        return Err(AppErrorDto::new(
            "ERR_INVALID_EVENT_PERIOD",
            "Начало периода не может быть позже конца периода",
            None,
        ));
    }

    Ok(())
}
