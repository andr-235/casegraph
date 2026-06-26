use rusqlite::{params, Connection, OptionalExtension};

use crate::domain::relations::{
    RelationDetailsDto, RelationListItemDto, RelationMaterialDto, RelationObjectDto,
};
use crate::errors::app_error::AppErrorDto;

#[derive(Debug)]
pub struct CreateRelationRecord {
    pub id: String,
    pub case_id: String,
    pub relation_code: String,
    pub source_object_id: String,
    pub target_object_id: String,
    pub relation_type: String,
    pub title: Option<String>,
    pub basis: String,
    pub confidence_level: String,
    pub supporting_material_id: Option<String>,
    pub analyst_comment: Option<String>,
    pub include_in_report: bool,
    pub created_by_user_id: String,
}

#[derive(Debug)]
pub struct UpdateRelationRecord {
    pub case_id: String,
    pub relation_id: String,
    pub relation_type: String,
    pub title: Option<String>,
    pub basis: String,
    pub confidence_level: String,
    pub supporting_material_id: Option<String>,
    pub analyst_comment: Option<String>,
    pub include_in_report: bool,
}

#[derive(Debug, Clone)]
pub struct ObjectCaseInfo {
    pub id: String,
    pub case_id: String,
}

#[derive(Debug, Clone)]
pub struct MaterialCaseInfo {
    pub id: String,
    pub case_id: String,
}

pub struct RelationRepository;

impl RelationRepository {
    pub fn generate_next_relation_code(
        conn: &Connection,
        case_id: &str,
    ) -> Result<String, AppErrorDto> {
        let last_code: Option<String> = conn
            .query_row(
                r#"
                SELECT relation_code
                FROM relations
                WHERE case_id = ?1
                ORDER BY CAST(SUBSTR(relation_code, 5) AS INTEGER) DESC
                LIMIT 1
                "#,
                params![case_id],
                |row| row.get(0),
            )
            .optional()
            .map_err(|err| AppErrorDto::database(err.to_string()))?;

        let next_number = last_code
            .as_deref()
            .and_then(|code| code.strip_prefix("REL-"))
            .and_then(|number| number.parse::<i64>().ok())
            .unwrap_or(0)
            + 1;

        Ok(format!("REL-{next_number:03}"))
    }

    pub fn find_object_case_info(
        conn: &Connection,
        object_id: &str,
    ) -> Result<Option<ObjectCaseInfo>, AppErrorDto> {
        conn.query_row(
            r#"
            SELECT id, case_id
            FROM object_nodes
            WHERE id = ?1
              AND archived_at IS NULL
            LIMIT 1
            "#,
            params![object_id],
            |row| {
                Ok(ObjectCaseInfo {
                    id: row.get(0)?,
                    case_id: row.get(1)?,
                })
            },
        )
        .optional()
        .map_err(|err| AppErrorDto::database(err.to_string()))
    }

    pub fn find_material_case_info(
        conn: &Connection,
        material_id: &str,
    ) -> Result<Option<MaterialCaseInfo>, AppErrorDto> {
        conn.query_row(
            r#"
            SELECT id, case_id
            FROM materials
            WHERE id = ?1
              AND archived_at IS NULL
            LIMIT 1
            "#,
            params![material_id],
            |row| {
                Ok(MaterialCaseInfo {
                    id: row.get(0)?,
                    case_id: row.get(1)?,
                })
            },
        )
        .optional()
        .map_err(|err| AppErrorDto::database(err.to_string()))
    }

    pub fn create(conn: &Connection, record: CreateRelationRecord) -> Result<(), AppErrorDto> {
        conn.execute(
            r#"
            INSERT INTO relations (
                id,
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
                include_in_report,
                created_by_user_id
            )
            VALUES (?1, ?2, ?3, ?4, ?5, ?6, ?7, ?8, ?9, ?10, ?11, ?12, ?13)
            "#,
            params![
                record.id,
                record.case_id,
                record.relation_code,
                record.source_object_id,
                record.target_object_id,
                record.relation_type,
                record.title,
                record.basis,
                record.confidence_level,
                record.supporting_material_id,
                record.analyst_comment,
                if record.include_in_report { 1 } else { 0 },
                record.created_by_user_id,
            ],
        )
        .map_err(|err| AppErrorDto::database(err.to_string()))?;

        Ok(())
    }

    pub fn get_by_id(
        conn: &Connection,
        relation_id: &str,
    ) -> Result<Option<RelationListItemDto>, AppErrorDto> {
        conn.query_row(
            relation_select_sql_with_where("relations.id = ?1").as_str(),
            params![relation_id],
            map_relation_row,
        )
        .optional()
        .map_err(|err| AppErrorDto::database(err.to_string()))
    }

    pub fn list_by_case(
        conn: &Connection,
        case_id: &str,
    ) -> Result<Vec<RelationListItemDto>, AppErrorDto> {
        let mut stmt = conn
            .prepare(
                relation_select_sql_with_where(
                    "relations.case_id = ?1 AND relations.archived_at IS NULL",
                )
                .as_str(),
            )
            .map_err(|err| AppErrorDto::database(err.to_string()))?;

        let rows = stmt
            .query_map(params![case_id], map_relation_row)
            .map_err(|err| AppErrorDto::database(err.to_string()))?;

        let mut items = Vec::new();

        for row in rows {
            items.push(row.map_err(|err| AppErrorDto::database(err.to_string()))?);
        }

        Ok(items)
    }

    pub fn update_relation(
        conn: &Connection,
        record: UpdateRelationRecord,
    ) -> Result<(), AppErrorDto> {
        let changed_count = conn
            .execute(
                r#"
                UPDATE relations
                SET
                    relation_type = ?3,
                    title = ?4,
                    basis = ?5,
                    confidence_level = ?6,
                    supporting_material_id = ?7,
                    analyst_comment = ?8,
                    include_in_report = ?9,
                    updated_at = CURRENT_TIMESTAMP
                WHERE id = ?1
                  AND case_id = ?2
                  AND archived_at IS NULL
                "#,
                params![
                    record.relation_id,
                    record.case_id,
                    record.relation_type,
                    record.title,
                    record.basis,
                    record.confidence_level,
                    record.supporting_material_id,
                    record.analyst_comment,
                    if record.include_in_report { 1 } else { 0 },
                ],
            )
            .map_err(|err| AppErrorDto::database(err.to_string()))?;

        if changed_count == 0 {
            return Err(AppErrorDto::new(
                "ERR_RELATION_NOT_FOUND",
                "Связь не найдена.",
                None,
            ));
        }

        Ok(())
    }

    pub fn soft_delete_relation(
        conn: &Connection,
        case_id: &str,
        relation_id: &str,
    ) -> Result<(), AppErrorDto> {
        let changed_count = conn
            .execute(
                r#"
                UPDATE relations
                SET
                    archived_at = CURRENT_TIMESTAMP,
                    updated_at = CURRENT_TIMESTAMP
                WHERE id = ?1
                  AND case_id = ?2
                  AND archived_at IS NULL
                "#,
                params![relation_id, case_id],
            )
            .map_err(|err| AppErrorDto::database(err.to_string()))?;

        if changed_count == 0 {
            return Err(AppErrorDto::new(
                "ERR_RELATION_NOT_FOUND",
                "Связь не найдена.",
                None,
            ));
        }

        Ok(())
    }

    pub fn get_details_by_id(
        conn: &Connection,
        case_id: &str,
        relation_id: &str,
    ) -> Result<Option<RelationDetailsDto>, AppErrorDto> {
        conn.query_row(
            relation_details_select_sql().as_str(),
            params![case_id, relation_id],
            map_relation_details_row,
        )
        .optional()
        .map_err(|err| AppErrorDto::database(err.to_string()))
    }
}

fn relation_select_sql_with_where(where_sql: &str) -> String {
    format!(
        r#"
        SELECT
            relations.id,
            relations.case_id,
            relations.relation_code,
            relations.relation_type,
            relations.title,
            relations.basis,
            relations.confidence_level,
            relations.include_in_report,
            relations.created_at,
            relations.updated_at,

            source_object.id,
            source_object.object_code,
            source_object.object_type,
            source_object.title,
            source_object.value,
            source_object.is_key,

            target_object.id,
            target_object.object_code,
            target_object.object_type,
            target_object.title,
            target_object.value,
            target_object.is_key,

            materials.id,
            materials.material_code,
            materials.title,
            materials.material_type,
            materials.integrity_status
        FROM relations
        INNER JOIN object_nodes AS source_object
            ON source_object.id = relations.source_object_id
        INNER JOIN object_nodes AS target_object
            ON target_object.id = relations.target_object_id
        LEFT JOIN materials
            ON materials.id = relations.supporting_material_id
        WHERE {where_sql}
        ORDER BY relations.created_at DESC
        "#
    )
}

fn map_relation_row(row: &rusqlite::Row<'_>) -> rusqlite::Result<RelationListItemDto> {
    let supporting_material_id: Option<String> = row.get(22)?;

    let supporting_material = match supporting_material_id {
        Some(id) => Some(RelationMaterialDto {
            id,
            material_code: row.get(23)?,
            title: row.get(24)?,
            material_type: row.get(25)?,
            integrity_status: row.get(26)?,
        }),
        None => None,
    };

    Ok(RelationListItemDto {
        id: row.get(0)?,
        case_id: row.get(1)?,
        relation_code: row.get(2)?,
        relation_type: row.get(3)?,
        title: row.get(4)?,
        basis: row.get(5)?,
        confidence_level: row.get(6)?,
        include_in_report: row.get::<_, i64>(7)? == 1,
        created_at: row.get(8)?,
        updated_at: row.get(9)?,
        source_object: RelationObjectDto {
            id: row.get(10)?,
            object_code: row.get(11)?,
            object_type: row.get(12)?,
            title: row.get(13)?,
            value: row.get(14)?,
            is_key: row.get::<_, i64>(15)? == 1,
        },
        target_object: RelationObjectDto {
            id: row.get(16)?,
            object_code: row.get(17)?,
            object_type: row.get(18)?,
            title: row.get(19)?,
            value: row.get(20)?,
            is_key: row.get::<_, i64>(21)? == 1,
        },
        supporting_material,
    })
}

fn relation_details_select_sql() -> String {
    r#"
    SELECT
        relations.id,
        relations.case_id,
        relations.relation_code,
        relations.relation_type,
        relations.title,
        relations.basis,
        relations.confidence_level,
        relations.analyst_comment,
        relations.include_in_report,
        relations.created_at,
        relations.updated_at,

        source_object.id,
        source_object.object_code,
        source_object.object_type,
        source_object.title,
        source_object.value,
        source_object.is_key,

        target_object.id,
        target_object.object_code,
        target_object.object_type,
        target_object.title,
        target_object.value,
        target_object.is_key,

        materials.id,
        materials.material_code,
        materials.title,
        materials.material_type,
        materials.integrity_status
    FROM relations
    INNER JOIN object_nodes AS source_object
        ON source_object.id = relations.source_object_id
    INNER JOIN object_nodes AS target_object
        ON target_object.id = relations.target_object_id
    LEFT JOIN materials
        ON materials.id = relations.supporting_material_id
    WHERE relations.case_id = ?1
      AND relations.id = ?2
      AND relations.archived_at IS NULL
    LIMIT 1
    "#
    .to_string()
}

fn map_relation_details_row(row: &rusqlite::Row<'_>) -> rusqlite::Result<RelationDetailsDto> {
    let supporting_material_id: Option<String> = row.get(23)?;

    let supporting_material = match supporting_material_id {
        Some(id) => Some(RelationMaterialDto {
            id,
            material_code: row.get(24)?,
            title: row.get(25)?,
            material_type: row.get(26)?,
            integrity_status: row.get(27)?,
        }),
        None => None,
    };

    Ok(RelationDetailsDto {
        id: row.get(0)?,
        case_id: row.get(1)?,
        relation_code: row.get(2)?,
        relation_type: row.get(3)?,
        title: row.get(4)?,
        basis: row.get(5)?,
        confidence_level: row.get(6)?,
        analyst_comment: row.get(7)?,
        include_in_report: row.get::<_, i64>(8)? == 1,
        created_at: row.get(9)?,
        updated_at: row.get(10)?,
        source_object: RelationObjectDto {
            id: row.get(11)?,
            object_code: row.get(12)?,
            object_type: row.get(13)?,
            title: row.get(14)?,
            value: row.get(15)?,
            is_key: row.get::<_, i64>(16)? == 1,
        },
        target_object: RelationObjectDto {
            id: row.get(17)?,
            object_code: row.get(18)?,
            object_type: row.get(19)?,
            title: row.get(20)?,
            value: row.get(21)?,
            is_key: row.get::<_, i64>(22)? == 1,
        },
        supporting_material,
    })
}
