use tauri::AppHandle;
use uuid::Uuid;

use crate::db::connection::open_connection;
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
use crate::security::session::{CurrentUserDto, SessionState};
use crate::storage::material_file_storage::import_material_file;

pub struct MaterialService;

impl MaterialService {
    pub fn get_materials(
        app: &AppHandle,
        session: &SessionState,
        payload: GetMaterialsPayload,
    ) -> Result<Vec<MaterialDto>, AppErrorDto> {
        require_current_user(session)?;

        let case_id = payload.case_id.trim().to_string();

        validate_case_id(&case_id)?;

        let conn = open_connection(app)?;

        ensure_case_exists(&conn, &case_id)?;

        let rows = MaterialRepository::get_materials_by_case_id(&conn, &case_id)?;

        Ok(rows.into_iter().map(material_row_to_dto).collect())
    }

    pub fn create_material(
        app: &AppHandle,
        session: &SessionState,
        payload: CreateMaterialPayload,
    ) -> Result<CreateMaterialResponse, AppErrorDto> {
        let current_user = require_current_user(session)?;

        let case_id = payload.case_id.trim().to_string();
        let title = payload.title.trim().to_string();
        let material_type = payload.material_type.trim().to_string();
        let source_name = payload.source_name.unwrap_or_default().trim().to_string();
        let description = payload.description.unwrap_or_default().trim().to_string();
        let captured_at = normalize_optional_string(payload.captured_at);

        validate_case_id(&case_id)?;
        validate_create_material_payload(&title, &material_type)?;

        let conn = open_connection(app)?;

        ensure_case_exists(&conn, &case_id)?;

        let material_code = MaterialRepository::get_next_material_code(&conn, &case_id)?;
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
            &conn,
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
                created_by_user_id: current_user.user_id,
            },
        )?;

        let created_material = MaterialRepository::get_material_by_id(&conn, &material_id)?
            .ok_or_else(|| {
                AppErrorDto::new(
                    "ERR_MATERIAL_NOT_FOUND_AFTER_CREATE",
                    "Материал создан, но не найден после сохранения.",
                    None,
                )
            })?;

        Ok(CreateMaterialResponse {
            material: material_row_to_dto(created_material),
        })
    }

    pub fn delete_material(
        app: &AppHandle,
        session: &SessionState,
        payload: DeleteMaterialPayload,
    ) -> Result<DeleteMaterialResponse, AppErrorDto> {
        require_current_user(session)?;

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

        let conn = open_connection(app)?;

        ensure_case_exists(&conn, &case_id)?;

        MaterialRepository::soft_delete_material(&conn, &case_id, &material_id)?;

        Ok(DeleteMaterialResponse { material_id })
    }

    pub fn update_material(
        app: &AppHandle,
        session: &SessionState,
        payload: UpdateMaterialPayload,
    ) -> Result<UpdateMaterialResponse, AppErrorDto> {
        require_current_user(session)?;

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

        let conn = open_connection(app)?;

        ensure_case_exists(&conn, &case_id)?;

        MaterialRepository::update_material(
            &conn,
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

        let updated_material = MaterialRepository::get_material_by_id(&conn, &material_id)?
            .ok_or_else(|| {
                AppErrorDto::new(
                    "ERR_MATERIAL_NOT_FOUND_AFTER_UPDATE",
                    "Материал обновлён, но не найден после сохранения.",
                    None,
                )
            })?;

        Ok(UpdateMaterialResponse {
            material: material_row_to_dto(updated_material),
        })
    }
}

fn require_current_user(session: &SessionState) -> Result<CurrentUserDto, AppErrorDto> {
    session
        .get_current_user()
        .ok_or_else(|| AppErrorDto::new("ERR_UNAUTHORIZED", "Требуется вход в систему.", None))
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
