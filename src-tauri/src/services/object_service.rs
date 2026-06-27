use tauri::AppHandle;
use uuid::Uuid;

use crate::domain::objects::{
    CreateObjectPayload, CreateObjectResponse, GetObjectByIdPayload, GetObjectByIdResponse,
    GetObjectsPayload, GetObjectsResponse, LinkObjectToMaterialsPayload,
    LinkObjectToMaterialsResponse, ObjectDetailsDto, ObjectListItemDto, SoftDeleteObjectPayload,
    SoftDeleteObjectResponse, UpdateObjectPayload, UpdateObjectResponse,
};
use crate::errors::app_error::AppErrorDto;
use crate::repositories::case_repository::CaseRepository;
use crate::repositories::object_repository::{
    CreateObjectRecord, ObjectMaterialLinkRecord, ObjectRepository, UpdateObjectRecord,
};
use crate::security::session::SessionState;
use crate::services::object_validation::{
    normalize_confidence_note, normalize_link_reason, normalize_object_description,
    normalize_object_title, normalize_object_type, normalize_optional_value, normalize_required_id,
    normalize_unique_ids,
};
use crate::services::protected_service_context::{
    require_protected_analyst_or_admin_for, require_protected_user_for,
};

pub struct ObjectService;

impl ObjectService {
    pub fn create_object(
        app: &AppHandle,
        session: &SessionState,
        payload: CreateObjectPayload,
    ) -> Result<CreateObjectResponse, AppErrorDto> {
        let context = require_protected_analyst_or_admin_for(app, session, "CREATE_OBJECT")?;
        let current_user = &context.current_user;
        let conn = &context.conn;

        let case_id =
            normalize_required_id(&payload.case_id, "ERR_CASE_REQUIRED", "Не выбрано дело.")?;

        let object_type = normalize_object_type(&payload.object_type)?;
        let title = normalize_object_title(&payload.title)?;
        let value = normalize_optional_value(payload.value)?;
        let description = normalize_object_description(payload.description)?;
        let confidence_note = normalize_confidence_note(payload.confidence_note)?;

        let case_item = CaseRepository::get_case_by_id(conn, &case_id)?;

        if case_item.is_none() {
            return Err(AppErrorDto::new(
                "ERR_CASE_NOT_FOUND",
                "Дело не найдено.",
                None,
            ));
        }

        let object_id = Uuid::new_v4().to_string();
        let object_code =
            ObjectRepository::generate_next_object_code(conn, &case_id, &object_type)?;

        ObjectRepository::create(
            conn,
            CreateObjectRecord {
                id: object_id.clone(),
                case_id: case_id.clone(),
                object_code,
                object_type,
                title,
                value,
                description,
                is_key: payload.is_key.unwrap_or(false),
                confidence_note,
                include_in_report: payload.include_in_report.unwrap_or(true),
                created_by_user_id: current_user.user_id.clone(),
            },
        )?;

        let items = ObjectRepository::list_by_case(conn, &case_id)?;
        let object_item = items
            .into_iter()
            .find(|item| item.id == object_id)
            .ok_or_else(|| {
                AppErrorDto::new("ERR_OBJECT_NOT_FOUND", "Созданный объект не найден.", None)
            })?;

        write_object_created_audit_best_effort(app, current_user, &object_item);

        Ok(CreateObjectResponse { object_item })
    }

    pub fn get_objects(
        app: &AppHandle,
        session: &SessionState,
        payload: GetObjectsPayload,
    ) -> Result<GetObjectsResponse, AppErrorDto> {
        let context = require_protected_user_for(app, session, "GET_OBJECTS")?;
        let conn = &context.conn;

        let case_id =
            normalize_required_id(&payload.case_id, "ERR_CASE_REQUIRED", "Не выбрано дело.")?;

        let items = ObjectRepository::list_by_case(conn, &case_id)?;

        Ok(GetObjectsResponse { items })
    }

    pub fn get_object_by_id(
        app: &AppHandle,
        session: &SessionState,
        payload: GetObjectByIdPayload,
    ) -> Result<GetObjectByIdResponse, AppErrorDto> {
        let context = require_protected_user_for(app, session, "GET_OBJECT_BY_ID")?;
        let conn = &context.conn;

        let case_id =
            normalize_required_id(&payload.case_id, "ERR_CASE_REQUIRED", "Не выбрано дело.")?;

        let object_id = normalize_required_id(
            &payload.object_id,
            "ERR_OBJECT_REQUIRED",
            "Не выбран объект.",
        )?;

        let object_item = ObjectRepository::find_by_id(conn, &case_id, &object_id)?
            .ok_or_else(|| AppErrorDto::new("ERR_OBJECT_NOT_FOUND", "Объект не найден.", None))?;

        Ok(GetObjectByIdResponse { object_item })
    }

    pub fn link_object_to_materials(
        app: &AppHandle,
        session: &SessionState,
        payload: LinkObjectToMaterialsPayload,
    ) -> Result<LinkObjectToMaterialsResponse, AppErrorDto> {
        let context =
            require_protected_analyst_or_admin_for(app, session, "LINK_OBJECT_TO_MATERIALS")?;
        let current_user = &context.current_user;
        let conn = &context.conn;

        let case_id =
            normalize_required_id(&payload.case_id, "ERR_CASE_REQUIRED", "Не выбрано дело.")?;

        let object_id = normalize_required_id(
            &payload.object_id,
            "ERR_OBJECT_REQUIRED",
            "Не выбран объект.",
        )?;

        let material_ids = normalize_unique_ids(payload.material_ids);
        let link_reason = normalize_link_reason(payload.link_reason)?;

        ObjectRepository::validate_materials_belong_to_case(conn, &case_id, &material_ids)?;

        let old_object = ObjectRepository::find_by_id(conn, &case_id, &object_id)?
            .ok_or_else(|| AppErrorDto::new("ERR_OBJECT_NOT_FOUND", "Объект не найден.", None))?;

        let records = material_ids
            .clone()
            .into_iter()
            .map(|material_id| ObjectMaterialLinkRecord {
                object_id: object_id.clone(),
                material_id,
                link_reason: link_reason.clone(),
                created_by_user_id: current_user.user_id.clone(),
            })
            .collect();

        ObjectRepository::replace_material_links(conn, &case_id, &object_id, records)?;

        let object_item =
            ObjectRepository::find_by_id(conn, &case_id, &object_id)?.ok_or_else(|| {
                AppErrorDto::new(
                    "ERR_OBJECT_NOT_FOUND",
                    "Объект найден но не найден после обновления связей.",
                    None,
                )
            })?;

        write_object_links_changed_audit_best_effort(
            app,
            current_user,
            &old_object,
            &object_item,
            &material_ids,
        );

        Ok(LinkObjectToMaterialsResponse { object_item })
    }

    pub fn update_object(
        app: &AppHandle,
        session: &SessionState,
        payload: UpdateObjectPayload,
    ) -> Result<UpdateObjectResponse, AppErrorDto> {
        let context = require_protected_analyst_or_admin_for(app, session, "UPDATE_OBJECT")?;
        let current_user = &context.current_user;
        let conn = &context.conn;

        let case_id =
            normalize_required_id(&payload.case_id, "ERR_CASE_REQUIRED", "Не выбрано дело.")?;

        let object_id = normalize_required_id(
            &payload.object_id,
            "ERR_OBJECT_REQUIRED",
            "Не выбран объект.",
        )?;

        let title = normalize_object_title(&payload.title)?;
        let value = normalize_optional_value(payload.value)?;
        let description = normalize_object_description(payload.description)?;
        let confidence_note = normalize_confidence_note(payload.confidence_note)?;

        let old_object = ObjectRepository::find_by_id(conn, &case_id, &object_id)?
            .ok_or_else(|| AppErrorDto::new("ERR_OBJECT_NOT_FOUND", "Объект не найден.", None))?;

        ObjectRepository::update_object(
            conn,
            UpdateObjectRecord {
                case_id: case_id.clone(),
                object_id: object_id.clone(),
                title,
                value,
                description,
                is_key: payload.is_key,
                confidence_note,
                include_in_report: payload.include_in_report,
            },
        )?;

        let object_item =
            ObjectRepository::find_by_id(conn, &case_id, &object_id)?.ok_or_else(|| {
                AppErrorDto::new(
                    "ERR_OBJECT_NOT_FOUND",
                    "Объект не найден после обновления.",
                    None,
                )
            })?;

        write_object_updated_audit_best_effort(app, current_user, &old_object, &object_item);

        Ok(UpdateObjectResponse { object_item })
    }

    pub fn soft_delete_object(
        app: &AppHandle,
        session: &SessionState,
        payload: SoftDeleteObjectPayload,
    ) -> Result<SoftDeleteObjectResponse, AppErrorDto> {
        let context = require_protected_analyst_or_admin_for(app, session, "SOFT_DELETE_OBJECT")?;
        let current_user = &context.current_user;
        let conn = &context.conn;

        let case_id =
            normalize_required_id(&payload.case_id, "ERR_CASE_REQUIRED", "Не выбрано дело.")?;

        let object_id = normalize_required_id(
            &payload.object_id,
            "ERR_OBJECT_REQUIRED",
            "Не выбран объект.",
        )?;

        let old_object = ObjectRepository::find_by_id(conn, &case_id, &object_id)?
            .ok_or_else(|| AppErrorDto::new("ERR_OBJECT_NOT_FOUND", "Объект не найден.", None))?;

        ObjectRepository::soft_delete_object(conn, &case_id, &object_id)?;

        write_object_deleted_audit_best_effort(app, current_user, &old_object);

        Ok(SoftDeleteObjectResponse { object_id })
    }
}

fn write_object_created_audit_best_effort(
    app: &AppHandle,
    current_user: &crate::security::session::CurrentUserDto,
    created_object: &ObjectListItemDto,
) {
    use crate::audit::audit_metadata;
    use crate::domain::audit_action;
    use crate::services::audit_service::{AuditService, AuditWriteInput};

    let result = (|| {
        let technical_details = audit_metadata::object_created(
            &created_object.id,
            &created_object.object_code,
            &created_object.object_type,
        )?;

        let new_value = audit_metadata::safe_object_snapshot(
            &created_object.object_code,
            &created_object.object_type,
            &created_object.title,
            Some(created_object.description.as_str()),
            created_object.is_key,
            Some(created_object.include_in_report),
        )?;

        AuditService::write_best_effort(
            app,
            AuditWriteInput::success(current_user, audit_action::object::CREATED)
                .with_case_id(created_object.case_id.clone())
                .with_entity("object", created_object.id.clone())
                .with_snapshots(None, Some(new_value))
                .with_details(technical_details),
        );
        Ok::<(), crate::errors::app_error::AppErrorDto>(())
    })();

    if let Err(err) = result {
        eprintln!("[audit] write_object_created_audit failed: {}", err.message);
    }
}

fn write_object_updated_audit_best_effort(
    app: &AppHandle,
    current_user: &crate::security::session::CurrentUserDto,
    old_object: &ObjectDetailsDto,
    new_object: &ObjectDetailsDto,
) {
    use crate::audit::audit_metadata;
    use crate::domain::audit_action;
    use crate::services::audit_service::{AuditService, AuditWriteInput};

    let result = (|| {
        let mut changed = Vec::new();
        audit_metadata::push_changed(
            &mut changed,
            "objectType",
            &old_object.object_type,
            &new_object.object_type,
        );
        audit_metadata::push_changed(&mut changed, "title", &old_object.title, &new_object.title);
        audit_metadata::push_changed(
            &mut changed,
            "description",
            &old_object.description,
            &new_object.description,
        );
        audit_metadata::push_changed(
            &mut changed,
            "isKeyObject",
            &old_object.is_key,
            &new_object.is_key,
        );
        audit_metadata::push_changed(
            &mut changed,
            "includeInReport",
            &old_object.include_in_report,
            &new_object.include_in_report,
        );

        if changed.is_empty() {
            return Ok(());
        }

        let is_key_toggle = changed.len() == 1 && changed[0] == "isKeyObject";
        let action = if is_key_toggle {
            audit_action::object::KEY_FLAG_CHANGED
        } else {
            audit_action::object::UPDATED
        };

        let technical_details = if is_key_toggle {
            audit_metadata::object_key_flag_changed(
                &new_object.id,
                &new_object.object_code,
                new_object.is_key,
            )?
        } else {
            audit_metadata::object_updated(&new_object.id, &new_object.object_code, &changed)?
        };

        let old_val = audit_metadata::safe_object_snapshot(
            &old_object.object_code,
            &old_object.object_type,
            &old_object.title,
            Some(old_object.description.as_str()),
            old_object.is_key,
            Some(old_object.include_in_report),
        )?;

        let new_val = audit_metadata::safe_object_snapshot(
            &new_object.object_code,
            &new_object.object_type,
            &new_object.title,
            Some(new_object.description.as_str()),
            new_object.is_key,
            Some(new_object.include_in_report),
        )?;

        AuditService::write_best_effort(
            app,
            AuditWriteInput::success(current_user, action)
                .with_case_id(new_object.case_id.clone())
                .with_entity("object", new_object.id.clone())
                .with_snapshots(Some(old_val), Some(new_val))
                .with_details(technical_details),
        );
        Ok::<(), crate::errors::app_error::AppErrorDto>(())
    })();

    if let Err(err) = result {
        eprintln!("[audit] write_object_updated_audit failed: {}", err.message);
    }
}

fn write_object_links_changed_audit_best_effort(
    app: &AppHandle,
    current_user: &crate::security::session::CurrentUserDto,
    old_object: &ObjectDetailsDto,
    new_object: &ObjectDetailsDto,
    material_ids: &[String],
) {
    use crate::audit::audit_metadata;
    use crate::domain::audit_action;
    use crate::services::audit_service::{AuditService, AuditWriteInput};

    let result = (|| {
        let technical_details = audit_metadata::object_material_links_changed(
            &new_object.id,
            &new_object.object_code,
            material_ids,
        )?;

        let old_val = audit_metadata::safe_object_snapshot(
            &old_object.object_code,
            &old_object.object_type,
            &old_object.title,
            Some(old_object.description.as_str()),
            old_object.is_key,
            Some(old_object.include_in_report),
        )?;

        let new_val = audit_metadata::safe_object_snapshot(
            &new_object.object_code,
            &new_object.object_type,
            &new_object.title,
            Some(new_object.description.as_str()),
            new_object.is_key,
            Some(new_object.include_in_report),
        )?;

        AuditService::write_best_effort(
            app,
            AuditWriteInput::success(current_user, audit_action::object::MATERIAL_LINKS_CHANGED)
                .with_case_id(new_object.case_id.clone())
                .with_entity("object", new_object.id.clone())
                .with_snapshots(Some(old_val), Some(new_val))
                .with_details(technical_details),
        );
        Ok::<(), crate::errors::app_error::AppErrorDto>(())
    })();

    if let Err(err) = result {
        eprintln!(
            "[audit] write_object_links_changed_audit failed: {}",
            err.message
        );
    }
}

fn write_object_deleted_audit_best_effort(
    app: &AppHandle,
    current_user: &crate::security::session::CurrentUserDto,
    old_object: &ObjectDetailsDto,
) {
    use crate::audit::audit_metadata;
    use crate::domain::audit_action;
    use crate::services::audit_service::{AuditService, AuditWriteInput};

    let result = (|| {
        let technical_details =
            audit_metadata::object_deleted(&old_object.id, &old_object.object_code)?;

        let old_value = audit_metadata::safe_object_snapshot(
            &old_object.object_code,
            &old_object.object_type,
            &old_object.title,
            Some(old_object.description.as_str()),
            old_object.is_key,
            Some(old_object.include_in_report),
        )?;

        AuditService::write_best_effort(
            app,
            AuditWriteInput::success(current_user, audit_action::object::DELETED)
                .with_case_id(old_object.case_id.clone())
                .with_entity("object", old_object.id.clone())
                .with_snapshots(Some(old_value), None)
                .with_details(technical_details),
        );
        Ok::<(), crate::errors::app_error::AppErrorDto>(())
    })();

    if let Err(err) = result {
        eprintln!("[audit] write_object_deleted_audit failed: {}", err.message);
    }
}
