use tauri::{AppHandle, State};

use crate::domain::relations::{
    CreateRelationPayload, CreateRelationResponse, GetRelationsPayload, GetRelationsResponse,
};
use crate::errors::app_error::CommandResult;
use crate::security::session::SessionState;
use crate::services::relation_service::RelationService;

#[tauri::command]
pub fn create_relation(
    app: AppHandle,
    session: State<SessionState>,
    payload: CreateRelationPayload,
) -> CommandResult<CreateRelationResponse> {
    match RelationService::create_relation(&app, &session, payload) {
        Ok(response) => CommandResult::ok(response),
        Err(error) => CommandResult::err(error),
    }
}

#[tauri::command]
pub fn get_relations(
    app: AppHandle,
    session: State<SessionState>,
    payload: GetRelationsPayload,
) -> CommandResult<GetRelationsResponse> {
    match RelationService::get_relations(&app, &session, payload) {
        Ok(response) => CommandResult::ok(response),
        Err(error) => CommandResult::err(error),
    }
}
