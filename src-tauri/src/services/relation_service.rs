use tauri::AppHandle;
use uuid::Uuid;

use crate::domain::relations::{
    CreateRelationPayload, CreateRelationResponse, GetRelationByIdPayload, GetRelationByIdResponse,
    GetRelationsPayload, GetRelationsResponse, SoftDeleteRelationPayload,
    SoftDeleteRelationResponse, UpdateRelationPayload, UpdateRelationResponse,
};
use crate::errors::app_error::AppErrorDto;
use crate::repositories::case_repository::CaseRepository;
use crate::repositories::relation_repository::{
    CreateRelationRecord, RelationRepository, UpdateRelationRecord,
};
use crate::security::session::SessionState;
use crate::services::protected_service_context::{
    require_protected_analyst_or_admin_for, require_protected_user_for,
};
use crate::services::relation_validation::{
    normalize_analyst_comment, normalize_confidence_level, normalize_optional_id,
    normalize_relation_basis, normalize_relation_title, normalize_relation_type,
    normalize_required_id, validate_relation_endpoints,
};

pub struct RelationService;

impl RelationService {
    pub fn create_relation(
        app: &AppHandle,
        session: &SessionState,
        payload: CreateRelationPayload,
    ) -> Result<CreateRelationResponse, AppErrorDto> {
        let context = require_protected_analyst_or_admin_for(app, session, "CREATE_RELATION")?;
        let current_user = &context.current_user;
        let conn = &context.conn;

        let case_id =
            normalize_required_id(&payload.case_id, "ERR_CASE_REQUIRED", "Не выбрано дело.")?;

        let source_object_id = normalize_required_id(
            &payload.source_object_id,
            "ERR_SOURCE_OBJECT_REQUIRED",
            "Не выбран исходный объект связи.",
        )?;

        let target_object_id = normalize_required_id(
            &payload.target_object_id,
            "ERR_TARGET_OBJECT_REQUIRED",
            "Не выбран целевой объект связи.",
        )?;

        validate_relation_endpoints(&source_object_id, &target_object_id)?;

        let relation_type = normalize_relation_type(&payload.relation_type)?;
        let title = normalize_relation_title(&payload.title)?;
        let basis = normalize_relation_basis(&payload.basis)?;
        let confidence_level = normalize_confidence_level(&payload.confidence_level)?;
        let supporting_material_id = normalize_optional_id(&payload.supporting_material_id);
        let analyst_comment = normalize_analyst_comment(&payload.analyst_comment)?;

        CaseRepository::get_case_by_id(conn, &case_id)?
            .ok_or_else(|| AppErrorDto::new("ERR_CASE_NOT_FOUND", "Дело не найдено.", None))?;

        let source_object = RelationRepository::find_object_case_info(conn, &source_object_id)?
            .ok_or_else(|| {
                AppErrorDto::new("ERR_OBJECT_NOT_FOUND", "Первый объект не найден.", None)
            })?;

        let target_object = RelationRepository::find_object_case_info(conn, &target_object_id)?
            .ok_or_else(|| {
                AppErrorDto::new("ERR_OBJECT_NOT_FOUND", "Второй объект не найден.", None)
            })?;

        if source_object.case_id != case_id || target_object.case_id != case_id {
            return Err(AppErrorDto::new(
                "ERR_RELATION_OBJECT_CASE_MISMATCH",
                "Объекты не относятся к выбранному делу.",
                None,
            ));
        }

        if let Some(material_id) = supporting_material_id.as_deref() {
            let material = RelationRepository::find_material_case_info(conn, material_id)?
                .ok_or_else(|| {
                    AppErrorDto::new("ERR_MATERIAL_NOT_FOUND", "Материал не найден.", None)
                })?;

            if material.case_id != case_id {
                return Err(AppErrorDto::new(
                    "ERR_RELATION_MATERIAL_CASE_MISMATCH",
                    "Подтверждающий материал не относится к выбранному делу.",
                    None,
                ));
            }
        }

        let relation_id = Uuid::new_v4().to_string();
        let relation_code = RelationRepository::generate_next_relation_code(conn, &case_id)?;

        RelationRepository::create(
            conn,
            CreateRelationRecord {
                id: relation_id.clone(),
                case_id,
                relation_code,
                source_object_id,
                target_object_id,
                relation_type,
                title,
                basis,
                confidence_level,
                supporting_material_id,
                analyst_comment,
                include_in_report: payload.include_in_report.unwrap_or(true),
                created_by_user_id: current_user.user_id.clone(),
            },
        )?;

        let relation_item = RelationRepository::get_by_id(conn, &relation_id)?
            .ok_or_else(|| AppErrorDto::new("ERR_RELATION_NOT_FOUND", "Связь не найдена.", None))?;

        Ok(CreateRelationResponse { relation_item })
    }

    pub fn get_relations(
        app: &AppHandle,
        session: &SessionState,
        payload: GetRelationsPayload,
    ) -> Result<GetRelationsResponse, AppErrorDto> {
        let context = require_protected_user_for(app, session, "GET_RELATIONS")?;
        let conn = &context.conn;

        let case_id =
            normalize_required_id(&payload.case_id, "ERR_CASE_REQUIRED", "Не выбрано дело.")?;

        CaseRepository::get_case_by_id(conn, &case_id)?
            .ok_or_else(|| AppErrorDto::new("ERR_CASE_NOT_FOUND", "Дело не найдено.", None))?;

        let items = RelationRepository::list_by_case(conn, &case_id)?;

        Ok(GetRelationsResponse { items })
    }

    pub fn get_relation_by_id(
        app: &AppHandle,
        session: &SessionState,
        payload: GetRelationByIdPayload,
    ) -> Result<GetRelationByIdResponse, AppErrorDto> {
        let context = require_protected_user_for(app, session, "GET_RELATION_BY_ID")?;
        let conn = &context.conn;

        let case_id =
            normalize_required_id(&payload.case_id, "ERR_CASE_REQUIRED", "Не выбрано дело.")?;

        let relation_id = normalize_required_id(
            &payload.relation_id,
            "ERR_RELATION_REQUIRED",
            "Не выбрана связь.",
        )?;

        let relation = RelationRepository::get_details_by_id(conn, &case_id, &relation_id)?
            .ok_or_else(|| AppErrorDto::new("ERR_RELATION_NOT_FOUND", "Связь не найдена.", None))?;

        Ok(GetRelationByIdResponse { relation })
    }

    pub fn update_relation(
        app: &AppHandle,
        session: &SessionState,
        payload: UpdateRelationPayload,
    ) -> Result<UpdateRelationResponse, AppErrorDto> {
        let context = require_protected_analyst_or_admin_for(app, session, "UPDATE_RELATION")?;
        let conn = &context.conn;

        let case_id =
            normalize_required_id(&payload.case_id, "ERR_CASE_REQUIRED", "Не выбрано дело.")?;

        let relation_id = normalize_required_id(
            &payload.relation_id,
            "ERR_RELATION_REQUIRED",
            "Не выбрана связь.",
        )?;

        let relation_type = normalize_relation_type(&payload.relation_type)?;
        let title = normalize_relation_title(&payload.title)?;
        let basis = normalize_relation_basis(&payload.basis)?;
        let confidence_level = normalize_confidence_level(&payload.confidence_level)?;
        let supporting_material_id = normalize_optional_id(&payload.supporting_material_id);
        let analyst_comment = normalize_analyst_comment(&payload.analyst_comment)?;

        RelationRepository::get_details_by_id(conn, &case_id, &relation_id)?
            .ok_or_else(|| AppErrorDto::new("ERR_RELATION_NOT_FOUND", "Связь не найдена.", None))?;

        if let Some(material_id) = supporting_material_id.as_deref() {
            let material = RelationRepository::find_material_case_info(conn, material_id)?
                .ok_or_else(|| {
                    AppErrorDto::new("ERR_MATERIAL_NOT_FOUND", "Материал не найден.", None)
                })?;

            if material.case_id != case_id {
                return Err(AppErrorDto::new(
                    "ERR_RELATION_MATERIAL_CASE_MISMATCH",
                    "Подтверждающий материал не относится к выбранному делу.",
                    None,
                ));
            }
        }

        RelationRepository::update_relation(
            conn,
            UpdateRelationRecord {
                case_id: case_id.clone(),
                relation_id: relation_id.clone(),
                relation_type,
                title,
                basis,
                confidence_level,
                supporting_material_id,
                analyst_comment,
                include_in_report: payload.include_in_report,
            },
        )?;

        let relation = RelationRepository::get_details_by_id(conn, &case_id, &relation_id)?
            .ok_or_else(|| AppErrorDto::new("ERR_RELATION_NOT_FOUND", "Связь не найдена.", None))?;

        Ok(UpdateRelationResponse { relation })
    }

    pub fn soft_delete_relation(
        app: &AppHandle,
        session: &SessionState,
        payload: SoftDeleteRelationPayload,
    ) -> Result<SoftDeleteRelationResponse, AppErrorDto> {
        let context = require_protected_analyst_or_admin_for(app, session, "SOFT_DELETE_RELATION")?;
        let conn = &context.conn;

        let case_id =
            normalize_required_id(&payload.case_id, "ERR_CASE_REQUIRED", "Не выбрано дело.")?;

        let relation_id = normalize_required_id(
            &payload.relation_id,
            "ERR_RELATION_REQUIRED",
            "Не выбрана связь.",
        )?;

        let _existing = RelationRepository::get_details_by_id(conn, &case_id, &relation_id)?
            .ok_or_else(|| AppErrorDto::new("ERR_RELATION_NOT_FOUND", "Связь не найдена.", None))?;

        RelationRepository::soft_delete_relation(conn, &case_id, &relation_id)?;

        Ok(SoftDeleteRelationResponse { relation_id })
    }
}
