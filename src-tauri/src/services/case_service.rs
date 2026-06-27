use tauri::AppHandle;
use uuid::Uuid;

use crate::domain::case_status::is_editable_case_status;
use crate::domain::cases::{
    CaseDto, CreateCasePayload, CreateCaseResponse, GetCaseByIdPayload, UpdateCasePayload,
    UpdateCaseResponse, UpdateCaseStatusPayload, UpdateCaseStatusResponse,
};
use crate::errors::app_error::AppErrorDto;
use crate::repositories::case_repository::{
    CaseRepository, CaseRow, CreateCaseRecord, UpdateCaseRecord,
};
use crate::security::session::SessionState;
use crate::services::protected_service_context::require_protected_user;

pub struct CaseService;

impl CaseService {
    pub fn get_cases(app: &AppHandle, session: &SessionState) -> Result<Vec<CaseDto>, AppErrorDto> {
        let context = require_protected_user(app, session)?;
        let conn = &context.conn;
        let rows = CaseRepository::get_cases(conn)?;

        Ok(rows.into_iter().map(case_row_to_dto).collect())
    }

    pub fn create_case(
        app: &AppHandle,
        session: &SessionState,
        payload: CreateCasePayload,
    ) -> Result<CreateCaseResponse, AppErrorDto> {
        let context = require_protected_user(app, session)?;
        let current_user = &context.current_user;
        let conn = &context.conn;

        let title = payload.title.trim().to_string();
        let subject = payload.subject.trim().to_string();
        let description = payload.description.unwrap_or_default().trim().to_string();

        validate_create_case_payload(
            &title,
            &subject,
            payload.period_start.as_deref(),
            payload.period_end.as_deref(),
        )?;

        let case_code = CaseRepository::get_next_case_code(conn)?;
        let case_id = Uuid::new_v4().to_string();

        CaseRepository::create_case(
            conn,
            CreateCaseRecord {
                id: case_id.clone(),
                case_code: case_code.clone(),
                title,
                subject,
                description,
                period_start: payload.period_start,
                period_end: payload.period_end,
                created_by_user_id: current_user.user_id.clone(),
            },
        )?;

        let created_case = CaseRepository::get_case_by_id(conn, &case_id)?.ok_or_else(|| {
            AppErrorDto::new(
                "ERR_CASE_NOT_FOUND_AFTER_CREATE",
                "Дело создано, но не найдено после сохранения.",
                None,
            )
        })?;

        Ok(CreateCaseResponse {
            case_item: case_row_to_dto(created_case),
        })
    }

    pub fn get_case_by_id(
        app: &AppHandle,
        session: &SessionState,
        payload: GetCaseByIdPayload,
    ) -> Result<CaseDto, AppErrorDto> {
        let context = require_protected_user(app, session)?;
        let conn = &context.conn;

        let case_id = payload.case_id.trim();

        if case_id.is_empty() {
            return Err(AppErrorDto::new(
                "ERR_VALIDATION",
                "Не указан идентификатор дела.",
                None,
            ));
        }

        let case_row = CaseRepository::get_case_by_id(conn, case_id)?
            .ok_or_else(|| AppErrorDto::new("ERR_CASE_NOT_FOUND", "Дело не найдено.", None))?;

        Ok(case_row_to_dto(case_row))
    }

    pub fn update_case(
        app: &AppHandle,
        session: &SessionState,
        payload: UpdateCasePayload,
    ) -> Result<UpdateCaseResponse, AppErrorDto> {
        let context = require_protected_user(app, session)?;
        let conn = &context.conn;

        let case_id = payload.case_id.trim().to_string();
        let title = payload.title.trim().to_string();
        let subject = payload.subject.trim().to_string();
        let description = payload.description.unwrap_or_default().trim().to_string();

        if case_id.is_empty() {
            return Err(AppErrorDto::new(
                "ERR_VALIDATION",
                "Не указан идентификатор дела.",
                None,
            ));
        }

        validate_create_case_payload(
            &title,
            &subject,
            payload.period_start.as_deref(),
            payload.period_end.as_deref(),
        )?;

        CaseRepository::update_case(
            conn,
            UpdateCaseRecord {
                case_id: case_id.clone(),
                title,
                subject,
                description,
                period_start: normalize_optional_string(payload.period_start),
                period_end: normalize_optional_string(payload.period_end),
            },
        )?;

        let updated_case = CaseRepository::get_case_by_id(conn, &case_id)?.ok_or_else(|| {
            AppErrorDto::new(
                "ERR_CASE_NOT_FOUND_AFTER_UPDATE",
                "Дело обновлено, но не найдено после сохранения.",
                None,
            )
        })?;

        Ok(UpdateCaseResponse {
            case_item: case_row_to_dto(updated_case),
        })
    }

    pub fn update_case_status(
        app: &AppHandle,
        session: &SessionState,
        payload: UpdateCaseStatusPayload,
    ) -> Result<UpdateCaseStatusResponse, AppErrorDto> {
        let context = require_protected_user(app, session)?;
        let conn = &context.conn;

        let case_id = payload.case_id.trim().to_string();
        let status = payload.status.trim().to_string();

        if case_id.is_empty() {
            return Err(AppErrorDto::new(
                "ERR_VALIDATION",
                "Не указан идентификатор дела.",
                None,
            ));
        }

        validate_case_status(&status)?;

        CaseRepository::update_case_status(conn, &case_id, &status)?;

        let updated_case = CaseRepository::get_case_by_id(conn, &case_id)?.ok_or_else(|| {
            AppErrorDto::new(
                "ERR_CASE_NOT_FOUND_AFTER_UPDATE",
                "Дело обновлено, но не найдено после сохранения.",
                None,
            )
        })?;

        Ok(UpdateCaseStatusResponse {
            case_item: case_row_to_dto(updated_case),
        })
    }
}

fn validate_create_case_payload(
    title: &str,
    subject: &str,
    period_start: Option<&str>,
    period_end: Option<&str>,
) -> Result<(), AppErrorDto> {
    if title.len() < 3 {
        return Err(AppErrorDto::new(
            "ERR_VALIDATION",
            "Название дела должно содержать минимум 3 символа.",
            None,
        ));
    }

    if subject.len() < 2 {
        return Err(AppErrorDto::new(
            "ERR_VALIDATION",
            "Объект анализа должен содержать минимум 2 символа.",
            None,
        ));
    }

    if let (Some(start), Some(end)) = (period_start, period_end) {
        if !start.is_empty() && !end.is_empty() && start > end {
            return Err(AppErrorDto::new(
                "ERR_VALIDATION",
                "Дата начала периода не может быть позже даты окончания.",
                None,
            ));
        }
    }

    Ok(())
}

fn case_row_to_dto(row: CaseRow) -> CaseDto {
    CaseDto {
        id: row.id,
        case_code: row.case_code,
        title: row.title,
        subject: row.subject,
        description: row.description,
        status: row.status,
        period_start: row.period_start,
        period_end: row.period_end,
        created_by_user_id: row.created_by_user_id,
        created_at: row.created_at,
        updated_at: row.updated_at,
    }
}

fn normalize_optional_string(value: Option<String>) -> Option<String> {
    value
        .map(|item| item.trim().to_string())
        .filter(|item| !item.is_empty())
}

fn validate_case_status(status: &str) -> Result<(), AppErrorDto> {
    if is_editable_case_status(status) {
        return Ok(());
    }

    Err(AppErrorDto::new(
        "ERR_VALIDATION",
        "Недопустимый статус дела.",
        None,
    ))
}
