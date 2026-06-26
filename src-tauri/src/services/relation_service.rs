use tauri::AppHandle;
use uuid::Uuid;

use crate::db::connection::open_connection;
use crate::domain::relation_confidence::is_valid_confidence_level;
use crate::domain::relation_type::is_valid_relation_type;
use crate::domain::relations::{
    CreateRelationPayload, CreateRelationResponse, GetRelationsPayload, GetRelationsResponse,
};
use crate::errors::app_error::AppErrorDto;
use crate::repositories::case_repository::CaseRepository;
use crate::repositories::relation_repository::{CreateRelationRecord, RelationRepository};
use crate::security::session::SessionState;

const MAX_RELATION_TITLE_LEN: usize = 200;
const MAX_RELATION_BASIS_LEN: usize = 10_000;
const MAX_ANALYST_COMMENT_LEN: usize = 5_000;

pub struct RelationService;

impl RelationService {
    pub fn create_relation(
        app: &AppHandle,
        session: &SessionState,
        payload: CreateRelationPayload,
    ) -> Result<CreateRelationResponse, AppErrorDto> {
        let current_user = session.get_current_user().ok_or_else(|| {
            AppErrorDto::new("ERR_UNAUTHORIZED", "Пользователь не авторизован.", None)
        })?;

        if current_user.role != "administrator" && current_user.role != "analyst" {
            return Err(AppErrorDto::new(
                "ERR_ACCESS_DENIED",
                "Недостаточно прав для создания связи.",
                None,
            ));
        }

        let case_id =
            normalize_required_id(&payload.case_id, "ERR_CASE_REQUIRED", "Не выбрано дело.")?;
        let source_object_id = normalize_required_id(
            &payload.source_object_id,
            "ERR_RELATION_OBJECT_REQUIRED",
            "Укажите первый объект связи.",
        )?;
        let target_object_id = normalize_required_id(
            &payload.target_object_id,
            "ERR_RELATION_OBJECT_REQUIRED",
            "Укажите второй объект связи.",
        )?;

        if source_object_id == target_object_id {
            return Err(AppErrorDto::new(
                "ERR_RELATION_SAME_OBJECT",
                "Нельзя создать связь объекта с самим собой.",
                None,
            ));
        }

        let relation_type = normalize_relation_type(&payload.relation_type)?;
        let confidence_level = normalize_confidence_level(&payload.confidence_level)?;
        let title = normalize_optional_text(
            payload.title,
            MAX_RELATION_TITLE_LEN,
            "ERR_RELATION_TITLE_TOO_LONG",
            "Название связи слишком длинное.",
        )?;
        let basis = normalize_basis(&payload.basis)?;
        let analyst_comment = normalize_optional_text(
            payload.analyst_comment,
            MAX_ANALYST_COMMENT_LEN,
            "ERR_RELATION_ANALYST_COMMENT_TOO_LONG",
            "Комментарий аналитика слишком длинный.",
        )?;
        let supporting_material_id = payload
            .supporting_material_id
            .map(|value| value.trim().to_string())
            .filter(|value| !value.is_empty());

        let conn = open_connection(app)?;

        CaseRepository::get_case_by_id(&conn, &case_id)?
            .ok_or_else(|| AppErrorDto::new("ERR_CASE_NOT_FOUND", "Дело не найдено.", None))?;

        let source_object = RelationRepository::find_object_case_info(&conn, &source_object_id)?
            .ok_or_else(|| {
                AppErrorDto::new("ERR_OBJECT_NOT_FOUND", "Первый объект не найден.", None)
            })?;

        let target_object = RelationRepository::find_object_case_info(&conn, &target_object_id)?
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
            let material = RelationRepository::find_material_case_info(&conn, material_id)?
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
        let relation_code = RelationRepository::generate_next_relation_code(&conn, &case_id)?;

        RelationRepository::create(
            &conn,
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
                created_by_user_id: current_user.user_id,
            },
        )?;

        let relation_item = RelationRepository::get_by_id(&conn, &relation_id)?
            .ok_or_else(|| AppErrorDto::new("ERR_RELATION_NOT_FOUND", "Связь не найдена.", None))?;

        Ok(CreateRelationResponse { relation_item })
    }

    pub fn get_relations(
        app: &AppHandle,
        session: &SessionState,
        payload: GetRelationsPayload,
    ) -> Result<GetRelationsResponse, AppErrorDto> {
        session.get_current_user().ok_or_else(|| {
            AppErrorDto::new("ERR_UNAUTHORIZED", "Пользователь не авторизован.", None)
        })?;

        let case_id =
            normalize_required_id(&payload.case_id, "ERR_CASE_REQUIRED", "Не выбрано дело.")?;
        let conn = open_connection(app)?;

        CaseRepository::get_case_by_id(&conn, &case_id)?
            .ok_or_else(|| AppErrorDto::new("ERR_CASE_NOT_FOUND", "Дело не найдено.", None))?;

        let items = RelationRepository::list_by_case(&conn, &case_id)?;

        Ok(GetRelationsResponse { items })
    }
}

fn normalize_required_id(value: &str, code: &str, message: &str) -> Result<String, AppErrorDto> {
    let normalized = value.trim().to_string();

    if normalized.is_empty() {
        return Err(AppErrorDto::new(code, message, None));
    }

    Ok(normalized)
}

fn normalize_relation_type(value: &str) -> Result<String, AppErrorDto> {
    let normalized = value.trim().to_lowercase();

    if normalized.is_empty() {
        return Err(AppErrorDto::new(
            "ERR_RELATION_TYPE_REQUIRED",
            "Укажите тип связи.",
            None,
        ));
    }

    if !is_valid_relation_type(&normalized) {
        return Err(AppErrorDto::new(
            "ERR_RELATION_TYPE_INVALID",
            "Некорректный тип связи.",
            None,
        ));
    }

    Ok(normalized)
}

fn normalize_confidence_level(value: &str) -> Result<String, AppErrorDto> {
    let normalized = value.trim().to_lowercase();

    if normalized.is_empty() {
        return Err(AppErrorDto::new(
            "ERR_CONFIDENCE_LEVEL_REQUIRED",
            "Укажите уровень достоверности.",
            None,
        ));
    }

    if !is_valid_confidence_level(&normalized) {
        return Err(AppErrorDto::new(
            "ERR_CONFIDENCE_LEVEL_INVALID",
            "Некорректный уровень достоверности.",
            None,
        ));
    }

    Ok(normalized)
}

fn normalize_basis(value: &str) -> Result<String, AppErrorDto> {
    let normalized = value.trim().to_string();

    if normalized.is_empty() {
        return Err(AppErrorDto::new(
            "ERR_RELATION_BASIS_REQUIRED",
            "Укажите основание связи.",
            None,
        ));
    }

    if normalized.chars().count() < 3 || normalized.chars().count() > MAX_RELATION_BASIS_LEN {
        return Err(AppErrorDto::new(
            "ERR_RELATION_BASIS_INVALID",
            "Основание связи должно содержать от 3 до 10000 символов.",
            None,
        ));
    }

    Ok(normalized)
}

fn normalize_optional_text(
    value: Option<String>,
    max_len: usize,
    code: &str,
    message: &str,
) -> Result<Option<String>, AppErrorDto> {
    let Some(value) = value else {
        return Ok(None);
    };

    let normalized = value.trim().to_string();

    if normalized.is_empty() {
        return Ok(None);
    }

    if normalized.chars().count() > max_len {
        return Err(AppErrorDto::new(code, message, None));
    }

    Ok(Some(normalized))
}
