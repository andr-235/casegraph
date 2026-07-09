use tauri::{AppHandle, State};

use crate::domain::case_overview::CaseOverviewDto;
use crate::domain::case_summary::{CaseSummaryDto, GetCaseSummaryPayload};
use crate::domain::case_overview::GetCaseOverviewPayload;
use crate::errors::app_error::CommandResult;
use crate::security::session::SessionState;
use crate::services::case_summary_service::CaseSummaryService;

#[tauri::command]
pub fn get_case_summary(
    app: AppHandle,
    session: State<SessionState>,
    payload: GetCaseSummaryPayload,
) -> CommandResult<CaseSummaryDto> {
    log::debug!(
        "[get_case_summary] START case_id={}",
        payload.case_id
    );

    match CaseSummaryService::get_case_summary(&app, &session, &payload.case_id) {
        Ok(summary) => {
            log::info!(
                "[get_case_summary] SUCCESS case_id={}",
                payload.case_id
            );
            CommandResult::ok(summary)
        }
        Err(error) => {
            log::error!(
                "[get_case_summary] ERROR case_id={} error={}",
                payload.case_id,
                error.message
            );
            CommandResult::err(error)
        }
    }
}

#[tauri::command]
pub fn get_case_overview(
    app: AppHandle,
    session: State<SessionState>,
    payload: GetCaseOverviewPayload,
) -> CommandResult<CaseOverviewDto> {
    log::debug!(
        "[get_case_overview] START case_id={}",
        payload.case_id
    );

    match CaseSummaryService::get_case_overview(&app, &session, &payload.case_id) {
        Ok(overview) => {
            log::info!(
                "[get_case_overview] SUCCESS case_id={}",
                payload.case_id
            );
            CommandResult::ok(overview)
        }
        Err(error) => {
            log::error!(
                "[get_case_overview] ERROR case_id={} error={}",
                payload.case_id,
                error.message
            );
            CommandResult::err(error)
        }
    }
}
