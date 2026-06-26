use tauri::{AppHandle, State};

use crate::domain::cases::{
    CaseDto, CreateCasePayload, CreateCaseResponse, GetCaseByIdPayload, UpdateCasePayload,
    UpdateCaseResponse,
};
use crate::errors::app_error::CommandResult;
use crate::security::session::SessionState;
use crate::services::case_service::CaseService;

#[tauri::command]
pub fn get_cases(app: AppHandle, session: State<'_, SessionState>) -> CommandResult<Vec<CaseDto>> {
    match CaseService::get_cases(&app, &session) {
        Ok(response) => CommandResult::ok(response),
        Err(error) => CommandResult::err(error),
    }
}

#[tauri::command]
pub fn create_case(
    app: AppHandle,
    session: State<'_, SessionState>,
    payload: CreateCasePayload,
) -> CommandResult<CreateCaseResponse> {
    match CaseService::create_case(&app, &session, payload) {
        Ok(response) => CommandResult::ok(response),
        Err(error) => CommandResult::err(error),
    }
}

#[tauri::command]
pub fn get_case_by_id(
    app: AppHandle,
    session: State<'_, SessionState>,
    payload: GetCaseByIdPayload,
) -> CommandResult<CaseDto> {
    match CaseService::get_case_by_id(&app, &session, payload) {
        Ok(response) => CommandResult::ok(response),
        Err(error) => CommandResult::err(error),
    }
}

#[tauri::command]
pub fn update_case(
    app: AppHandle,
    session: State<'_, SessionState>,
    payload: UpdateCasePayload,
) -> CommandResult<UpdateCaseResponse> {
    match CaseService::update_case(&app, &session, payload) {
        Ok(response) => CommandResult::ok(response),
        Err(error) => CommandResult::err(error),
    }
}
