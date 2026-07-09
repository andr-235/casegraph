use rusqlite::{params, Connection};

use crate::domain::case_overview::{ActivityItemDto, ObjectPreviewDto};
use crate::errors::app_error::AppErrorDto;

/// Репозиторий для сводки по делу и карточки дела.
/// Все методы — query-only, read-only.
pub struct CaseSummaryRepository;

impl CaseSummaryRepository {
    /// Количество объектов дела (не удалённых)
    pub fn count_objects(conn: &Connection, case_id: &str) -> Result<i64, AppErrorDto> {
        log::debug!("[CaseSummaryRepository] count_objects case_id={}", case_id);
        conn.query_row(
            "SELECT COUNT(*) FROM objects WHERE case_id = ?1 AND is_deleted = 0",
            params![case_id],
            |row| row.get::<_, i64>(0),
        )
        .map_err(|err| {
            log::error!("[CaseSummaryRepository] count_objects error: {}", err);
            AppErrorDto::database(err.to_string())
        })
        .map(|count| {
            log::debug!(
                "[CaseSummaryRepository] count_objects result={}",
                count
            );
            count
        })
    }

    /// Количество ключевых объектов дела
    pub fn count_key_objects(conn: &Connection, case_id: &str) -> Result<i64, AppErrorDto> {
        log::debug!(
            "[CaseSummaryRepository] count_key_objects case_id={}",
            case_id
        );
        conn.query_row(
            "SELECT COUNT(*) FROM objects WHERE case_id = ?1 AND is_deleted = 0 AND is_key = 1",
            params![case_id],
            |row| row.get::<_, i64>(0),
        )
        .map_err(|err| {
            log::error!(
                "[CaseSummaryRepository] count_key_objects error: {}",
                err
            );
            AppErrorDto::database(err.to_string())
        })
        .map(|count| {
            log::debug!(
                "[CaseSummaryRepository] count_key_objects result={}",
                count
            );
            count
        })
    }

    /// Количество материалов дела (не удалённых)
    pub fn count_materials(conn: &Connection, case_id: &str) -> Result<i64, AppErrorDto> {
        log::debug!(
            "[CaseSummaryRepository] count_materials case_id={}",
            case_id
        );
        conn.query_row(
            "SELECT COUNT(*) FROM materials WHERE case_id = ?1 AND is_deleted = 0",
            params![case_id],
            |row| row.get::<_, i64>(0),
        )
        .map_err(|err| {
            log::error!(
                "[CaseSummaryRepository] count_materials error: {}",
                err
            );
            AppErrorDto::database(err.to_string())
        })
        .map(|count| {
            log::debug!(
                "[CaseSummaryRepository] count_materials result={}",
                count
            );
            count
        })
    }

    /// Количество материалов с подтверждёнными проблемами целостности
    /// (исключая not_checked — это значение по умолчанию для непроверенных)
    pub fn count_materials_with_integrity_issues(
        conn: &Connection,
        case_id: &str,
    ) -> Result<i64, AppErrorDto> {
        log::debug!(
            "[CaseSummaryRepository] count_materials_with_integrity_issues case_id={}",
            case_id
        );
        conn.query_row(
            "SELECT COUNT(*) FROM materials WHERE case_id = ?1 AND is_deleted = 0 AND integrity_status IN ('mismatch', 'missing', 'read_error')",
            params![case_id],
            |row| row.get::<_, i64>(0),
        )
        .map_err(|err| {
            log::error!(
                "[CaseSummaryRepository] count_materials_with_integrity_issues error: {}",
                err
            );
            AppErrorDto::database(err.to_string())
        })
        .map(|count| {
            log::debug!(
                "[CaseSummaryRepository] count_materials_with_integrity_issues result={}",
                count
            );
            count
        })
    }

    /// Количество связей (не удалённых)
    pub fn count_relations(conn: &Connection, case_id: &str) -> Result<i64, AppErrorDto> {
        log::debug!(
            "[CaseSummaryRepository] count_relations case_id={}",
            case_id
        );
        conn.query_row(
            "SELECT COUNT(*) FROM relations WHERE case_id = ?1 AND is_deleted = 0",
            params![case_id],
            |row| row.get::<_, i64>(0),
        )
        .map_err(|err| {
            log::error!(
                "[CaseSummaryRepository] count_relations error: {}",
                err
            );
            AppErrorDto::database(err.to_string())
        })
        .map(|count| {
            log::debug!(
                "[CaseSummaryRepository] count_relations result={}",
                count
            );
            count
        })
    }

    /// Количество событий хронологии (не удалённых)
    pub fn count_events(conn: &Connection, case_id: &str) -> Result<i64, AppErrorDto> {
        log::debug!("[CaseSummaryRepository] count_events case_id={}", case_id);
        conn.query_row(
            "SELECT COUNT(*) FROM timeline_events WHERE case_id = ?1 AND is_deleted = 0",
            params![case_id],
            |row| row.get::<_, i64>(0),
        )
        .map_err(|err| {
            log::error!("[CaseSummaryRepository] count_events error: {}", err);
            AppErrorDto::database(err.to_string())
        })
        .map(|count| {
            log::debug!(
                "[CaseSummaryRepository] count_events result={}",
                count
            );
            count
        })
    }

    /// Количество событий, включённых в справку
    pub fn count_report_events(conn: &Connection, case_id: &str) -> Result<i64, AppErrorDto> {
        log::debug!(
            "[CaseSummaryRepository] count_report_events case_id={}",
            case_id
        );
        conn.query_row(
            "SELECT COUNT(*) FROM timeline_events WHERE case_id = ?1 AND is_deleted = 0 AND include_in_report = 1",
            params![case_id],
            |row| row.get::<_, i64>(0),
        )
        .map_err(|err| {
            log::error!(
                "[CaseSummaryRepository] count_report_events error: {}",
                err
            );
            AppErrorDto::database(err.to_string())
        })
        .map(|count| {
            log::debug!(
                "[CaseSummaryRepository] count_report_events result={}",
                count
            );
            count
        })
    }

    /// Ключевые объекты (до `limit` штук)
    pub fn get_key_objects(
        conn: &Connection,
        case_id: &str,
        limit: i64,
    ) -> Result<Vec<ObjectPreviewDto>, AppErrorDto> {
        log::debug!(
            "[CaseSummaryRepository] get_key_objects case_id={} limit={}",
            case_id,
            limit
        );
        let mut stmt = conn
            .prepare(
                "SELECT id, object_code, object_type, title, is_key \
                 FROM objects WHERE case_id = ?1 AND is_deleted = 0 AND is_key = 1 \
                 ORDER BY updated_at DESC LIMIT ?2",
            )
            .map_err(|err| {
                log::error!(
                    "[CaseSummaryRepository] get_key_objects prepare error: {}",
                    err
                );
                AppErrorDto::database(err.to_string())
            })?;

        let rows = stmt
            .query_map(params![case_id, limit], |row| {
                Ok(ObjectPreviewDto {
                    id: row.get(0)?,
                    object_code: row.get(1)?,
                    object_type: row.get(2)?,
                    title: row.get(3)?,
                    is_key: row.get(4)?,
                })
            })
            .map_err(|err| {
                log::error!(
                    "[CaseSummaryRepository] get_key_objects query error: {}",
                    err
                );
                AppErrorDto::database(err.to_string())
            })?;

        let mut result = Vec::new();
        for row in rows {
            result.push(row.map_err(|err| {
                log::error!(
                    "[CaseSummaryRepository] get_key_objects row error: {}",
                    err
                );
                AppErrorDto::database(err.to_string())
            })?);
        }

        log::debug!(
            "[CaseSummaryRepository] get_key_objects result_count={}",
            result.len()
        );
        Ok(result)
    }

    /// Последние изменения по делу (до `limit` записей).
    /// UNION ALL трёх запросов (objects, materials, relations) с сортировкой по updated_at.
    pub fn get_recent_activity(
        conn: &Connection,
        case_id: &str,
        limit: i64,
    ) -> Result<Vec<ActivityItemDto>, AppErrorDto> {
        log::debug!(
            "[CaseSummaryRepository] get_recent_activity case_id={} limit={}",
            case_id,
            limit
        );

        let sql = r#"
            SELECT entity_type, entity_id, code, title, timestamp, action FROM (
                SELECT 'object' AS entity_type, id AS entity_id, object_code AS code, title, updated_at AS timestamp, 'updated' AS action
                FROM objects WHERE case_id = ?1 AND is_deleted = 0
                UNION ALL
                SELECT 'material' AS entity_type, id AS entity_id, material_code AS code, title, updated_at AS timestamp, 'updated' AS action
                FROM materials WHERE case_id = ?1 AND is_deleted = 0
                UNION ALL
                SELECT 'relation' AS entity_type, id AS entity_id, relation_code AS code, '' AS title, updated_at AS timestamp, 'updated' AS action
                FROM relations WHERE case_id = ?1 AND is_deleted = 0
            ) ORDER BY timestamp DESC LIMIT ?2
        "#;

        let mut stmt = conn.prepare(sql).map_err(|err| {
            log::error!(
                "[CaseSummaryRepository] get_recent_activity prepare error: {}",
                err
            );
            AppErrorDto::database(err.to_string())
        })?;

        let rows = stmt
            .query_map(params![case_id, limit], |row| {
                Ok(ActivityItemDto {
                    entity_type: row.get(0)?,
                    entity_id: row.get(1)?,
                    code: row.get(2)?,
                    title: row.get(3)?,
                    timestamp: row.get(4)?,
                    action: row.get(5)?,
                })
            })
            .map_err(|err| {
                log::error!(
                    "[CaseSummaryRepository] get_recent_activity query error: {}",
                    err
                );
                AppErrorDto::database(err.to_string())
            })?;

        let mut result = Vec::new();
        for row in rows {
            result.push(row.map_err(|err| {
                log::error!(
                    "[CaseSummaryRepository] get_recent_activity row error: {}",
                    err
                );
                AppErrorDto::database(err.to_string())
            })?);
        }

        log::debug!(
            "[CaseSummaryRepository] get_recent_activity result_count={}",
            result.len()
        );
        Ok(result)
    }

    /// Общий updated_at — максимальное значение среди всех затронутых записей
    pub fn get_max_updated_at(conn: &Connection, case_id: &str) -> Result<String, AppErrorDto> {
        log::debug!(
            "[CaseSummaryRepository] get_max_updated_at case_id={}",
            case_id
        );
        let sql = r#"
            SELECT MAX(ts) FROM (
                SELECT updated_at AS ts FROM objects WHERE case_id = ?1 AND is_deleted = 0
                UNION ALL
                SELECT updated_at AS ts FROM materials WHERE case_id = ?1 AND is_deleted = 0
                UNION ALL
                SELECT updated_at AS ts FROM relations WHERE case_id = ?1 AND is_deleted = 0
                UNION ALL
                SELECT updated_at AS ts FROM timeline_events WHERE case_id = ?1 AND is_deleted = 0
            )
        "#;

        let result: Option<String> = conn
            .query_row(sql, params![case_id], |row| row.get(0))
            .map_err(|err| {
                log::error!(
                    "[CaseSummaryRepository] get_max_updated_at error: {}",
                    err
                );
                AppErrorDto::database(err.to_string())
            })?;

        let updated_at = result.unwrap_or_default();

        if updated_at.is_empty() {
            log::warn!(
                "[CaseSummaryRepository] get_max_updated_at — нет записей для case_id={}",
                case_id
            );
        }

        log::debug!(
            "[CaseSummaryRepository] get_max_updated_at result={}",
            updated_at
        );
        Ok(updated_at)
    }
}
