use rusqlite::{params, params_from_iter, Connection, Row, ToSql};
use uuid::Uuid;

use crate::domain::timeline::{
    EventDetailsDto, EventLinkedMaterialDto, EventLinkedObjectDto, TimelineEventDto,
};
use crate::errors::app_error::AppErrorDto;

#[derive(Debug)]
pub struct CreateEventRecord {
    pub id: String,
    pub case_id: String,
    pub event_code: String,
    pub event_type: String,
    pub title: String,
    pub description: String,
    pub event_date: String,
    pub event_time: Option<String>,
    pub date_precision: String,
    pub period_start: Option<String>,
    pub period_end: Option<String>,
    pub source_note: String,
    pub analyst_comment: String,
    pub include_in_report: bool,
    pub created_by_user_id: String,
}

#[derive(Debug)]
pub struct UpdateEventRecord {
    pub id: String,
    pub case_id: String,
    pub event_type: String,
    pub title: String,
    pub description: String,
    pub event_date: String,
    pub event_time: Option<String>,
    pub date_precision: String,
    pub period_start: Option<String>,
    pub period_end: Option<String>,
    pub source_note: String,
    pub analyst_comment: String,
    pub include_in_report: bool,
}

#[derive(Debug)]
pub struct TimelineFiltersRecord {
    pub query: Option<String>,
    pub event_type: Option<String>,
    pub object_id: Option<String>,
    pub material_id: Option<String>,
    pub date_from: Option<String>,
    pub date_to: Option<String>,
    pub include_in_report: Option<bool>,
}

fn map_timeline_event_row(row: &Row<'_>) -> rusqlite::Result<TimelineEventDto> {
    let include_in_report: i64 = row.get(13)?;

    Ok(TimelineEventDto {
        id: row.get(0)?,
        case_id: row.get(1)?,
        event_code: row.get(2)?,
        event_type: row.get(3)?,
        title: row.get(4)?,
        description: row.get(5)?,
        event_date: row.get(6)?,
        event_time: row.get(7)?,
        date_precision: row.get(8)?,
        period_start: row.get(9)?,
        period_end: row.get(10)?,
        source_note: row.get(11)?,
        analyst_comment: row.get(12)?,
        include_in_report: include_in_report == 1,
        linked_object_count: row.get(14)?,
        linked_material_count: row.get(15)?,
        created_by_user_id: row.get(16)?,
        created_at: row.get(17)?,
        updated_at: row.get(18)?,
    })
}

pub struct TimelineRepository;

impl TimelineRepository {
    pub fn get_next_event_code(conn: &Connection, case_id: &str) -> Result<String, AppErrorDto> {
        let last_code: Option<String> = conn
            .query_row(
                r#"
                SELECT event_code
                FROM events
                WHERE case_id = ?1
                ORDER BY CAST(SUBSTR(event_code, 5) AS INTEGER) DESC
                LIMIT 1
                "#,
                params![case_id],
                |row| row.get(0),
            )
            .ok();

        let next_number = last_code
            .as_deref()
            .and_then(|code| code.strip_prefix("EVT-"))
            .and_then(|number| number.parse::<u32>().ok())
            .unwrap_or(0)
            + 1;

        Ok(format!("EVT-{next_number:03}"))
    }

    pub fn create_event(conn: &Connection, record: &CreateEventRecord) -> Result<(), AppErrorDto> {
        conn.execute(
            r#"
            INSERT INTO events (
                id,
                case_id,
                event_code,
                event_type,
                title,
                description,
                event_date,
                event_time,
                date_precision,
                period_start,
                period_end,
                source_note,
                analyst_comment,
                include_in_report,
                created_by_user_id
            )
            VALUES (?1, ?2, ?3, ?4, ?5, ?6, ?7, ?8, ?9, ?10, ?11, ?12, ?13, ?14, ?15)
            "#,
            params![
                record.id,
                record.case_id,
                record.event_code,
                record.event_type,
                record.title,
                record.description,
                record.event_date,
                record.event_time,
                record.date_precision,
                record.period_start,
                record.period_end,
                record.source_note,
                record.analyst_comment,
                record.include_in_report as i32,
                record.created_by_user_id
            ],
        )
        .map_err(|err| AppErrorDto::database(err.to_string()))?;

        Ok(())
    }

    pub fn link_event_to_objects(
        conn: &Connection,
        case_id: &str,
        event_id: &str,
        object_ids: &[String],
        link_note: &str,
        created_by_user_id: &str,
    ) -> Result<(), AppErrorDto> {
        for object_id in object_ids {
            conn.execute(
                r#"
                INSERT OR IGNORE INTO event_objects (
                    id,
                    case_id,
                    event_id,
                    object_id,
                    link_note,
                    created_by_user_id
                )
                VALUES (?1, ?2, ?3, ?4, ?5, ?6)
                "#,
                params![
                    Uuid::new_v4().to_string(),
                    case_id,
                    event_id,
                    object_id,
                    link_note,
                    created_by_user_id
                ],
            )
            .map_err(|err| AppErrorDto::database(err.to_string()))?;
        }

        Ok(())
    }

    pub fn link_event_to_materials(
        conn: &Connection,
        case_id: &str,
        event_id: &str,
        material_ids: &[String],
        link_note: &str,
        created_by_user_id: &str,
    ) -> Result<(), AppErrorDto> {
        for material_id in material_ids {
            conn.execute(
                r#"
                INSERT OR IGNORE INTO event_materials (
                    id,
                    case_id,
                    event_id,
                    material_id,
                    link_note,
                    created_by_user_id
                )
                VALUES (?1, ?2, ?3, ?4, ?5, ?6)
                "#,
                params![
                    Uuid::new_v4().to_string(),
                    case_id,
                    event_id,
                    material_id,
                    link_note,
                    created_by_user_id
                ],
            )
            .map_err(|err| AppErrorDto::database(err.to_string()))?;
        }

        Ok(())
    }

    pub fn get_timeline(
        conn: &Connection,
        case_id: &str,
        filters: &TimelineFiltersRecord,
    ) -> Result<Vec<TimelineEventDto>, AppErrorDto> {
        let mut sql = String::from(
            "
            SELECT
                e.id,
                e.case_id,
                e.event_code,
                e.event_type,
                e.title,
                e.description,
                e.event_date,
                e.event_time,
                e.date_precision,
                e.period_start,
                e.period_end,
                e.source_note,
                e.analyst_comment,
                e.include_in_report,
                (
                    SELECT COUNT(*)
                    FROM event_objects eo
                    WHERE eo.event_id = e.id
                ) AS linked_object_count,
                (
                    SELECT COUNT(*)
                    FROM event_materials em
                    WHERE em.event_id = e.id
                ) AS linked_material_count,
                e.created_by_user_id,
                e.created_at,
                e.updated_at
            FROM events e
            WHERE e.case_id = ?
              AND e.archived_at IS NULL
            ",
        );

        let mut query_params: Vec<Box<dyn ToSql>> = vec![Box::new(case_id.to_string())];

        if let Some(query) = &filters.query {
            sql.push_str(
                "
                AND (
                    e.event_code LIKE ?
                    OR e.title LIKE ?
                    OR e.description LIKE ?
                    OR e.source_note LIKE ?
                    OR e.analyst_comment LIKE ?
                )
                ",
            );

            let like_query = format!("%{}%", query);
            query_params.push(Box::new(like_query.clone()));
            query_params.push(Box::new(like_query.clone()));
            query_params.push(Box::new(like_query.clone()));
            query_params.push(Box::new(like_query.clone()));
            query_params.push(Box::new(like_query));
        }

        if let Some(event_type) = &filters.event_type {
            sql.push_str(" AND e.event_type = ? ");
            query_params.push(Box::new(event_type.clone()));
        }

        if let Some(object_id) = &filters.object_id {
            sql.push_str(
                "
                AND EXISTS (
                    SELECT 1
                    FROM event_objects eo
                    WHERE eo.event_id = e.id
                      AND eo.object_id = ?
                )
                ",
            );
            query_params.push(Box::new(object_id.clone()));
        }

        if let Some(material_id) = &filters.material_id {
            sql.push_str(
                "
                AND EXISTS (
                    SELECT 1
                    FROM event_materials em
                    WHERE em.event_id = e.id
                      AND em.material_id = ?
                )
                ",
            );
            query_params.push(Box::new(material_id.clone()));
        }

        if let Some(date_from) = &filters.date_from {
            sql.push_str(" AND e.event_date >= ? ");
            query_params.push(Box::new(date_from.clone()));
        }

        if let Some(date_to) = &filters.date_to {
            sql.push_str(" AND e.event_date <= ? ");
            query_params.push(Box::new(date_to.clone()));
        }

        if let Some(include_in_report) = filters.include_in_report {
            sql.push_str(" AND e.include_in_report = ? ");
            let value: i64 = if include_in_report { 1 } else { 0 };
            query_params.push(Box::new(value));
        }

        sql.push_str(
            "
            ORDER BY
                e.event_date ASC,
                e.event_time ASC,
                e.created_at ASC
            ",
        );

        let params = query_params
            .iter()
            .map(|value| value.as_ref())
            .collect::<Vec<&dyn ToSql>>();

        let mut stmt = conn
            .prepare(&sql)
            .map_err(|err| AppErrorDto::database(err.to_string()))?;

        let rows = stmt
            .query_map(params_from_iter(params), map_timeline_event_row)
            .map_err(|err| AppErrorDto::database(err.to_string()))?;

        let mut items = Vec::new();

        for row in rows {
            items.push(row.map_err(|err| AppErrorDto::database(err.to_string()))?);
        }

        Ok(items)
    }

    pub fn object_belongs_to_case(
        conn: &Connection,
        case_id: &str,
        object_id: &str,
    ) -> Result<bool, AppErrorDto> {
        let count: i64 = conn
            .query_row(
                r#"
                SELECT COUNT(*)
                FROM object_nodes
                WHERE id = ?1
                  AND case_id = ?2
                  AND archived_at IS NULL
                "#,
                params![object_id, case_id],
                |row| row.get(0),
            )
            .map_err(|err| AppErrorDto::database(err.to_string()))?;

        Ok(count > 0)
    }

    pub fn material_belongs_to_case(
        conn: &Connection,
        case_id: &str,
        material_id: &str,
    ) -> Result<bool, AppErrorDto> {
        let count: i64 = conn
            .query_row(
                r#"
                SELECT COUNT(*)
                FROM materials
                WHERE id = ?1
                  AND case_id = ?2
                  AND archived_at IS NULL
                "#,
                params![material_id, case_id],
                |row| row.get(0),
            )
            .map_err(|err| AppErrorDto::database(err.to_string()))?;

        Ok(count > 0)
    }

    pub fn event_belongs_to_case(
        conn: &Connection,
        case_id: &str,
        event_id: &str,
    ) -> Result<bool, AppErrorDto> {
        let count: i64 = conn
            .query_row(
                r#"
                SELECT COUNT(*)
                FROM events
                WHERE id = ?1
                  AND case_id = ?2
                  AND archived_at IS NULL
                "#,
                params![event_id, case_id],
                |row| row.get(0),
            )
            .map_err(|err| AppErrorDto::database(err.to_string()))?;

        Ok(count > 0)
    }

    pub fn update_event(conn: &Connection, record: &UpdateEventRecord) -> Result<(), AppErrorDto> {
        conn.execute(
            r#"
            UPDATE events
            SET
                event_type = ?3,
                title = ?4,
                description = ?5,
                event_date = ?6,
                event_time = ?7,
                date_precision = ?8,
                period_start = ?9,
                period_end = ?10,
                source_note = ?11,
                analyst_comment = ?12,
                include_in_report = ?13,
                updated_at = CURRENT_TIMESTAMP
            WHERE id = ?1
              AND case_id = ?2
              AND archived_at IS NULL
            "#,
            params![
                record.id,
                record.case_id,
                record.event_type,
                record.title,
                record.description,
                record.event_date,
                record.event_time,
                record.date_precision,
                record.period_start,
                record.period_end,
                record.source_note,
                record.analyst_comment,
                if record.include_in_report { 1 } else { 0 },
            ],
        )
        .map_err(|err| AppErrorDto::database(err.to_string()))?;

        Ok(())
    }

    pub fn replace_event_object_links(
        conn: &Connection,
        case_id: &str,
        event_id: &str,
        object_ids: &[String],
        link_note: &str,
        created_by_user_id: &str,
    ) -> Result<(), AppErrorDto> {
        conn.execute(
            "DELETE FROM event_objects WHERE event_id = ?1 AND case_id = ?2",
            params![event_id, case_id],
        )
        .map_err(|err| AppErrorDto::database(err.to_string()))?;

        Self::link_event_to_objects(
            conn,
            case_id,
            event_id,
            object_ids,
            link_note,
            created_by_user_id,
        )
    }

    pub fn replace_event_material_links(
        conn: &Connection,
        case_id: &str,
        event_id: &str,
        material_ids: &[String],
        link_note: &str,
        created_by_user_id: &str,
    ) -> Result<(), AppErrorDto> {
        conn.execute(
            "DELETE FROM event_materials WHERE event_id = ?1 AND case_id = ?2",
            params![event_id, case_id],
        )
        .map_err(|err| AppErrorDto::database(err.to_string()))?;

        Self::link_event_to_materials(
            conn,
            case_id,
            event_id,
            material_ids,
            link_note,
            created_by_user_id,
        )
    }

    pub fn get_event_by_id(
        conn: &Connection,
        case_id: &str,
        event_id: &str,
    ) -> Result<Option<EventDetailsDto>, AppErrorDto> {
        let items = Self::get_timeline(
            conn,
            case_id,
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
        let Some(event_item) = items.into_iter().find(|item| item.id == event_id) else {
            return Ok(None);
        };

        let linked_objects = Self::get_event_objects(conn, case_id, event_id)?;
        let linked_materials = Self::get_event_materials(conn, case_id, event_id)?;

        Ok(Some(EventDetailsDto {
            event_item,
            linked_objects,
            linked_materials,
        }))
    }

    fn get_event_objects(
        conn: &Connection,
        case_id: &str,
        event_id: &str,
    ) -> Result<Vec<EventLinkedObjectDto>, AppErrorDto> {
        let mut stmt = conn
            .prepare(
                r#"
                SELECT
                    eo.id,
                    eo.object_id,
                    o.object_code,
                    o.object_type,
                    o.title,
                    eo.link_note
                FROM event_objects eo
                JOIN object_nodes o ON o.id = eo.object_id
                WHERE eo.case_id = ?1
                  AND eo.event_id = ?2
                  AND o.archived_at IS NULL
                ORDER BY o.object_code ASC
                "#,
            )
            .map_err(|err| AppErrorDto::database(err.to_string()))?;

        let rows = stmt
            .query_map(params![case_id, event_id], |row| {
                Ok(EventLinkedObjectDto {
                    id: row.get(0)?,
                    object_id: row.get(1)?,
                    object_code: row.get(2)?,
                    object_type: row.get(3)?,
                    title: row.get(4)?,
                    link_note: row.get(5)?,
                })
            })
            .map_err(|err| AppErrorDto::database(err.to_string()))?;

        let mut items = Vec::new();

        for row in rows {
            items.push(row.map_err(|err| AppErrorDto::database(err.to_string()))?);
        }

        Ok(items)
    }

    pub fn soft_delete_event(
        conn: &Connection,
        case_id: &str,
        event_id: &str,
    ) -> Result<(), AppErrorDto> {
        let affected = conn
            .execute(
                "
                UPDATE events
                SET
                    archived_at = CURRENT_TIMESTAMP,
                    updated_at = CURRENT_TIMESTAMP
                WHERE id = ?1
                  AND case_id = ?2
                  AND archived_at IS NULL
                ",
                params![event_id, case_id],
            )
            .map_err(|err| AppErrorDto::database(err.to_string()))?;

        if affected == 0 {
            return Err(AppErrorDto::new(
                "ERR_EVENT_NOT_FOUND",
                "Событие не найдено",
                None,
            ));
        }

        Ok(())
    }

    pub fn set_event_report_include(
        conn: &Connection,
        case_id: &str,
        event_id: &str,
        include_in_report: bool,
    ) -> Result<TimelineEventDto, AppErrorDto> {
        let include_value: i64 = if include_in_report { 1 } else { 0 };

        let affected = conn
            .execute(
                "
                UPDATE events
                SET include_in_report = ?1,
                    updated_at = CURRENT_TIMESTAMP
                WHERE id = ?2
                  AND case_id = ?3
                  AND archived_at IS NULL
                ",
                params![include_value, event_id, case_id],
            )
            .map_err(|err| AppErrorDto::database(err.to_string()))?;

        if affected == 0 {
            return Err(AppErrorDto::new(
                "ERR_EVENT_NOT_FOUND",
                "Событие не найдено",
                None,
            ));
        }

        Self::get_event_list_item_by_id(conn, case_id, event_id)
    }

    pub fn get_event_list_item_by_id(
        conn: &Connection,
        case_id: &str,
        event_id: &str,
    ) -> Result<TimelineEventDto, AppErrorDto> {
        conn.query_row(
            "
            SELECT
                e.id,
                e.case_id,
                e.event_code,
                e.event_type,
                e.title,
                e.description,
                e.event_date,
                e.event_time,
                e.date_precision,
                e.period_start,
                e.period_end,
                e.source_note,
                e.analyst_comment,
                e.include_in_report,
                (
                    SELECT COUNT(*)
                    FROM event_objects eo
                    WHERE eo.event_id = e.id
                ) AS linked_object_count,
                (
                    SELECT COUNT(*)
                    FROM event_materials em
                    WHERE em.event_id = e.id
                ) AS linked_material_count,
                e.created_by_user_id,
                e.created_at,
                e.updated_at
            FROM events e
            WHERE e.id = ?1
              AND e.case_id = ?2
              AND e.archived_at IS NULL
            LIMIT 1
            ",
            params![event_id, case_id],
            map_timeline_event_row,
        )
        .map_err(|err| {
            if matches!(err, rusqlite::Error::QueryReturnedNoRows) {
                AppErrorDto::new("ERR_EVENT_NOT_FOUND", "Событие не найдено", None)
            } else {
                AppErrorDto::database(err.to_string())
            }
        })
    }

    fn get_event_materials(
        conn: &Connection,
        case_id: &str,
        event_id: &str,
    ) -> Result<Vec<EventLinkedMaterialDto>, AppErrorDto> {
        let mut stmt = conn
            .prepare(
                r#"
                SELECT
                    em.id,
                    em.material_id,
                    m.material_code,
                    m.title,
                    m.material_type,
                    em.link_note
                FROM event_materials em
                JOIN materials m ON m.id = em.material_id
                WHERE em.case_id = ?1
                  AND em.event_id = ?2
                  AND m.archived_at IS NULL
                ORDER BY m.material_code ASC
                "#,
            )
            .map_err(|err| AppErrorDto::database(err.to_string()))?;

        let rows = stmt
            .query_map(params![case_id, event_id], |row| {
                Ok(EventLinkedMaterialDto {
                    id: row.get(0)?,
                    material_id: row.get(1)?,
                    material_code: row.get(2)?,
                    title: row.get(3)?,
                    material_type: row.get(4)?,
                    link_note: row.get(5)?,
                })
            })
            .map_err(|err| AppErrorDto::database(err.to_string()))?;

        let mut items = Vec::new();

        for row in rows {
            items.push(row.map_err(|err| AppErrorDto::database(err.to_string()))?);
        }

        Ok(items)
    }
}
