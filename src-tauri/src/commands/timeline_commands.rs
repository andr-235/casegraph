use tauri::{AppHandle, State};

use crate::domain::timeline::{
    CreateEventPayload, CreateEventResponse, GetEventByIdPayload, GetEventByIdResponse,
    GetTimelinePayload, GetTimelineResponse, UpdateEventPayload, UpdateEventResponse,
};
use crate::errors::app_error::CommandResult;
use crate::security::session::SessionState;
use crate::services::timeline_service::TimelineService;

#[tauri::command]
pub fn get_timeline(
    app: AppHandle,
    session: State<SessionState>,
    payload: GetTimelinePayload,
) -> CommandResult<GetTimelineResponse> {
    match TimelineService::get_timeline(&app, &session, payload) {
        Ok(data) => CommandResult::ok(data),
        Err(error) => CommandResult::err(error),
    }
}

#[tauri::command]
pub fn create_event(
    app: AppHandle,
    session: State<SessionState>,
    payload: CreateEventPayload,
) -> CommandResult<CreateEventResponse> {
    match TimelineService::create_event(&app, &session, payload) {
        Ok(data) => CommandResult::ok(data),
        Err(error) => CommandResult::err(error),
    }
}

#[tauri::command]
pub fn get_event_by_id(
    app: AppHandle,
    session: State<SessionState>,
    payload: GetEventByIdPayload,
) -> CommandResult<GetEventByIdResponse> {
    match TimelineService::get_event_by_id(&app, &session, payload) {
        Ok(response) => CommandResult::ok(response),
        Err(error) => CommandResult::err(error),
    }
}

#[tauri::command]
pub fn update_event(
    app: AppHandle,
    session: State<SessionState>,
    payload: UpdateEventPayload,
) -> CommandResult<UpdateEventResponse> {
    match TimelineService::update_event(&app, &session, payload) {
        Ok(response) => CommandResult::ok(response),
        Err(error) => CommandResult::err(error),
    }
}
