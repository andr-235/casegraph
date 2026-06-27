use std::collections::HashSet;

use crate::domain::event_date_precision::{is_valid_date_precision, DATE_PRECISION_PERIOD};
use crate::domain::event_type::is_valid_event_type;
use crate::domain::timeline::{CreateEventPayload, GetTimelinePayload, UpdateEventPayload};
use crate::errors::app_error::AppErrorDto;

const MAX_EVENT_TITLE_LEN: usize = 200;
const MAX_EVENT_DESCRIPTION_LEN: usize = 5_000;
const MAX_EVENT_SOURCE_NOTE_LEN: usize = 2_000;
const MAX_EVENT_ANALYST_COMMENT_LEN: usize = 2_000;
const MAX_EVENT_LINK_NOTE_LEN: usize = 2_000;

#[derive(Debug)]
pub struct NormalizedCreateEventInput {
    pub case_id: String,
    pub event_type: String,
    pub title: String,
    pub description: String,
    pub event_date: String,
    pub event_time: Option<String>,
    pub date_precision: String,
    pub period_start: Option<String>,
    pub period_end: Option<String>,
    pub source_note: String,
    pub analyst_comment: String,
    pub include_in_report: bool,
    pub object_ids: Vec<String>,
    pub material_ids: Vec<String>,
    pub link_note: String,
}

#[derive(Debug)]
pub struct NormalizedUpdateEventInput {
    pub case_id: String,
    pub event_id: String,
    pub event_type: String,
    pub title: String,
    pub description: String,
    pub event_date: String,
    pub event_time: Option<String>,
    pub date_precision: String,
    pub period_start: Option<String>,
    pub period_end: Option<String>,
    pub source_note: String,
    pub analyst_comment: String,
    pub include_in_report: bool,
    pub object_ids: Vec<String>,
    pub material_ids: Vec<String>,
    pub link_note: String,
}

#[derive(Debug)]
pub struct NormalizedTimelineFilters {
    pub query: Option<String>,
    pub event_type: Option<String>,
    pub object_id: Option<String>,
    pub material_id: Option<String>,
    pub date_from: Option<String>,
    pub date_to: Option<String>,
    pub include_in_report: Option<bool>,
}

pub fn normalize_timeline_filters(
    payload: &GetTimelinePayload,
) -> Result<NormalizedTimelineFilters, AppErrorDto> {
    let query =
        normalize_optional_short_text(payload.query.clone(), 200, "ERR_INVALID_TIMELINE_QUERY")?;

    let event_type = match payload
        .event_type
        .clone()
        .map(|value| value.trim().to_string())
    {
        Some(value) if value.is_empty() => None,
        Some(value) => {
            if !is_valid_event_type(&value) {
                return Err(AppErrorDto::new(
                    "ERR_INVALID_EVENT_TYPE",
                    "Некорректный тип события",
                    None,
                ));
            }

            Some(value)
        }
        None => None,
    };

    let object_id = normalize_optional_id(payload.object_id.clone(), "ERR_INVALID_OBJECT_ID")?;
    let material_id =
        normalize_optional_id(payload.material_id.clone(), "ERR_INVALID_MATERIAL_ID")?;
    let date_from =
        normalize_optional_date_filter(payload.date_from.clone(), "ERR_INVALID_DATE_FROM")?;
    let date_to = normalize_optional_date_filter(payload.date_to.clone(), "ERR_INVALID_DATE_TO")?;

    if let (Some(from), Some(to)) = (&date_from, &date_to) {
        if from > to {
            return Err(AppErrorDto::new(
                "ERR_INVALID_DATE_PERIOD",
                "Начальная дата периода не может быть позже конечной",
                None,
            ));
        }
    }

    Ok(NormalizedTimelineFilters {
        query,
        event_type,
        object_id,
        material_id,
        date_from,
        date_to,
        include_in_report: payload.include_in_report,
    })
}

pub fn normalize_update_event_payload(
    payload: UpdateEventPayload,
) -> Result<NormalizedUpdateEventInput, AppErrorDto> {
    let case_id = normalize_required_id(
        &payload.case_id,
        "ERR_INVALID_CASE_ID",
        "ID дела обязателен",
    )?;

    let event_id = normalize_required_id(
        &payload.event_id,
        "ERR_INVALID_EVENT_ID",
        "ID события обязателен",
    )?;

    let event_type = normalize_event_type(&payload.event_type)?;
    let title = normalize_event_title(&payload.title)?;
    let description = normalize_event_description(&payload.description)?;
    let event_date = normalize_event_date(&payload.event_date)?;
    let event_time = normalize_event_time(payload.event_time)?;
    let date_precision = normalize_date_precision(&payload.date_precision)?;

    let period_start = normalize_optional_date(payload.period_start);
    let period_end = normalize_optional_date(payload.period_end);

    validate_period(&date_precision, &period_start, &period_end)?;

    let source_note = normalize_event_source_note(&payload.source_note)?;
    let analyst_comment = normalize_event_analyst_comment(&payload.analyst_comment)?;
    let link_note = normalize_event_link_note(&payload.link_note)?;

    let object_ids = normalize_id_list(payload.object_ids);
    let material_ids = normalize_id_list(payload.material_ids);

    Ok(NormalizedUpdateEventInput {
        case_id,
        event_id,
        event_type,
        title,
        description,
        event_date,
        event_time,
        date_precision,
        period_start,
        period_end,
        source_note,
        analyst_comment,
        include_in_report: payload.include_in_report,
        object_ids,
        material_ids,
        link_note,
    })
}

pub fn normalize_create_event_payload(
    payload: CreateEventPayload,
) -> Result<NormalizedCreateEventInput, AppErrorDto> {
    let case_id = normalize_required_id(
        &payload.case_id,
        "ERR_INVALID_CASE_ID",
        "ID дела обязателен",
    )?;

    let event_type = normalize_event_type(&payload.event_type)?;
    let title = normalize_event_title(&payload.title)?;
    let description = normalize_event_description(&payload.description)?;
    let event_date = normalize_event_date(&payload.event_date)?;
    let event_time = normalize_event_time(payload.event_time)?;
    let date_precision = normalize_date_precision(&payload.date_precision)?;

    let period_start = normalize_optional_date(payload.period_start);
    let period_end = normalize_optional_date(payload.period_end);

    validate_period(&date_precision, &period_start, &period_end)?;

    let source_note = normalize_event_source_note(&payload.source_note)?;
    let analyst_comment = normalize_event_analyst_comment(&payload.analyst_comment)?;
    let link_note = normalize_event_link_note(&payload.link_note)?;

    let object_ids = normalize_id_list(payload.object_ids);
    let material_ids = normalize_id_list(payload.material_ids);

    Ok(NormalizedCreateEventInput {
        case_id,
        event_type,
        title,
        description,
        event_date,
        event_time,
        date_precision,
        period_start,
        period_end,
        source_note,
        analyst_comment,
        include_in_report: payload.include_in_report,
        object_ids,
        material_ids,
        link_note,
    })
}

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

fn normalize_id_list(values: Vec<String>) -> Vec<String> {
    let mut seen = HashSet::new();
    let mut normalized = Vec::new();

    for value in values {
        let trimmed = value.trim().to_string();

        if trimmed.is_empty() {
            continue;
        }

        if seen.insert(trimmed.clone()) {
            normalized.push(trimmed);
        }
    }

    normalized
}

fn normalize_event_type(value: &str) -> Result<String, AppErrorDto> {
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

fn normalize_date_precision(value: &str) -> Result<String, AppErrorDto> {
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

fn normalize_event_title(value: &str) -> Result<String, AppErrorDto> {
    normalize_required_text(
        value,
        MAX_EVENT_TITLE_LEN,
        "ERR_INVALID_EVENT_TITLE",
        "Название события обязательно",
        "Название события слишком длинное",
    )
}

fn normalize_event_description(value: &str) -> Result<String, AppErrorDto> {
    normalize_optional_text(
        value,
        MAX_EVENT_DESCRIPTION_LEN,
        "ERR_INVALID_EVENT_DESCRIPTION",
        "Описание события слишком длинное",
    )
}

fn normalize_event_source_note(value: &str) -> Result<String, AppErrorDto> {
    normalize_optional_text(
        value,
        MAX_EVENT_SOURCE_NOTE_LEN,
        "ERR_INVALID_EVENT_SOURCE_NOTE",
        "Основание события слишком длинное",
    )
}

fn normalize_event_analyst_comment(value: &str) -> Result<String, AppErrorDto> {
    normalize_optional_text(
        value,
        MAX_EVENT_ANALYST_COMMENT_LEN,
        "ERR_INVALID_EVENT_ANALYST_COMMENT",
        "Комментарий аналитика слишком длинный",
    )
}

fn normalize_event_link_note(value: &str) -> Result<String, AppErrorDto> {
    normalize_optional_text(
        value,
        MAX_EVENT_LINK_NOTE_LEN,
        "ERR_INVALID_EVENT_LINK_NOTE",
        "Комментарий связи события слишком длинный",
    )
}

fn normalize_event_date(value: &str) -> Result<String, AppErrorDto> {
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

fn normalize_event_time(value: Option<String>) -> Result<Option<String>, AppErrorDto> {
    let Some(raw) = value else {
        return Ok(None);
    };

    let normalized = raw.trim().to_string();

    if normalized.is_empty() {
        return Ok(None);
    }

    let is_valid_hh_mm = normalized.len() == 5
        && normalized.as_bytes()[2] == b':'
        && normalized[0..2].chars().all(|ch| ch.is_ascii_digit())
        && normalized[3..5].chars().all(|ch| ch.is_ascii_digit());

    if !is_valid_hh_mm {
        return Err(AppErrorDto::new(
            "ERR_INVALID_EVENT_TIME",
            "Время события должно быть в формате HH:MM",
            None,
        ));
    }

    Ok(Some(normalized))
}

fn normalize_optional_date(value: Option<String>) -> Option<String> {
    value
        .map(|item| item.trim().to_string())
        .filter(|item| !item.is_empty())
}

fn validate_period(
    date_precision: &str,
    period_start: &Option<String>,
    period_end: &Option<String>,
) -> Result<(), AppErrorDto> {
    if date_precision != DATE_PRECISION_PERIOD {
        return Ok(());
    }

    let Some(start) = period_start else {
        return Err(AppErrorDto::new(
            "ERR_INVALID_EVENT_PERIOD",
            "Для периода нужно указать начало и конец",
            None,
        ));
    };

    let Some(end) = period_end else {
        return Err(AppErrorDto::new(
            "ERR_INVALID_EVENT_PERIOD",
            "Для периода нужно указать начало и конец",
            None,
        ));
    };

    if start > end {
        return Err(AppErrorDto::new(
            "ERR_INVALID_EVENT_PERIOD",
            "Начало периода не может быть позже конца периода",
            None,
        ));
    }

    Ok(())
}

fn normalize_required_text(
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

fn normalize_optional_short_text(
    value: Option<String>,
    max_len: usize,
    code: &str,
) -> Result<Option<String>, AppErrorDto> {
    match value {
        Some(value) => {
            let trimmed = value.trim().to_string();

            if trimmed.is_empty() {
                return Ok(None);
            }

            if trimmed.chars().count() > max_len {
                return Err(AppErrorDto::new(
                    code,
                    "Значение фильтра слишком длинное",
                    None,
                ));
            }

            Ok(Some(trimmed))
        }
        None => Ok(None),
    }
}

fn normalize_optional_id(
    value: Option<String>,
    _code: &str,
) -> Result<Option<String>, AppErrorDto> {
    match value {
        Some(value) => {
            let trimmed = value.trim().to_string();

            if trimmed.is_empty() {
                return Ok(None);
            }

            Ok(Some(trimmed))
        }
        None => Ok(None),
    }
}

fn normalize_optional_date_filter(
    value: Option<String>,
    code: &str,
) -> Result<Option<String>, AppErrorDto> {
    match value {
        Some(value) => {
            let trimmed = value.trim().to_string();

            if trimmed.is_empty() {
                return Ok(None);
            }

            if trimmed.len() != 10 {
                return Err(AppErrorDto::new(
                    code,
                    "Дата должна быть в формате YYYY-MM-DD",
                    None,
                ));
            }

            Ok(Some(trimmed))
        }
        None => Ok(None),
    }
}

fn normalize_optional_text(
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
