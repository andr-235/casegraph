use serde::{Deserialize, Serialize};
use tauri::AppHandle;
use uuid::Uuid;

use crate::db::connection::open_connection;
use crate::errors::app_error::AppErrorDto;
use crate::repositories::case_repository::{CaseRepository, CaseRow, CreateCaseRecord};
use crate::security::session::{CurrentUserDto, SessionState};

#[derive(Debug, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct CaseDto {
    pub id: String,
    pub case_code: String,
    pub title: String,
    pub subject: String,
    pub description: String,
    pub status: String,
    pub period_start: Option<String>,
    pub period_end: Option<String>,
    pub created_by_user_id: String,
    pub created_at: String,
    pub updated_at: String,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CreateCasePayload {
    pub title: String,
    pub subject: String,
    pub description: Option<String>,
    pub period_start: Option<String>,
    pub period_end: Option<String>,
}

#[derive(Debug, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct CreateCaseResponse {
    pub case_item: CaseDto,
}

pub struct CaseService;

impl CaseService {
    pub fn get_cases(app: &AppHandle, session: &SessionState) -> Result<Vec<CaseDto>, AppErrorDto> {
        require_current_user(session)?;

        let conn = open_connection(app)?;
        let rows = CaseRepository::get_cases(&conn)?;

        Ok(rows.into_iter().map(case_row_to_dto).collect())
    }

    pub fn create_case(
        app: &AppHandle,
        session: &SessionState,
        payload: CreateCasePayload,
    ) -> Result<CreateCaseResponse, AppErrorDto> {
        let current_user = require_current_user(session)?;

        let title = payload.title.trim().to_string();
        let subject = payload.subject.trim().to_string();
        let description = payload.description.unwrap_or_default().trim().to_string();

        validate_create_case_payload(&title, &subject)?;

        let conn = open_connection(app)?;
        let case_code = CaseRepository::get_next_case_code(&conn)?;
        let case_id = Uuid::new_v4().to_string();

        CaseRepository::create_case(
            &conn,
            CreateCaseRecord {
                id: case_id.clone(),
                case_code: case_code.clone(),
                title,
                subject,
                description,
                period_start: payload.period_start,
                period_end: payload.period_end,
                created_by_user_id: current_user.user_id,
            },
        )?;

        let created_case = CaseRepository::get_case_by_id(&conn, &case_id)?.ok_or_else(|| {
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
}

fn require_current_user(session: &SessionState) -> Result<CurrentUserDto, AppErrorDto> {
    session
        .get_current_user()
        .ok_or_else(|| AppErrorDto::new("ERR_UNAUTHORIZED", "Требуется вход в систему.", None))
}

fn validate_create_case_payload(title: &str, subject: &str) -> Result<(), AppErrorDto> {
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
