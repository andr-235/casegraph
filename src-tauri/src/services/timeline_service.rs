use tauri::AppHandle;
use uuid::Uuid;

use crate::db::connection::open_connection;
use crate::domain::timeline::{
    CreateEventPayload, CreateEventResponse, GetEventByIdPayload, GetEventByIdResponse,
    GetTimelinePayload, GetTimelineResponse, SoftDeleteEventPayload, SoftDeleteEventResponse,
    ToggleEventReportIncludePayload, ToggleEventReportIncludeResponse, UpdateEventPayload,
    UpdateEventResponse,
};
use crate::errors::app_error::AppErrorDto;
use crate::repositories::timeline_repository::{
    CreateEventRecord, TimelineFiltersRecord, TimelineRepository, UpdateEventRecord,
};
use crate::security::session::SessionState;

use super::timeline_validation::{
    normalize_create_event_payload, normalize_required_id, normalize_timeline_filters,
    normalize_toggle_event_report_include_payload, normalize_update_event_payload,
};

pub struct TimelineService;

impl TimelineService {
    pub fn get_timeline(
        app: &AppHandle,
        session: &SessionState,
        payload: GetTimelinePayload,
    ) -> Result<GetTimelineResponse, AppErrorDto> {
        let _current_user = session.get_current_user().ok_or_else(|| {
            AppErrorDto::new("ERR_UNAUTHORIZED", "Требуется вход в систему.", None)
        })?;

        let case_id = normalize_required_id(
            &payload.case_id,
            "ERR_INVALID_CASE_ID",
            "ID дела обязателен",
        )?;

        let filters = normalize_timeline_filters(&payload)?;

        let filters_record = TimelineFiltersRecord {
            query: filters.query,
            event_type: filters.event_type,
            object_id: filters.object_id,
            material_id: filters.material_id,
            date_from: filters.date_from,
            date_to: filters.date_to,
            include_in_report: filters.include_in_report,
        };

        let conn = open_connection(app)?;
        let items = TimelineRepository::get_timeline(&conn, &case_id, &filters_record)?;

        Ok(GetTimelineResponse { items })
    }

    pub fn create_event(
        app: &AppHandle,
        session: &SessionState,
        payload: CreateEventPayload,
    ) -> Result<CreateEventResponse, AppErrorDto> {
        let current_user = session.get_current_user().ok_or_else(|| {
            AppErrorDto::new("ERR_UNAUTHORIZED", "Требуется вход в систему.", None)
        })?;

        if current_user.role == "viewer" {
            return Err(AppErrorDto::new(
                "ERR_ACCESS_DENIED",
                "Недостаточно прав для создания события",
                None,
            ));
        }

        let normalized = normalize_create_event_payload(payload)?;

        let mut conn = open_connection(app)?;
        let tx = conn
            .transaction()
            .map_err(|err| AppErrorDto::database(err.to_string()))?;

        for object_id in &normalized.object_ids {
            let belongs =
                TimelineRepository::object_belongs_to_case(&tx, &normalized.case_id, object_id)?;

            if !belongs {
                return Err(AppErrorDto::new(
                    "ERR_EVENT_OBJECT_NOT_IN_CASE",
                    "Один из объектов не принадлежит выбранному делу",
                    None,
                ));
            }
        }

        for material_id in &normalized.material_ids {
            let belongs = TimelineRepository::material_belongs_to_case(
                &tx,
                &normalized.case_id,
                material_id,
            )?;

            if !belongs {
                return Err(AppErrorDto::new(
                    "ERR_EVENT_MATERIAL_NOT_IN_CASE",
                    "Один из материалов не принадлежит выбранному делу",
                    None,
                ));
            }
        }

        let event_id = Uuid::new_v4().to_string();
        let event_code = TimelineRepository::get_next_event_code(&tx, &normalized.case_id)?;

        let record = CreateEventRecord {
            id: event_id.clone(),
            case_id: normalized.case_id.clone(),
            event_code,
            event_type: normalized.event_type,
            title: normalized.title,
            description: normalized.description,
            event_date: normalized.event_date,
            event_time: normalized.event_time,
            date_precision: normalized.date_precision,
            period_start: normalized.period_start,
            period_end: normalized.period_end,
            source_note: normalized.source_note,
            analyst_comment: normalized.analyst_comment,
            include_in_report: normalized.include_in_report,
            created_by_user_id: current_user.user_id.clone(),
        };

        TimelineRepository::create_event(&tx, &record)?;

        TimelineRepository::link_event_to_objects(
            &tx,
            &normalized.case_id,
            &event_id,
            &normalized.object_ids,
            &normalized.link_note,
            &current_user.user_id,
        )?;

        TimelineRepository::link_event_to_materials(
            &tx,
            &normalized.case_id,
            &event_id,
            &normalized.material_ids,
            &normalized.link_note,
            &current_user.user_id,
        )?;

        tx.commit()
            .map_err(|err| AppErrorDto::database(err.to_string()))?;

        let conn = open_connection(app)?;
        let items = TimelineRepository::get_timeline(
            &conn,
            &normalized.case_id,
            &TimelineFiltersRecord {
                query: None,
                event_type: None,
                object_id: None,
                material_id: None,
                date_from: None,
                date_to: None,
                include_in_report: None,
            },
        )?;
        let event_item = items
            .into_iter()
            .find(|item| item.id == event_id)
            .ok_or_else(|| {
                AppErrorDto::new(
                    "ERR_EVENT_NOT_FOUND_AFTER_CREATE",
                    "Событие создано, но не найдено после сохранения",
                    None,
                )
            })?;

        Ok(CreateEventResponse { event_item })
    }

    pub fn get_event_by_id(
        app: &AppHandle,
        session: &SessionState,
        payload: GetEventByIdPayload,
    ) -> Result<GetEventByIdResponse, AppErrorDto> {
        let _current_user = session.get_current_user().ok_or_else(|| {
            AppErrorDto::new("ERR_UNAUTHORIZED", "Требуется вход в систему.", None)
        })?;

        let case_id = normalize_required_id(
            &payload.case_id,
            "ERR_INVALID_CASE_ID",
            "ID дела обязателен",
        )?;

        let event_id = normalize_required_id(
            &payload.event_id,
            "ERR_INVALID_EVENT_ID",
            "ID события обязателен",
        )?;

        let conn = open_connection(app)?;
        let event_details = TimelineRepository::get_event_by_id(&conn, &case_id, &event_id)?
            .ok_or_else(|| AppErrorDto::new("ERR_EVENT_NOT_FOUND", "Событие не найдено", None))?;

        Ok(GetEventByIdResponse { event_details })
    }

    pub fn update_event(
        app: &AppHandle,
        session: &SessionState,
        payload: UpdateEventPayload,
    ) -> Result<UpdateEventResponse, AppErrorDto> {
        let current_user = session.get_current_user().ok_or_else(|| {
            AppErrorDto::new("ERR_UNAUTHORIZED", "Требуется вход в систему.", None)
        })?;

        if current_user.role == "viewer" {
            return Err(AppErrorDto::new(
                "ERR_ACCESS_DENIED",
                "Недостаточно прав для изменения события",
                None,
            ));
        }

        let normalized = normalize_update_event_payload(payload)?;

        let mut conn = open_connection(app)?;
        let tx = conn
            .transaction()
            .map_err(|err| AppErrorDto::database(err.to_string()))?;

        let exists = TimelineRepository::event_belongs_to_case(
            &tx,
            &normalized.case_id,
            &normalized.event_id,
        )?;

        if !exists {
            return Err(AppErrorDto::new(
                "ERR_EVENT_NOT_FOUND",
                "Событие не найдено",
                None,
            ));
        }

        for object_id in &normalized.object_ids {
            let belongs =
                TimelineRepository::object_belongs_to_case(&tx, &normalized.case_id, object_id)?;

            if !belongs {
                return Err(AppErrorDto::new(
                    "ERR_EVENT_OBJECT_CASE_MISMATCH",
                    "Один из объектов не принадлежит выбранному делу",
                    None,
                ));
            }
        }

        for material_id in &normalized.material_ids {
            let belongs = TimelineRepository::material_belongs_to_case(
                &tx,
                &normalized.case_id,
                material_id,
            )?;

            if !belongs {
                return Err(AppErrorDto::new(
                    "ERR_EVENT_MATERIAL_CASE_MISMATCH",
                    "Один из материалов не принадлежит выбранному делу",
                    None,
                ));
            }
        }

        let record = UpdateEventRecord {
            id: normalized.event_id.clone(),
            case_id: normalized.case_id.clone(),
            event_type: normalized.event_type,
            title: normalized.title,
            description: normalized.description,
            event_date: normalized.event_date,
            event_time: normalized.event_time,
            date_precision: normalized.date_precision,
            period_start: normalized.period_start,
            period_end: normalized.period_end,
            source_note: normalized.source_note,
            analyst_comment: normalized.analyst_comment,
            include_in_report: normalized.include_in_report,
        };

        TimelineRepository::update_event(&tx, &record)?;

        TimelineRepository::replace_event_object_links(
            &tx,
            &normalized.case_id,
            &normalized.event_id,
            &normalized.object_ids,
            &normalized.link_note,
            &current_user.user_id,
        )?;

        TimelineRepository::replace_event_material_links(
            &tx,
            &normalized.case_id,
            &normalized.event_id,
            &normalized.material_ids,
            &normalized.link_note,
            &current_user.user_id,
        )?;

        tx.commit()
            .map_err(|err| AppErrorDto::database(err.to_string()))?;

        let conn = open_connection(app)?;
        let event_details =
            TimelineRepository::get_event_by_id(&conn, &normalized.case_id, &normalized.event_id)?
                .ok_or_else(|| {
                    AppErrorDto::new(
                        "ERR_EVENT_NOT_FOUND_AFTER_UPDATE",
                        "Событие обновлено, но не найдено после сохранения",
                        None,
                    )
                })?;

        Ok(UpdateEventResponse { event_details })
    }

    pub fn soft_delete_event(
        app: &AppHandle,
        session: &SessionState,
        payload: SoftDeleteEventPayload,
    ) -> Result<SoftDeleteEventResponse, AppErrorDto> {
        let current_user = session.get_current_user().ok_or_else(|| {
            AppErrorDto::new("ERR_UNAUTHORIZED", "Требуется вход в систему.", None)
        })?;

        if current_user.role == "viewer" {
            return Err(AppErrorDto::new(
                "ERR_ACCESS_DENIED",
                "Недостаточно прав для удаления события",
                None,
            ));
        }

        let case_id = normalize_required_id(
            &payload.case_id,
            "ERR_INVALID_CASE_ID",
            "ID дела обязателен",
        )?;

        let event_id = normalize_required_id(
            &payload.event_id,
            "ERR_INVALID_EVENT_ID",
            "ID события обязателен",
        )?;

        let conn = open_connection(app)?;

        TimelineRepository::soft_delete_event(&conn, &case_id, &event_id)?;

        Ok(SoftDeleteEventResponse { event_id })
    }

    pub fn toggle_event_report_include(
        app: &AppHandle,
        session: &SessionState,
        payload: ToggleEventReportIncludePayload,
    ) -> Result<ToggleEventReportIncludeResponse, AppErrorDto> {
        let current_user = session.get_current_user().ok_or_else(|| {
            AppErrorDto::new("ERR_UNAUTHORIZED", "Пользователь не авторизован", None)
        })?;

        if current_user.role != "administrator" && current_user.role != "analyst" {
            return Err(AppErrorDto::new(
                "ERR_ACCESS_DENIED",
                "Недостаточно прав для изменения признака включения события в справку",
                None,
            ));
        }

        let normalized = normalize_toggle_event_report_include_payload(payload)?;

        let conn = open_connection(app)?;

        let event_item = TimelineRepository::set_event_report_include(
            &conn,
            &normalized.case_id,
            &normalized.event_id,
            normalized.include_in_report,
        )?;

        // TODO: подключить AuditService:
        // EVENT_REPORT_INCLUDE_CHANGED

        Ok(ToggleEventReportIncludeResponse { event_item })
    }
}
