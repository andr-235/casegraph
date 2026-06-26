use rusqlite::{params, Connection, OptionalExtension};

use crate::errors::app_error::AppErrorDto;

#[derive(Debug)]
pub struct MaterialRow {
    pub id: String,
    pub case_id: String,
    pub material_code: String,
    pub title: String,
    pub material_type: String,
    pub source_name: String,
    pub description: String,
    pub captured_at: Option<String>,
    pub include_in_report: i64,
    pub original_file_name: Option<String>,
    pub original_path: Option<String>,
    pub stored_file_path: Option<String>,
    pub file_size: Option<i64>,
    pub mime_type: Option<String>,
    pub sha256: Option<String>,
    pub integrity_status: String,
    pub created_by_user_id: String,
    pub created_at: String,
    pub updated_at: String,
}

#[derive(Debug)]
pub struct CreateMaterialRecord {
    pub id: String,
    pub case_id: String,
    pub material_code: String,
    pub title: String,
    pub material_type: String,
    pub source_name: String,
    pub description: String,
    pub captured_at: Option<String>,
    pub include_in_report: bool,
    pub created_by_user_id: String,
}

pub struct MaterialRepository;

impl MaterialRepository {
    pub fn get_next_material_code(conn: &Connection, case_id: &str) -> Result<String, AppErrorDto> {
        let next_number: i64 = conn
            .query_row(
                r#"
                SELECT COALESCE(
                    MAX(CAST(SUBSTR(material_code, 5) AS INTEGER)),
                    0
                ) + 1
                FROM materials
                WHERE case_id = ?1
                  AND material_code LIKE 'MAT-%'
                "#,
                params![case_id],
                |row| row.get(0),
            )
            .map_err(|err| AppErrorDto::database(err.to_string()))?;

        Ok(format!("MAT-{next_number:03}"))
    }

    pub fn create_material(
        conn: &Connection,
        record: CreateMaterialRecord,
    ) -> Result<(), AppErrorDto> {
        conn.execute(
            r#"
            INSERT INTO materials (
                id,
                case_id,
                material_code,
                title,
                material_type,
                source_name,
                description,
                captured_at,
                include_in_report,
                created_by_user_id
            )
            VALUES (?1, ?2, ?3, ?4, ?5, ?6, ?7, ?8, ?9, ?10)
            "#,
            params![
                record.id,
                record.case_id,
                record.material_code,
                record.title,
                record.material_type,
                record.source_name,
                record.description,
                record.captured_at,
                if record.include_in_report { 1 } else { 0 },
                record.created_by_user_id
            ],
        )
        .map_err(|err| AppErrorDto::database(err.to_string()))?;

        Ok(())
    }

    pub fn get_materials_by_case_id(
        conn: &Connection,
        case_id: &str,
    ) -> Result<Vec<MaterialRow>, AppErrorDto> {
        let mut statement = conn
            .prepare(
                r#"
                SELECT
                    id,
                    case_id,
                    material_code,
                    title,
                    material_type,
                    source_name,
                    description,
                    captured_at,
                    include_in_report,
                    original_file_name,
                    original_path,
                    stored_file_path,
                    file_size,
                    mime_type,
                    sha256,
                    integrity_status,
                    created_by_user_id,
                    created_at,
                    updated_at
                FROM materials
                WHERE case_id = ?1
                  AND archived_at IS NULL
                ORDER BY created_at DESC
                "#,
            )
            .map_err(|err| AppErrorDto::database(err.to_string()))?;

        let rows = statement
            .query_map(params![case_id], |row| {
                Ok(MaterialRow {
                    id: row.get(0)?,
                    case_id: row.get(1)?,
                    material_code: row.get(2)?,
                    title: row.get(3)?,
                    material_type: row.get(4)?,
                    source_name: row.get(5)?,
                    description: row.get(6)?,
                    captured_at: row.get(7)?,
                    include_in_report: row.get(8)?,
                    original_file_name: row.get(9)?,
                    original_path: row.get(10)?,
                    stored_file_path: row.get(11)?,
                    file_size: row.get(12)?,
                    mime_type: row.get(13)?,
                    sha256: row.get(14)?,
                    integrity_status: row.get(15)?,
                    created_by_user_id: row.get(16)?,
                    created_at: row.get(17)?,
                    updated_at: row.get(18)?,
                })
            })
            .map_err(|err| AppErrorDto::database(err.to_string()))?;

        let mut materials = Vec::new();

        for row in rows {
            materials.push(row.map_err(|err| AppErrorDto::database(err.to_string()))?);
        }

        Ok(materials)
    }

    pub fn get_material_by_id(
        conn: &Connection,
        material_id: &str,
    ) -> Result<Option<MaterialRow>, AppErrorDto> {
        conn.query_row(
            r#"
            SELECT
                id,
                case_id,
                material_code,
                title,
                material_type,
                source_name,
                description,
                captured_at,
                include_in_report,
                original_file_name,
                original_path,
                stored_file_path,
                file_size,
                mime_type,
                sha256,
                integrity_status,
                created_by_user_id,
                created_at,
                updated_at
            FROM materials
            WHERE id = ?1
              AND archived_at IS NULL
            LIMIT 1
            "#,
            params![material_id],
            |row| {
                Ok(MaterialRow {
                    id: row.get(0)?,
                    case_id: row.get(1)?,
                    material_code: row.get(2)?,
                    title: row.get(3)?,
                    material_type: row.get(4)?,
                    source_name: row.get(5)?,
                    description: row.get(6)?,
                    captured_at: row.get(7)?,
                    include_in_report: row.get(8)?,
                    original_file_name: row.get(9)?,
                    original_path: row.get(10)?,
                    stored_file_path: row.get(11)?,
                    file_size: row.get(12)?,
                    mime_type: row.get(13)?,
                    sha256: row.get(14)?,
                    integrity_status: row.get(15)?,
                    created_by_user_id: row.get(16)?,
                    created_at: row.get(17)?,
                    updated_at: row.get(18)?,
                })
            },
        )
        .optional()
        .map_err(|err| AppErrorDto::database(err.to_string()))
    }
}
