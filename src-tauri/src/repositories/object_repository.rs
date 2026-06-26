use rusqlite::{params, Connection, OptionalExtension};

use crate::domain::object_type::object_code_prefix;
use crate::domain::objects::ObjectListItemDto;
use crate::errors::app_error::AppErrorDto;

#[derive(Debug)]
pub struct CreateObjectRecord {
    pub id: String,
    pub case_id: String,
    pub object_code: String,
    pub object_type: String,
    pub title: String,
    pub value: Option<String>,
    pub description: String,
    pub is_key: bool,
    pub confidence_note: String,
    pub include_in_report: bool,
    pub created_by_user_id: String,
}

pub struct ObjectRepository;

impl ObjectRepository {
    pub fn generate_next_object_code(
        conn: &Connection,
        case_id: &str,
        object_type: &str,
    ) -> Result<String, AppErrorDto> {
        let prefix = object_code_prefix(object_type).ok_or_else(|| {
            AppErrorDto::new(
                "ERR_OBJECT_TYPE_INVALID",
                "Недопустимый тип объекта.",
                Some(object_type.to_string()),
            )
        })?;

        let like_pattern = format!("{prefix}-%");
        let number_start = (prefix.len() + 2) as i64;

        let last_code: Option<String> = conn
            .query_row(
                r#"
                SELECT object_code
                FROM object_nodes
                WHERE case_id = ?1
                  AND object_code LIKE ?2
                ORDER BY CAST(SUBSTR(object_code, ?3) AS INTEGER) DESC
                LIMIT 1
                "#,
                params![case_id, like_pattern, number_start],
                |row| row.get(0),
            )
            .optional()
            .map_err(|err| AppErrorDto::database(err.to_string()))?;

        let next_number = last_code
            .as_deref()
            .and_then(|code| code.split('-').nth(1))
            .and_then(|part| part.parse::<i64>().ok())
            .unwrap_or(0)
            + 1;

        Ok(format!("{prefix}-{next_number:03}"))
    }

    pub fn create(conn: &Connection, record: CreateObjectRecord) -> Result<(), AppErrorDto> {
        conn.execute(
            r#"
            INSERT INTO object_nodes (
                id,
                case_id,
                object_code,
                object_type,
                title,
                value,
                description,
                is_key,
                confidence_note,
                include_in_report,
                created_by_user_id
            )
            VALUES (?1, ?2, ?3, ?4, ?5, ?6, ?7, ?8, ?9, ?10, ?11)
            "#,
            params![
                record.id,
                record.case_id,
                record.object_code,
                record.object_type,
                record.title,
                record.value,
                record.description,
                if record.is_key { 1 } else { 0 },
                record.confidence_note,
                if record.include_in_report { 1 } else { 0 },
                record.created_by_user_id,
            ],
        )
        .map_err(|err| AppErrorDto::database(err.to_string()))?;

        Ok(())
    }

    pub fn list_by_case(
        conn: &Connection,
        case_id: &str,
    ) -> Result<Vec<ObjectListItemDto>, AppErrorDto> {
        let mut stmt = conn
            .prepare(
                r#"
                SELECT
                    o.id,
                    o.case_id,
                    o.object_code,
                    o.object_type,
                    o.title,
                    o.value,
                    o.description,
                    o.is_key,
                    o.include_in_report,
                    (
                        SELECT COUNT(*)
                        FROM object_materials om
                        WHERE om.object_id = o.id
                    ) AS linked_material_count,
                    0 AS relation_count,
                    o.created_at,
                    o.updated_at
                FROM object_nodes o
                WHERE o.case_id = ?1
                  AND o.archived_at IS NULL
                ORDER BY o.created_at DESC
                "#,
            )
            .map_err(|err| AppErrorDto::database(err.to_string()))?;

        let rows = stmt
            .query_map(params![case_id], |row| {
                let is_key: i64 = row.get(7)?;
                let include_in_report: i64 = row.get(8)?;

                Ok(ObjectListItemDto {
                    id: row.get(0)?,
                    case_id: row.get(1)?,
                    object_code: row.get(2)?,
                    object_type: row.get(3)?,
                    title: row.get(4)?,
                    value: row.get(5)?,
                    description: row.get(6)?,
                    is_key: is_key == 1,
                    include_in_report: include_in_report == 1,
                    linked_material_count: row.get(9)?,
                    relation_count: row.get(10)?,
                    created_at: row.get(11)?,
                    updated_at: row.get(12)?,
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
