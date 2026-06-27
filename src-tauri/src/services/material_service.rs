use tauri::AppHandle;
use uuid::Uuid;

use crate::domain::material_integrity_status::{
    MATERIAL_INTEGRITY_NOT_CHECKED, MATERIAL_INTEGRITY_OK,
};
use crate::domain::material_type::is_valid_material_type;
use crate::domain::materials::{
    CreateMaterialPayload, CreateMaterialResponse, DeleteMaterialPayload, DeleteMaterialResponse,
    GetMaterialsPayload, MaterialDto, UpdateMaterialPayload, UpdateMaterialResponse,
};
use crate::errors::app_error::AppErrorDto;
use crate::repositories::case_repository::CaseRepository;
use crate::repositories::material_repository::{
    CreateMaterialRecord, MaterialRepository, MaterialRow, UpdateMaterialRecord,
};
use crate::security::session::SessionState;
use crate::services::protected_service_context::require_protected_user_for;
use crate::storage::material_file_storage::import_material_file;

pub struct MaterialService;

impl MaterialService {
    pub fn get_materials(
        app: &AppHandle,
        session: &SessionState,
        payload: GetMaterialsPayload,
    ) -> Result<Vec<MaterialDto>, AppErrorDto> {
        let context = require_protected_user_for(app, session, "GET_MATERIALS")?;
        let conn = &context.conn;

        let case_id = payload.case_id.trim().to_string();

        validate_case_id(&case_id)?;

        ensure_case_exists(conn, &case_id)?;

        let rows = MaterialRepository::get_materials_by_case_id(conn, &case_id)?;

        Ok(rows.into_iter().map(material_row_to_dto).collect())
    }

    pub fn create_material(
        app: &AppHandle,
        session: &SessionState,
        payload: CreateMaterialPayload,
    ) -> Result<CreateMaterialResponse, AppErrorDto> {
        let context = require_protected_user_for(app, session, "CREATE_MATERIAL")?;
        let current_user = &context.current_user;
        let conn = &context.conn;

        let case_id = payload.case_id.trim().to_string();
        let title = payload.title.trim().to_string();
        let material_type = payload.material_type.trim().to_string();
        let source_name = payload.source_name.unwrap_or_default().trim().to_string();
        let description = payload.description.unwrap_or_default().trim().to_string();
        let captured_at = normalize_optional_string(payload.captured_at);

        validate_case_id(&case_id)?;
        validate_create_material_payload(&title, &material_type)?;

        ensure_case_exists(conn, &case_id)?;

        let material_code = MaterialRepository::get_next_material_code(conn, &case_id)?;
        let material_id = Uuid::new_v4().to_string();

        let source_file_path = normalize_optional_string(payload.source_file_path);

        let imported_file = match source_file_path {
            Some(path) => Some(import_material_file(app, &case_id, &material_id, &path)?),
            None => None,
        };

        let (
            original_file_name,
            original_path,
            stored_file_path,
            file_size,
            mime_type,
            sha256,
            integrity_status,
        ) = match imported_file {
            Some(file) => (
                Some(file.original_file_name),
                Some(file.original_path),
                Some(file.stored_file_path),
                Some(file.file_size),
                file.mime_type,
                Some(file.sha256),
                MATERIAL_INTEGRITY_OK.to_string(),
            ),
            None => (
                None,
                None,
                None,
                None,
                None,
                None,
                MATERIAL_INTEGRITY_NOT_CHECKED.to_string(),
            ),
        };

        MaterialRepository::create_material(
            conn,
            CreateMaterialRecord {
                id: material_id.clone(),
                case_id,
                material_code,
                title,
                material_type,
                source_name,
                description,
                captured_at,
                include_in_report: payload.include_in_report,
                original_file_name,
                original_path,
                stored_file_path,
                file_size,
                mime_type,
                sha256,
                integrity_status,
                created_by_user_id: current_user.user_id.clone(),
            },
        )?;

        let created_material = MaterialRepository::get_material_by_id(conn, &material_id)?
            .ok_or_else(|| {
                AppErrorDto::new(
                    "ERR_MATERIAL_NOT_FOUND_AFTER_CREATE",
                    "Материал создан, но не найден после сохранения.",
                    None,
                )
            })?;

        write_material_imported_audit_best_effort(app, current_user, &created_material);

        Ok(CreateMaterialResponse {
            material: material_row_to_dto(created_material),
        })
    }

    pub fn delete_material(
        app: &AppHandle,
        session: &SessionState,
        payload: DeleteMaterialPayload,
    ) -> Result<DeleteMaterialResponse, AppErrorDto> {
        let context = require_protected_user_for(app, session, "DELETE_MATERIAL")?;
        let current_user = &context.current_user;
        let conn = &context.conn;

        let case_id = payload.case_id.trim().to_string();
        let material_id = payload.material_id.trim().to_string();

        validate_case_id(&case_id)?;

        if material_id.is_empty() {
            return Err(AppErrorDto::new(
                "ERR_VALIDATION",
                "Не указан идентификатор материала.",
                None,
            ));
        }

        ensure_case_exists(conn, &case_id)?;

        let old_material =
            MaterialRepository::get_material_by_id(conn, &material_id)?.ok_or_else(|| {
                AppErrorDto::new("ERR_MATERIAL_NOT_FOUND", "Материал не найден.", None)
            })?;

        MaterialRepository::soft_delete_material(conn, &case_id, &material_id)?;

        write_material_deleted_audit_best_effort(app, current_user, &old_material);

        Ok(DeleteMaterialResponse { material_id })
    }

    pub fn update_material(
        app: &AppHandle,
        session: &SessionState,
        payload: UpdateMaterialPayload,
    ) -> Result<UpdateMaterialResponse, AppErrorDto> {
        let context = require_protected_user_for(app, session, "UPDATE_MATERIAL")?;
        let current_user = &context.current_user;
        let conn = &context.conn;

        let case_id = payload.case_id.trim().to_string();
        let material_id = payload.material_id.trim().to_string();
        let title = payload.title.trim().to_string();
        let material_type = payload.material_type.trim().to_string();
        let source_name = payload.source_name.unwrap_or_default().trim().to_string();
        let description = payload.description.unwrap_or_default().trim().to_string();
        let captured_at = normalize_optional_string(payload.captured_at);

        validate_case_id(&case_id)?;

        if material_id.is_empty() {
            return Err(AppErrorDto::new(
                "ERR_VALIDATION",
                "Не указан идентификатор материала.",
                None,
            ));
        }

        validate_create_material_payload(&title, &material_type)?;

        ensure_case_exists(conn, &case_id)?;

        let old_material =
            MaterialRepository::get_material_by_id(conn, &material_id)?.ok_or_else(|| {
                AppErrorDto::new("ERR_MATERIAL_NOT_FOUND", "Материал не найден.", None)
            })?;

        MaterialRepository::update_material(
            conn,
            UpdateMaterialRecord {
                material_id: material_id.clone(),
                case_id,
                title,
                material_type,
                source_name,
                description,
                captured_at,
                include_in_report: payload.include_in_report,
            },
        )?;

        let updated_material = MaterialRepository::get_material_by_id(conn, &material_id)?
            .ok_or_else(|| {
                AppErrorDto::new(
                    "ERR_MATERIAL_NOT_FOUND_AFTER_UPDATE",
                    "Материал обновлён, но не найден после сохранения.",
                    None,
                )
            })?;

        write_material_updated_audit_best_effort(
            app,
            current_user,
            &old_material,
            &updated_material,
        );

        Ok(UpdateMaterialResponse {
            material: material_row_to_dto(updated_material),
        })
    }
}

fn validate_case_id(case_id: &str) -> Result<(), AppErrorDto> {
    if case_id.is_empty() {
        return Err(AppErrorDto::new(
            "ERR_VALIDATION",
            "Не указан идентификатор дела.",
            None,
        ));
    }

    Ok(())
}

fn validate_create_material_payload(title: &str, material_type: &str) -> Result<(), AppErrorDto> {
    if title.len() < 2 {
        return Err(AppErrorDto::new(
            "ERR_VALIDATION",
            "Название материала должно содержать минимум 2 символа.",
            None,
        ));
    }

    if !is_valid_material_type(material_type) {
        return Err(AppErrorDto::new(
            "ERR_VALIDATION",
            "Недопустимый тип материала.",
            None,
        ));
    }

    Ok(())
}

fn ensure_case_exists(conn: &rusqlite::Connection, case_id: &str) -> Result<(), AppErrorDto> {
    let case_exists = CaseRepository::get_case_by_id(conn, case_id)?.is_some();

    if !case_exists {
        return Err(AppErrorDto::new(
            "ERR_CASE_NOT_FOUND",
            "Дело не найдено.",
            None,
        ));
    }

    Ok(())
}

fn normalize_optional_string(value: Option<String>) -> Option<String> {
    value
        .map(|item| item.trim().to_string())
        .filter(|item| !item.is_empty())
}

fn material_row_to_dto(row: MaterialRow) -> MaterialDto {
    MaterialDto {
        id: row.id,
        case_id: row.case_id,
        material_code: row.material_code,
        title: row.title,
        material_type: row.material_type,
        source_name: row.source_name,
        description: row.description,
        captured_at: row.captured_at,
        include_in_report: row.include_in_report == 1,
        original_file_name: row.original_file_name,
        original_path: row.original_path,
        stored_file_path: row.stored_file_path,
        file_size: row.file_size,
        mime_type: row.mime_type,
        sha256: row.sha256,
        integrity_status: row.integrity_status,
        created_by_user_id: row.created_by_user_id,
        created_at: row.created_at,
        updated_at: row.updated_at,
    }
}

fn write_material_imported_audit_best_effort(
    app: &AppHandle,
    current_user: &crate::security::session::CurrentUserDto,
    created_material: &MaterialRow,
) {
    use crate::audit::audit_metadata;
    use crate::domain::audit_action;
    use crate::audit::audit_service::{AuditService, AuditWriteInput};

    let result = (|| {
        let technical_details = audit_metadata::material_imported(
            &created_material.id,
            &created_material.material_code,
            created_material.original_file_name.as_deref().unwrap_or(""),
        )?;

        let new_value = audit_metadata::safe_material_snapshot(
            &created_material.material_code,
            created_material.original_file_name.as_deref().unwrap_or(""),
            &created_material.material_type,
            created_material.file_size,
            created_material.sha256.as_deref(),
            Some(created_material.integrity_status.as_str()),
            Some(created_material.description.as_str()),
            created_material.captured_at.as_deref(),
            created_material.include_in_report == 1,
        )?;

        AuditService::write_best_effort(
            app,
            AuditWriteInput::success(current_user, audit_action::material::IMPORTED)
                .with_case_id(created_material.case_id.clone())
                .with_entity("material", created_material.id.clone())
                .with_snapshots(None, Some(new_value))
                .with_details(technical_details),
        );
        Ok::<(), crate::errors::app_error::AppErrorDto>(())
    })();

    if let Err(err) = result {
        eprintln!(
            "[audit] write_material_imported_audit failed: {}",
            err.message
        );
    }
}

fn write_material_updated_audit_best_effort(
    app: &AppHandle,
    current_user: &crate::security::session::CurrentUserDto,
    old_material: &MaterialRow,
    new_material: &MaterialRow,
) {
    use crate::audit::audit_metadata;
    use crate::domain::audit_action;
    use crate::audit::audit_service::{AuditService, AuditWriteInput};

    let result = (|| {
        let mut changed = Vec::new();
        audit_metadata::push_changed(
            &mut changed,
            "title",
            &old_material.title,
            &new_material.title,
        );
        audit_metadata::push_changed(
            &mut changed,
            "materialType",
            &old_material.material_type,
            &new_material.material_type,
        );
        audit_metadata::push_changed(
            &mut changed,
            "sourceName",
            &old_material.source_name,
            &new_material.source_name,
        );
        audit_metadata::push_changed(
            &mut changed,
            "description",
            &old_material.description,
            &new_material.description,
        );
        audit_metadata::push_changed(
            &mut changed,
            "capturedAt",
            &old_material.captured_at,
            &new_material.captured_at,
        );
        audit_metadata::push_changed(
            &mut changed,
            "includeInReport",
            &old_material.include_in_report,
            &new_material.include_in_report,
        );

        if changed.is_empty() {
            return Ok(());
        }

        let is_toggle = changed.len() == 1 && changed[0] == "includeInReport";
        let action = if is_toggle {
            audit_action::material::REPORT_INCLUDE_CHANGED
        } else {
            audit_action::material::UPDATED
        };

        let technical_details = if is_toggle {
            audit_metadata::material_report_include_changed(
                &new_material.id,
                &new_material.material_code,
                new_material.include_in_report == 1,
            )?
        } else {
            audit_metadata::material_updated(
                &new_material.id,
                &new_material.material_code,
                &changed,
            )?
        };

        let old_val = audit_metadata::safe_material_snapshot(
            &old_material.material_code,
            old_material.original_file_name.as_deref().unwrap_or(""),
            &old_material.material_type,
            old_material.file_size,
            old_material.sha256.as_deref(),
            Some(old_material.integrity_status.as_str()),
            Some(old_material.description.as_str()),
            old_material.captured_at.as_deref(),
            old_material.include_in_report == 1,
        )?;

        let new_val = audit_metadata::safe_material_snapshot(
            &new_material.material_code,
            new_material.original_file_name.as_deref().unwrap_or(""),
            &new_material.material_type,
            new_material.file_size,
            new_material.sha256.as_deref(),
            Some(new_material.integrity_status.as_str()),
            Some(new_material.description.as_str()),
            new_material.captured_at.as_deref(),
            new_material.include_in_report == 1,
        )?;

        AuditService::write_best_effort(
            app,
            AuditWriteInput::success(current_user, action)
                .with_case_id(new_material.case_id.clone())
                .with_entity("material", new_material.id.clone())
                .with_snapshots(Some(old_val), Some(new_val))
                .with_details(technical_details),
        );
        Ok::<(), crate::errors::app_error::AppErrorDto>(())
    })();

    if let Err(err) = result {
        eprintln!(
            "[audit] write_material_updated_audit failed: {}",
            err.message
        );
    }
}

fn write_material_deleted_audit_best_effort(
    app: &AppHandle,
    current_user: &crate::security::session::CurrentUserDto,
    old_material: &MaterialRow,
) {
    use crate::audit::audit_metadata;
    use crate::domain::audit_action;
    use crate::audit::audit_service::{AuditService, AuditWriteInput};

    let result = (|| {
        let technical_details =
            audit_metadata::material_deleted(&old_material.id, &old_material.material_code)?;

        let old_value = audit_metadata::safe_material_snapshot(
            &old_material.material_code,
            old_material.original_file_name.as_deref().unwrap_or(""),
            &old_material.material_type,
            old_material.file_size,
            old_material.sha256.as_deref(),
            Some(old_material.integrity_status.as_str()),
            Some(old_material.description.as_str()),
            old_material.captured_at.as_deref(),
            old_material.include_in_report == 1,
        )?;

        AuditService::write_best_effort(
            app,
            AuditWriteInput::success(current_user, audit_action::material::DELETED)
                .with_case_id(old_material.case_id.clone())
                .with_entity("material", old_material.id.clone())
                .with_snapshots(Some(old_value), None)
                .with_details(technical_details),
        );
        Ok::<(), crate::errors::app_error::AppErrorDto>(())
    })();

    if let Err(err) = result {
        eprintln!(
            "[audit] write_material_deleted_audit failed: {}",
            err.message
        );
    }
}

#[allow(dead_code)]
fn write_material_hash_verified_audit_best_effort(
    app: &AppHandle,
    current_user: &crate::security::session::CurrentUserDto,
    old_material: &MaterialRow,
    new_material: &MaterialRow,
) {
    use crate::audit::audit_metadata;
    use crate::domain::audit_action;
    use crate::audit::audit_service::{AuditService, AuditWriteInput};

    let result = (|| {
        let integrity_status = new_material.integrity_status.as_str();
        let action = if integrity_status == "ok" {
            audit_action::material::HASH_VERIFIED
        } else {
            audit_action::material::HASH_MISMATCH
        };

        let technical_details = audit_metadata::material_hash_verified(
            &new_material.id,
            &new_material.material_code,
            integrity_status,
        )?;

        let old_val = audit_metadata::safe_material_snapshot(
            &old_material.material_code,
            old_material.original_file_name.as_deref().unwrap_or(""),
            &old_material.material_type,
            old_material.file_size,
            old_material.sha256.as_deref(),
            Some(old_material.integrity_status.as_str()),
            Some(old_material.description.as_str()),
            old_material.captured_at.as_deref(),
            old_material.include_in_report == 1,
        )?;

        let new_val = audit_metadata::safe_material_snapshot(
            &new_material.material_code,
            new_material.original_file_name.as_deref().unwrap_or(""),
            &new_material.material_type,
            new_material.file_size,
            new_material.sha256.as_deref(),
            Some(new_material.integrity_status.as_str()),
            Some(new_material.description.as_str()),
            new_material.captured_at.as_deref(),
            new_material.include_in_report == 1,
        )?;

        AuditService::write_best_effort(
            app,
            AuditWriteInput::success(current_user, action)
                .with_case_id(new_material.case_id.clone())
                .with_entity("material", new_material.id.clone())
                .with_snapshots(Some(old_val), Some(new_val))
                .with_details(technical_details),
        );
        Ok::<(), crate::errors::app_error::AppErrorDto>(())
    })();

    if let Err(err) = result {
        eprintln!(
            "[audit] write_material_hash_verified_audit failed: {}",
            err.message
        );
    }
}
