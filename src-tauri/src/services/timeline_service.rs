use serde_json::json;
use tauri::AppHandle;
use uuid::Uuid;

use crate::audit::audit_metadata;
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
use crate::services::audit_service::{
    AuditService, AuditSuccessInput, ENTITY_TYPE_EVENT, EVENT_CREATED, EVENT_DELETED,
    EVENT_REPORT_FLAG_CHANGED, EVENT_UPDATED,
};
use crate::services::protected_service_context::{
    require_protected_analyst_or_admin_for, require_protected_user_for,
};

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
        let context = require_protected_user_for(app, session, "GET_TIMELINE")?;
        let conn = &context.conn;

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

        let items = TimelineRepository::get_timeline(conn, &case_id, &filters_record)?;

        Ok(GetTimelineResponse { items })
    }

    pub fn create_event(
        app: &AppHandle,
        session: &SessionState,
        payload: CreateEventPayload,
    ) -> Result<CreateEventResponse, AppErrorDto> {
        let mut context = require_protected_analyst_or_admin_for(app, session, "CREATE_EVENT")?;
        let current_user = context.current_user.clone();

        let normalized = normalize_create_event_payload(payload)?;

        let tx = context
            .conn
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

        let technical_details = audit_metadata::timeline_event_created(
            &event_item.id,
            &event_item.event_code,
            &normalized.case_id,
        );

        AuditService::write_success_non_blocking(
            app.clone(),
            AuditSuccessInput::new(
                &current_user,
                EVENT_CREATED,
                ENTITY_TYPE_EVENT,
                Some(&event_item.id),
                Some(&normalized.case_id),
                None,
                audit_metadata::to_value(&event_item),
                Some(technical_details),
            ),
        );

        Ok(CreateEventResponse { event_item })
    }

    pub fn get_event_by_id(
        app: &AppHandle,
        session: &SessionState,
        payload: GetEventByIdPayload,
    ) -> Result<GetEventByIdResponse, AppErrorDto> {
        let context = require_protected_user_for(app, session, "GET_EVENT_BY_ID")?;
        let conn = &context.conn;

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

        let event_details = TimelineRepository::get_event_by_id(conn, &case_id, &event_id)?
            .ok_or_else(|| AppErrorDto::new("ERR_EVENT_NOT_FOUND", "Событие не найдено", None))?;

        Ok(GetEventByIdResponse { event_details })
    }

    pub fn update_event(
        app: &AppHandle,
        session: &SessionState,
        payload: UpdateEventPayload,
    ) -> Result<UpdateEventResponse, AppErrorDto> {
        let mut context = require_protected_analyst_or_admin_for(app, session, "UPDATE_EVENT")?;
        let current_user = context.current_user.clone();

        let normalized = normalize_update_event_payload(payload)?;

        let tx = context
            .conn
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

        let old_event = TimelineRepository::get_event_list_item_by_id(
            &tx,
            &normalized.case_id,
            &normalized.event_id,
        )?;

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

        let mut changed = Vec::new();
        if old_event.title != event_details.event_item.title {
            changed.push("title");
        }
        if old_event.event_type != event_details.event_item.event_type {
            changed.push("eventType");
        }
        if old_event.event_date != event_details.event_item.event_date {
            changed.push("eventDate");
        }
        if old_event.include_in_report != event_details.event_item.include_in_report {
            changed.push("includeInReport");
        }

        let technical_details = audit_metadata::timeline_event_updated(
            &normalized.event_id,
            &event_details.event_item.event_code,
            &changed,
        );

        let (old_val, new_val) = audit_metadata::old_new(&old_event, &event_details.event_item);

        AuditService::write_success_non_blocking(
            app.clone(),
            AuditSuccessInput::new(
                &current_user,
                EVENT_UPDATED,
                ENTITY_TYPE_EVENT,
                Some(&normalized.event_id),
                Some(&normalized.case_id),
                old_val,
                new_val,
                Some(technical_details),
            ),
        );

        Ok(UpdateEventResponse { event_details })
    }

    pub fn soft_delete_event(
        app: &AppHandle,
        session: &SessionState,
        payload: SoftDeleteEventPayload,
    ) -> Result<SoftDeleteEventResponse, AppErrorDto> {
        let context = require_protected_analyst_or_admin_for(app, session, "SOFT_DELETE_EVENT")?;
        let current_user = &context.current_user;
        let conn = &context.conn;

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

        let old_event_item =
            TimelineRepository::get_event_list_item_by_id(conn, &case_id, &event_id)?;

        TimelineRepository::soft_delete_event(conn, &case_id, &event_id)?;

        let technical_details =
            audit_metadata::timeline_event_deleted(&event_id, &old_event_item.event_code);

        AuditService::write_success_non_blocking(
            app.clone(),
            AuditSuccessInput::new(
                &current_user,
                EVENT_DELETED,
                ENTITY_TYPE_EVENT,
                Some(&event_id),
                Some(&case_id),
                audit_metadata::to_value(&old_event_item),
                Some(json!({ "archived": true })),
                Some(technical_details),
            ),
        );

        Ok(SoftDeleteEventResponse { event_id })
    }

    pub fn toggle_event_report_include(
        app: &AppHandle,
        session: &SessionState,
        payload: ToggleEventReportIncludePayload,
    ) -> Result<ToggleEventReportIncludeResponse, AppErrorDto> {
        let context =
            require_protected_analyst_or_admin_for(app, session, "TOGGLE_EVENT_REPORT_INCLUDE")?;
        let current_user = &context.current_user;
        let conn = &context.conn;

        let normalized = normalize_toggle_event_report_include_payload(payload)?;

        let old_event_item = TimelineRepository::get_event_list_item_by_id(
            conn,
            &normalized.case_id,
            &normalized.event_id,
        )?;

        let event_item = TimelineRepository::set_event_report_include(
            conn,
            &normalized.case_id,
            &normalized.event_id,
            normalized.include_in_report,
        )?;

        let technical_details = audit_metadata::timeline_event_report_include_changed(
            &event_item.id,
            &event_item.event_code,
            event_item.include_in_report,
        );

        AuditService::write_success_non_blocking(
            app.clone(),
            AuditSuccessInput::new(
                &current_user,
                EVENT_REPORT_FLAG_CHANGED,
                ENTITY_TYPE_EVENT,
                Some(&event_item.id),
                Some(&normalized.case_id),
                Some(json!({ "includeInReport": old_event_item.include_in_report })),
                Some(json!({ "includeInReport": event_item.include_in_report })),
                Some(technical_details),
            ),
        );

        Ok(ToggleEventReportIncludeResponse { event_item })
    }
}
