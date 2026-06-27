use rusqlite::{params, Connection};

use crate::domain::graph::{GraphEdgeDto, GraphNodeDto};
use crate::errors::app_error::AppErrorDto;

pub struct GraphRepository;

impl GraphRepository {
    pub fn get_nodes(conn: &Connection, case_id: &str) -> Result<Vec<GraphNodeDto>, AppErrorDto> {
        let mut stmt = conn
            .prepare(
                r#"
                SELECT
                    id,
                    case_id,
                    object_code,
                    object_type,
                    title,
                    value,
                    is_key,
                    include_in_report
                FROM object_nodes
                WHERE case_id = ?1
                  AND archived_at IS NULL
                ORDER BY is_key DESC, object_code ASC
                "#,
            )
            .map_err(|err| AppErrorDto::database(err.to_string()))?;

        let rows = stmt
            .query_map(params![case_id], |row| {
                let is_key_raw: i64 = row.get(6)?;
                let include_in_report_raw: i64 = row.get(7)?;

                Ok(GraphNodeDto {
                    id: row.get(0)?,
                    case_id: row.get(1)?,
                    object_code: row.get(2)?,
                    object_type: row.get(3)?,
                    title: row.get(4)?,
                    value: row.get(5)?,
                    is_key: is_key_raw != 0,
                    include_in_report: include_in_report_raw != 0,
                })
            })
            .map_err(|err| AppErrorDto::database(err.to_string()))?;

        let mut nodes = Vec::new();

        for row in rows {
            nodes.push(row.map_err(|err| AppErrorDto::database(err.to_string()))?);
        }

        Ok(nodes)
    }

    pub fn get_edges(conn: &Connection, case_id: &str) -> Result<Vec<GraphEdgeDto>, AppErrorDto> {
        let mut stmt = conn
            .prepare(
                r#"
                SELECT
                    r.id,
                    r.case_id,
                    r.relation_code,
                    r.source_object_id,
                    r.target_object_id,
                    r.relation_type,
                    r.title,
                    r.basis,
                    r.confidence_level,
                    r.supporting_material_id,
                    r.include_in_report
                FROM relations r
                INNER JOIN object_nodes source_object
                    ON source_object.id = r.source_object_id
                   AND source_object.case_id = r.case_id
                   AND source_object.archived_at IS NULL
                INNER JOIN object_nodes target_object
                    ON target_object.id = r.target_object_id
                   AND target_object.case_id = r.case_id
                   AND target_object.archived_at IS NULL
                WHERE r.case_id = ?1
                  AND r.archived_at IS NULL
                ORDER BY r.relation_code ASC
                "#,
            )
            .map_err(|err| AppErrorDto::database(err.to_string()))?;

        let rows = stmt
            .query_map(params![case_id], |row| {
                let include_in_report_raw: i64 = row.get(10)?;

                Ok(GraphEdgeDto {
                    id: row.get(0)?,
                    case_id: row.get(1)?,
                    relation_code: row.get(2)?,
                    source_object_id: row.get(3)?,
                    target_object_id: row.get(4)?,
                    relation_type: row.get(5)?,
                    title: row.get(6)?,
                    basis: row.get(7)?,
                    confidence_level: row.get(8)?,
                    supporting_material_id: row.get(9)?,
                    include_in_report: include_in_report_raw != 0,
                })
            })
            .map_err(|err| AppErrorDto::database(err.to_string()))?;

        let mut edges = Vec::new();

        for row in rows {
            edges.push(row.map_err(|err| AppErrorDto::database(err.to_string()))?);
        }

        Ok(edges)
    }
}
