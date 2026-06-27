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
use crate::services::protected_service_context::require_protected_user_for;

pub struct CaseService;

impl CaseService {
    pub fn get_cases(app: &AppHandle, session: &SessionState) -> Result<Vec<CaseDto>, AppErrorDto> {
        let context = require_protected_user_for(app, session, "GET_CASES")?;
        let conn = &context.conn;
        let rows = CaseRepository::get_cases(conn)?;

        Ok(rows.into_iter().map(case_row_to_dto).collect())
    }

    pub fn create_case(
        app: &AppHandle,
        session: &SessionState,
        payload: CreateCasePayload,
    ) -> Result<CreateCaseResponse, AppErrorDto> {
        let context = require_protected_user_for(app, session, "CREATE_CASE")?;
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

        write_case_created_audit_best_effort(app, current_user, &created_case);

        Ok(CreateCaseResponse {
            case_item: case_row_to_dto(created_case),
        })
    }

    pub fn get_case_by_id(
        app: &AppHandle,
        session: &SessionState,
        payload: GetCaseByIdPayload,
    ) -> Result<CaseDto, AppErrorDto> {
        let context = require_protected_user_for(app, session, "GET_CASE_BY_ID")?;
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
        let context = require_protected_user_for(app, session, "UPDATE_CASE")?;
        let current_user = &context.current_user;
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

        let old_case = CaseRepository::get_case_by_id(conn, &case_id)?
            .ok_or_else(|| AppErrorDto::new("ERR_CASE_NOT_FOUND", "Дело не найдено.", None))?;

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

        write_case_updated_audit_best_effort(app, current_user, &old_case, &updated_case);

        Ok(UpdateCaseResponse {
            case_item: case_row_to_dto(updated_case),
        })
    }

    pub fn update_case_status(
        app: &AppHandle,
        session: &SessionState,
        payload: UpdateCaseStatusPayload,
    ) -> Result<UpdateCaseStatusResponse, AppErrorDto> {
        let context = require_protected_user_for(app, session, "UPDATE_CASE_STATUS")?;
        let current_user = &context.current_user;
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

        let old_case = CaseRepository::get_case_by_id(conn, &case_id)?
            .ok_or_else(|| AppErrorDto::new("ERR_CASE_NOT_FOUND", "Дело не найдено.", None))?;

        CaseRepository::update_case_status(conn, &case_id, &status)?;

        let updated_case = CaseRepository::get_case_by_id(conn, &case_id)?.ok_or_else(|| {
            AppErrorDto::new(
                "ERR_CASE_NOT_FOUND_AFTER_UPDATE",
                "Дело обновлено, но не найдено после сохранения.",
                None,
            )
        })?;

        write_case_status_changed_audit_best_effort(app, current_user, &old_case, &updated_case);

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

fn write_case_created_audit_best_effort(
    app: &AppHandle,
    current_user: &crate::security::session::CurrentUserDto,
    created_case: &CaseRow,
) {
    use crate::audit::audit_metadata;
    use crate::domain::audit_action;
    use crate::services::audit_service::{AuditService, AuditSuccessInput};

    let technical_details = audit_metadata::case_created(
        &created_case.id,
        &created_case.case_code,
        &created_case.title,
    );

    let new_value = audit_metadata::snapshot(audit_metadata::case_snapshot(
        &created_case.case_code,
        &created_case.title,
        Some(created_case.subject.as_str()),
        &created_case.status,
        created_case.period_start.as_deref(),
        created_case.period_end.as_deref(),
        Some(created_case.description.as_str()),
    ));

    let input = AuditSuccessInput::new(
        current_user,
        audit_action::case::CREATED,
        "case",
        Some(&created_case.id),
        Some(&created_case.id),
        None,
        new_value,
        technical_details,
    );

    AuditService::write_success_non_blocking(app.clone(), input);
}

fn write_case_updated_audit_best_effort(
    app: &AppHandle,
    current_user: &crate::security::session::CurrentUserDto,
    old_case: &CaseRow,
    new_case: &CaseRow,
) {
    use crate::audit::audit_metadata;
    use crate::domain::audit_action;
    use crate::services::audit_service::{AuditService, AuditSuccessInput};

    let mut changed = Vec::new();
    audit_metadata::push_changed(&mut changed, "title", &old_case.title, &new_case.title);
    audit_metadata::push_changed(
        &mut changed,
        "subject",
        &old_case.subject,
        &new_case.subject,
    );
    audit_metadata::push_changed(
        &mut changed,
        "periodStart",
        &old_case.period_start,
        &new_case.period_start,
    );
    audit_metadata::push_changed(
        &mut changed,
        "periodEnd",
        &old_case.period_end,
        &new_case.period_end,
    );
    audit_metadata::push_changed(
        &mut changed,
        "description",
        &old_case.description,
        &new_case.description,
    );

    let technical_details =
        audit_metadata::case_updated(&new_case.id, &new_case.case_code, &changed);

    let (old_val, new_val) = audit_metadata::old_new(
        audit_metadata::case_snapshot(
            &old_case.case_code,
            &old_case.title,
            Some(old_case.subject.as_str()),
            &old_case.status,
            old_case.period_start.as_deref(),
            old_case.period_end.as_deref(),
            Some(old_case.description.as_str()),
        ),
        audit_metadata::case_snapshot(
            &new_case.case_code,
            &new_case.title,
            Some(new_case.subject.as_str()),
            &new_case.status,
            new_case.period_start.as_deref(),
            new_case.period_end.as_deref(),
            Some(new_case.description.as_str()),
        ),
    );

    let input = AuditSuccessInput::new(
        current_user,
        audit_action::case::UPDATED,
        "case",
        Some(&new_case.id),
        Some(&new_case.id),
        old_val,
        new_val,
        technical_details,
    );

    AuditService::write_success_non_blocking(app.clone(), input);
}

fn write_case_status_changed_audit_best_effort(
    app: &AppHandle,
    current_user: &crate::security::session::CurrentUserDto,
    old_case: &CaseRow,
    new_case: &CaseRow,
) {
    use crate::audit::audit_metadata;
    use crate::domain::audit_action;
    use crate::services::audit_service::{AuditService, AuditSuccessInput};

    let technical_details = audit_metadata::case_status_changed(
        &new_case.id,
        &new_case.case_code,
        &old_case.status,
        &new_case.status,
    );

    let (old_val, new_val) = audit_metadata::old_new(
        audit_metadata::case_snapshot(
            &old_case.case_code,
            &old_case.title,
            Some(old_case.subject.as_str()),
            &old_case.status,
            old_case.period_start.as_deref(),
            old_case.period_end.as_deref(),
            Some(old_case.description.as_str()),
        ),
        audit_metadata::case_snapshot(
            &new_case.case_code,
            &new_case.title,
            Some(new_case.subject.as_str()),
            &new_case.status,
            new_case.period_start.as_deref(),
            new_case.period_end.as_deref(),
            Some(new_case.description.as_str()),
        ),
    );

    let input = AuditSuccessInput::new(
        current_user,
        audit_action::case::STATUS_CHANGED,
        "case",
        Some(&new_case.id),
        Some(&new_case.id),
        old_val,
        new_val,
        technical_details,
    );

    AuditService::write_success_non_blocking(app.clone(), input);
}
