use tauri::AppHandle;
use uuid::Uuid;

use crate::db::connection::open_connection;
use crate::domain::timeline::{
    CreateEventPayload, CreateEventResponse, GetTimelinePayload, GetTimelineResponse,
};
use crate::errors::app_error::AppErrorDto;
use crate::repositories::timeline_repository::{CreateEventRecord, TimelineRepository};
use crate::security::session::SessionState;

use super::timeline_validation::{
    normalize_date_precision, normalize_event_analyst_comment, normalize_event_date,
    normalize_event_description, normalize_event_link_note, normalize_event_source_note,
    normalize_event_title, normalize_event_type, normalize_required_id, validate_period,
};

pub struct TimelineService;

impl TimelineService {
    pub fn get_timeline(
        app: &AppHandle,
        session: &SessionState,
        payload: GetTimelinePayload,
    ) -> Result<GetTimelineResponse, AppErrorDto> {
        let _current_user = session
            .get_current_user()
            .ok_or_else(|| AppErrorDto::new("ERR_UNAUTHORIZED", "Требуется вход в систему.", None))?;

        let case_id = normalize_required_id(
            &payload.case_id,
            "ERR_INVALID_CASE_ID",
            "ID дела обязателен",
        )?;

        let conn = open_connection(app)?;
        let items = TimelineRepository::get_timeline(&conn, &case_id)?;

        Ok(GetTimelineResponse { items })
    }

    pub fn create_event(
        app: &AppHandle,
        session: &SessionState,
        payload: CreateEventPayload,
    ) -> Result<CreateEventResponse, AppErrorDto> {
        let current_user = session
            .get_current_user()
            .ok_or_else(|| AppErrorDto::new("ERR_UNAUTHORIZED", "Требуется вход в систему.", None))?;

        if current_user.role == "viewer" {
            return Err(AppErrorDto::new(
                "ERR_ACCESS_DENIED",
                "Недостаточно прав для создания события",
                None,
            ));
        }

        let case_id = normalize_required_id(
            &payload.case_id,
            "ERR_INVALID_CASE_ID",
            "ID дела обязателен",
        )?;

        let event_type = normalize_event_type(&payload.event_type)?;
        let title = normalize_event_title(&payload.title)?;
        let description = normalize_event_description(&payload.description)?;
        let event_date = normalize_event_date(&payload.event_date)?;
        let date_precision = normalize_date_precision(&payload.date_precision)?;

        let period_start = payload.period_start.map(|value| value.trim().to_string());
        let period_end = payload.period_end.map(|value| value.trim().to_string());

        validate_period(&date_precision, &period_start, &period_end)?;

        let source_note = normalize_event_source_note(&payload.source_note)?;
        let analyst_comment = normalize_event_analyst_comment(&payload.analyst_comment)?;
        let link_note = normalize_event_link_note(&payload.link_note)?;

        let mut conn = open_connection(app)?;
        let tx = conn
            .transaction()
            .map_err(|err| AppErrorDto::database(err.to_string()))?;

        for object_id in &payload.object_ids {
            let object_id = object_id.trim();

            if object_id.is_empty() {
                continue;
            }

            let belongs = TimelineRepository::object_belongs_to_case(&tx, &case_id, object_id)?;

            if !belongs {
                return Err(AppErrorDto::new(
                    "ERR_EVENT_OBJECT_NOT_IN_CASE",
                    "Один из объектов не принадлежит выбранному делу",
                    None,
                ));
            }
        }

        for material_id in &payload.material_ids {
            let material_id = material_id.trim();

            if material_id.is_empty() {
                continue;
            }

            let belongs =
                TimelineRepository::material_belongs_to_case(&tx, &case_id, material_id)?;

            if !belongs {
                return Err(AppErrorDto::new(
                    "ERR_EVENT_MATERIAL_NOT_IN_CASE",
                    "Один из материалов не принадлежит выбранному делу",
                    None,
                ));
            }
        }

        let event_id = Uuid::new_v4().to_string();
        let event_code = TimelineRepository::get_next_event_code(&tx, &case_id)?;

        let record = CreateEventRecord {
            id: event_id.clone(),
            case_id: case_id.clone(),
            event_code,
            event_type,
            title,
            description,
            event_date,
            event_time: payload.event_time.map(|value| value.trim().to_string()),
            date_precision,
            period_start,
            period_end,
            source_note,
            analyst_comment,
            include_in_report: payload.include_in_report,
            created_by_user_id: current_user.user_id.clone(),
        };

        TimelineRepository::create_event(&tx, &record)?;

        TimelineRepository::link_event_to_objects(
            &tx,
            &case_id,
            &event_id,
            &payload.object_ids,
            &link_note,
            &current_user.user_id,
        )?;

        TimelineRepository::link_event_to_materials(
            &tx,
            &case_id,
            &event_id,
            &payload.material_ids,
            &link_note,
            &current_user.user_id,
        )?;

        tx.commit()
            .map_err(|err| AppErrorDto::database(err.to_string()))?;

        let conn = open_connection(app)?;
        let items = TimelineRepository::get_timeline(&conn, &case_id)?;
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
}
