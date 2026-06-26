use tauri::{AppHandle, State};

use crate::domain::materials::{
    CreateMaterialPayload, CreateMaterialResponse, GetMaterialsPayload, MaterialDto,
    UpdateMaterialPayload, UpdateMaterialResponse,
};
use crate::errors::app_error::CommandResult;
use crate::security::session::SessionState;
use crate::services::material_service::MaterialService;

#[tauri::command]
pub fn get_materials(
    app: AppHandle,
    session: State<'_, SessionState>,
    payload: GetMaterialsPayload,
) -> CommandResult<Vec<MaterialDto>> {
    match MaterialService::get_materials(&app, &session, payload) {
        Ok(response) => CommandResult::ok(response),
        Err(error) => CommandResult::err(error),
    }
}

#[tauri::command]
pub fn create_material(
    app: AppHandle,
    session: State<'_, SessionState>,
    payload: CreateMaterialPayload,
) -> CommandResult<CreateMaterialResponse> {
    match MaterialService::create_material(&app, &session, payload) {
        Ok(response) => CommandResult::ok(response),
        Err(error) => CommandResult::err(error),
    }
}

#[tauri::command]
pub fn update_material(
    app: AppHandle,
    session: State<'_, SessionState>,
    payload: UpdateMaterialPayload,
) -> CommandResult<UpdateMaterialResponse> {
    match MaterialService::update_material(&app, &session, payload) {
        Ok(response) => CommandResult::ok(response),
        Err(error) => CommandResult::err(error),
    }
}
