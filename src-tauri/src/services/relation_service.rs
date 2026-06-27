use tauri::AppHandle;
use uuid::Uuid;

use crate::domain::relations::{
    CreateRelationPayload, CreateRelationResponse, GetRelationByIdPayload, GetRelationByIdResponse,
    GetRelationsPayload, GetRelationsResponse, RelationDetailsDto, RelationListItemDto,
    SoftDeleteRelationPayload, SoftDeleteRelationResponse, UpdateRelationPayload,
    UpdateRelationResponse,
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

        write_relation_created_audit_best_effort(app, current_user, &relation_item);

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
        let current_user = &context.current_user;
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

        let old_relation = RelationRepository::get_details_by_id(conn, &case_id, &relation_id)?
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

        write_relation_updated_audit_best_effort(app, current_user, &old_relation, &relation);

        Ok(UpdateRelationResponse { relation })
    }

    pub fn soft_delete_relation(
        app: &AppHandle,
        session: &SessionState,
        payload: SoftDeleteRelationPayload,
    ) -> Result<SoftDeleteRelationResponse, AppErrorDto> {
        let context = require_protected_analyst_or_admin_for(app, session, "SOFT_DELETE_RELATION")?;
        let current_user = &context.current_user;
        let conn = &context.conn;

        let case_id =
            normalize_required_id(&payload.case_id, "ERR_CASE_REQUIRED", "Не выбрано дело.")?;

        let relation_id = normalize_required_id(
            &payload.relation_id,
            "ERR_RELATION_REQUIRED",
            "Не выбрана связь.",
        )?;

        let old_relation = RelationRepository::get_details_by_id(conn, &case_id, &relation_id)?
            .ok_or_else(|| AppErrorDto::new("ERR_RELATION_NOT_FOUND", "Связь не найдена.", None))?;

        RelationRepository::soft_delete_relation(conn, &case_id, &relation_id)?;

        write_relation_deleted_audit_best_effort(app, current_user, &old_relation);

        Ok(SoftDeleteRelationResponse { relation_id })
    }
}

fn write_relation_created_audit_best_effort(
    app: &AppHandle,
    current_user: &crate::security::session::CurrentUserDto,
    created_relation: &RelationListItemDto,
) {
    use crate::audit::audit_metadata;
    use crate::audit::audit_service::{AuditService, AuditWriteInput};
    use crate::domain::audit_action;

    let result = (|| {
        let technical_details = audit_metadata::relation_created(
            &created_relation.id,
            &created_relation.relation_code,
            &created_relation.source_object.id,
            &created_relation.target_object.id,
        )?;

        let new_value = audit_metadata::safe_relation_snapshot(
            &created_relation.relation_code,
            &created_relation.source_object.id,
            &created_relation.target_object.id,
            &created_relation.relation_type,
            &created_relation.confidence_level,
            Some(created_relation.basis.as_str()),
            created_relation
                .supporting_material
                .as_ref()
                .map(|m| m.id.as_str()),
            created_relation.include_in_report,
        )?;

        AuditService::write_best_effort(
            app,
            AuditWriteInput::success(current_user, audit_action::relation::CREATED)
                .with_case_id(created_relation.case_id.clone())
                .with_entity("relation", created_relation.id.clone())
                .with_snapshots(None, Some(new_value))
                .with_details(technical_details),
        );
        Ok::<(), crate::errors::app_error::AppErrorDto>(())
    })();

    if let Err(err) = result {
        eprintln!(
            "[audit] write_relation_created_audit failed: {}",
            err.message
        );
    }
}

fn write_relation_updated_audit_best_effort(
    app: &AppHandle,
    current_user: &crate::security::session::CurrentUserDto,
    old_relation: &RelationDetailsDto,
    new_relation: &RelationDetailsDto,
) {
    use crate::audit::audit_metadata;
    use crate::audit::audit_service::{AuditService, AuditWriteInput};
    use crate::domain::audit_action;

    let result = (|| {
        let mut changed = Vec::new();
        audit_metadata::push_changed(
            &mut changed,
            "sourceObjectId",
            &old_relation.source_object.id,
            &new_relation.source_object.id,
        );
        audit_metadata::push_changed(
            &mut changed,
            "targetObjectId",
            &old_relation.target_object.id,
            &new_relation.target_object.id,
        );
        audit_metadata::push_changed(
            &mut changed,
            "relationType",
            &old_relation.relation_type,
            &new_relation.relation_type,
        );
        audit_metadata::push_changed(
            &mut changed,
            "confidenceLevel",
            &old_relation.confidence_level,
            &new_relation.confidence_level,
        );
        audit_metadata::push_changed(
            &mut changed,
            "basis",
            &old_relation.basis,
            &new_relation.basis,
        );

        let old_material_id = old_relation
            .supporting_material
            .as_ref()
            .map(|m| m.id.as_str());
        let new_material_id = new_relation
            .supporting_material
            .as_ref()
            .map(|m| m.id.as_str());
        if old_material_id != new_material_id {
            changed.push("materialId");
        }

        audit_metadata::push_changed(
            &mut changed,
            "includeInReport",
            &old_relation.include_in_report,
            &new_relation.include_in_report,
        );

        if changed.is_empty() {
            return Ok(());
        }

        let is_toggle = changed.len() == 1 && changed[0] == "includeInReport";
        let action = if is_toggle {
            audit_action::relation::REPORT_INCLUDE_CHANGED
        } else {
            audit_action::relation::UPDATED
        };

        let technical_details = if is_toggle {
            audit_metadata::relation_report_include_changed(
                &new_relation.id,
                &new_relation.relation_code,
                new_relation.include_in_report,
            )?
        } else {
            audit_metadata::relation_updated(
                &new_relation.id,
                &new_relation.relation_code,
                &changed,
            )?
        };

        let old_val = audit_metadata::safe_relation_snapshot(
            &old_relation.relation_code,
            &old_relation.source_object.id,
            &old_relation.target_object.id,
            &old_relation.relation_type,
            &old_relation.confidence_level,
            Some(old_relation.basis.as_str()),
            old_relation
                .supporting_material
                .as_ref()
                .map(|m| m.id.as_str()),
            old_relation.include_in_report,
        )?;

        let new_val = audit_metadata::safe_relation_snapshot(
            &new_relation.relation_code,
            &new_relation.source_object.id,
            &new_relation.target_object.id,
            &new_relation.relation_type,
            &new_relation.confidence_level,
            Some(new_relation.basis.as_str()),
            new_relation
                .supporting_material
                .as_ref()
                .map(|m| m.id.as_str()),
            new_relation.include_in_report,
        )?;

        AuditService::write_best_effort(
            app,
            AuditWriteInput::success(current_user, action)
                .with_case_id(new_relation.case_id.clone())
                .with_entity("relation", new_relation.id.clone())
                .with_snapshots(Some(old_val), Some(new_val))
                .with_details(technical_details),
        );
        Ok::<(), crate::errors::app_error::AppErrorDto>(())
    })();

    if let Err(err) = result {
        eprintln!(
            "[audit] write_relation_updated_audit failed: {}",
            err.message
        );
    }
}

fn write_relation_deleted_audit_best_effort(
    app: &AppHandle,
    current_user: &crate::security::session::CurrentUserDto,
    old_relation: &RelationDetailsDto,
) {
    use crate::audit::audit_metadata;
    use crate::audit::audit_service::{AuditService, AuditWriteInput};
    use crate::domain::audit_action;

    let result = (|| {
        let technical_details =
            audit_metadata::relation_deleted(&old_relation.id, &old_relation.relation_code)?;

        let old_value = audit_metadata::safe_relation_snapshot(
            &old_relation.relation_code,
            &old_relation.source_object.id,
            &old_relation.target_object.id,
            &old_relation.relation_type,
            &old_relation.confidence_level,
            Some(old_relation.basis.as_str()),
            old_relation
                .supporting_material
                .as_ref()
                .map(|m| m.id.as_str()),
            old_relation.include_in_report,
        )?;

        AuditService::write_best_effort(
            app,
            AuditWriteInput::success(current_user, audit_action::relation::DELETED)
                .with_case_id(old_relation.case_id.clone())
                .with_entity("relation", old_relation.id.clone())
                .with_snapshots(Some(old_value), None)
                .with_details(technical_details),
        );
        Ok::<(), crate::errors::app_error::AppErrorDto>(())
    })();

    if let Err(err) = result {
        eprintln!(
            "[audit] write_relation_deleted_audit failed: {}",
            err.message
        );
    }
}
