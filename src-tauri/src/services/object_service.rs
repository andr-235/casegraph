use tauri::AppHandle;
use uuid::Uuid;

use crate::db::connection::open_connection;
use crate::domain::object_type::is_valid_object_type;
use crate::domain::objects::{
    CreateObjectPayload, CreateObjectResponse, GetObjectByIdPayload, GetObjectByIdResponse,
    GetObjectsPayload, GetObjectsResponse, UpdateObjectPayload, UpdateObjectResponse,
};
use crate::errors::app_error::AppErrorDto;
use crate::repositories::case_repository::CaseRepository;
use crate::repositories::object_repository::{
    CreateObjectRecord, ObjectRepository, UpdateObjectRecord,
};
use crate::security::session::SessionState;

pub struct ObjectService;

impl ObjectService {
    pub fn create_object(
        app: &AppHandle,
        session: &SessionState,
        payload: CreateObjectPayload,
    ) -> Result<CreateObjectResponse, AppErrorDto> {
        let current_user = session.get_current_user().ok_or_else(|| {
            AppErrorDto::new("ERR_UNAUTHORIZED", "Пользователь не авторизован.", None)
        })?;

        if current_user.role != "administrator" && current_user.role != "analyst" {
            return Err(AppErrorDto::new(
                "ERR_ACCESS_DENIED",
                "Недостаточно прав для создания объекта.",
                None,
            ));
        }

        let case_id = payload.case_id.trim().to_string();
        let object_type = payload.object_type.trim().to_string();
        let title = payload.title.trim().to_string();
        let value = payload
            .value
            .map(|v| v.trim().to_string())
            .filter(|v| !v.is_empty());
        let description = payload
            .description
            .map(|v| v.trim().to_string())
            .unwrap_or_default();
        let confidence_note = payload
            .confidence_note
            .map(|v| v.trim().to_string())
            .unwrap_or_default();

        if case_id.is_empty() {
            return Err(AppErrorDto::new(
                "ERR_CASE_REQUIRED",
                "Не выбрано дело.",
                None,
            ));
        }

        if !is_valid_object_type(&object_type) {
            return Err(AppErrorDto::new(
                "ERR_OBJECT_TYPE_INVALID",
                "Недопустимый тип объекта.",
                Some(object_type),
            ));
        }

        if title.chars().count() < 2 {
            return Err(AppErrorDto::new(
                "ERR_OBJECT_TITLE_TOO_SHORT",
                "Название объекта должно содержать минимум 2 символа.",
                None,
            ));
        }

        let conn = open_connection(app)?;

        let case_item = CaseRepository::get_case_by_id(&conn, &case_id)?;

        if case_item.is_none() {
            return Err(AppErrorDto::new(
                "ERR_CASE_NOT_FOUND",
                "Дело не найдено.",
                None,
            ));
        }

        let object_id = Uuid::new_v4().to_string();
        let object_code =
            ObjectRepository::generate_next_object_code(&conn, &case_id, &object_type)?;

        ObjectRepository::create(
            &conn,
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
                created_by_user_id: current_user.user_id,
            },
        )?;

        let items = ObjectRepository::list_by_case(&conn, &case_id)?;
        let object_item = items
            .into_iter()
            .find(|item| item.id == object_id)
            .ok_or_else(|| {
                AppErrorDto::new("ERR_OBJECT_NOT_FOUND", "Созданный объект не найден.", None)
            })?;

        Ok(CreateObjectResponse { object_item })
    }

    pub fn get_objects(
        app: &AppHandle,
        session: &SessionState,
        payload: GetObjectsPayload,
    ) -> Result<GetObjectsResponse, AppErrorDto> {
        let current_user = session.get_current_user().ok_or_else(|| {
            AppErrorDto::new("ERR_UNAUTHORIZED", "Пользователь не авторизован.", None)
        })?;

        if current_user.role != "administrator"
            && current_user.role != "analyst"
            && current_user.role != "viewer"
        {
            return Err(AppErrorDto::new(
                "ERR_ACCESS_DENIED",
                "Недостаточно прав для просмотра объектов.",
                None,
            ));
        }

        let case_id = payload.case_id.trim().to_string();

        if case_id.is_empty() {
            return Err(AppErrorDto::new(
                "ERR_CASE_REQUIRED",
                "Не выбрано дело.",
                None,
            ));
        }

        let conn = open_connection(app)?;
        let items = ObjectRepository::list_by_case(&conn, &case_id)?;

        Ok(GetObjectsResponse { items })
    }

    pub fn get_object_by_id(
        app: &AppHandle,
        session: &SessionState,
        payload: GetObjectByIdPayload,
    ) -> Result<GetObjectByIdResponse, AppErrorDto> {
        let current_user = session.get_current_user().ok_or_else(|| {
            AppErrorDto::new("ERR_UNAUTHORIZED", "Пользователь не авторизован.", None)
        })?;

        if current_user.role != "administrator"
            && current_user.role != "analyst"
            && current_user.role != "viewer"
        {
            return Err(AppErrorDto::new(
                "ERR_ACCESS_DENIED",
                "Недостаточно прав для просмотра объекта.",
                None,
            ));
        }

        let case_id = payload.case_id.trim().to_string();
        let object_id = payload.object_id.trim().to_string();

        if case_id.is_empty() {
            return Err(AppErrorDto::new(
                "ERR_CASE_REQUIRED",
                "Не выбрано дело.",
                None,
            ));
        }

        if object_id.is_empty() {
            return Err(AppErrorDto::new(
                "ERR_OBJECT_REQUIRED",
                "Не выбран объект.",
                None,
            ));
        }

        let conn = open_connection(app)?;

        let object_item = ObjectRepository::find_by_id(&conn, &case_id, &object_id)?
            .ok_or_else(|| AppErrorDto::new("ERR_OBJECT_NOT_FOUND", "Объект не найден.", None))?;

        Ok(GetObjectByIdResponse { object_item })
    }

    pub fn update_object(
        app: &AppHandle,
        session: &SessionState,
        payload: UpdateObjectPayload,
    ) -> Result<UpdateObjectResponse, AppErrorDto> {
        let current_user = session.get_current_user().ok_or_else(|| {
            AppErrorDto::new("ERR_UNAUTHORIZED", "Пользователь не авторизован.", None)
        })?;

        if current_user.role != "administrator" && current_user.role != "analyst" {
            return Err(AppErrorDto::new(
                "ERR_ACCESS_DENIED",
                "Недостаточно прав для изменения объекта.",
                None,
            ));
        }

        let case_id = payload.case_id.trim().to_string();
        let object_id = payload.object_id.trim().to_string();
        let title = payload.title.trim().to_string();
        let value = payload
            .value
            .map(|v| v.trim().to_string())
            .filter(|v| !v.is_empty());
        let description = payload
            .description
            .map(|v| v.trim().to_string())
            .unwrap_or_default();
        let confidence_note = payload
            .confidence_note
            .map(|v| v.trim().to_string())
            .unwrap_or_default();

        if case_id.is_empty() {
            return Err(AppErrorDto::new(
                "ERR_CASE_REQUIRED",
                "Не выбрано дело.",
                None,
            ));
        }

        if object_id.is_empty() {
            return Err(AppErrorDto::new(
                "ERR_OBJECT_REQUIRED",
                "Не выбран объект.",
                None,
            ));
        }

        if title.chars().count() < 2 {
            return Err(AppErrorDto::new(
                "ERR_OBJECT_TITLE_TOO_SHORT",
                "Название объекта должно содержать минимум 2 символа.",
                None,
            ));
        }

        let conn = open_connection(app)?;

        ObjectRepository::update_object(
            &conn,
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
            ObjectRepository::find_by_id(&conn, &case_id, &object_id)?.ok_or_else(|| {
                AppErrorDto::new(
                    "ERR_OBJECT_NOT_FOUND",
                    "Объект не найден после обновления.",
                    None,
                )
            })?;

        Ok(UpdateObjectResponse { object_item })
    }
}
