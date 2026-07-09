use rusqlite::{params, Connection};

use crate::domain::case_overview::{ActivityItemDto, ObjectPreviewDto};
use crate::errors::app_error::AppErrorDto;

#[cfg(test)]
mod tests {
    use rusqlite::Connection;

    use crate::repositories::case_summary_repository::CaseSummaryRepository;

    /// Вспомогательная функция: создаёт in-memory SQLite с таблицами для тестов
    fn setup_db() -> Connection {
        let conn = Connection::open_in_memory().unwrap();

        conn.execute_batch(
            "
            CREATE TABLE IF NOT EXISTS cases (
                id TEXT PRIMARY KEY,
                case_code TEXT NOT NULL UNIQUE,
                title TEXT NOT NULL,
                subject TEXT NOT NULL,
                description TEXT NOT NULL DEFAULT '',
                status TEXT NOT NULL DEFAULT 'draft',
                created_by_user_id TEXT NOT NULL DEFAULT '',
                created_at TEXT NOT NULL DEFAULT CURRENT_TIMESTAMP,
                updated_at TEXT NOT NULL DEFAULT CURRENT_TIMESTAMP
            );

            CREATE TABLE IF NOT EXISTS objects (
                id TEXT PRIMARY KEY,
                case_id TEXT NOT NULL,
                object_code TEXT NOT NULL,
                object_type TEXT NOT NULL,
                title TEXT NOT NULL,
                description TEXT NOT NULL DEFAULT '',
                is_key INTEGER NOT NULL DEFAULT 0,
                is_deleted INTEGER NOT NULL DEFAULT 0,
                include_in_report INTEGER NOT NULL DEFAULT 1,
                created_by_user_id TEXT NOT NULL DEFAULT '',
                created_at TEXT NOT NULL DEFAULT CURRENT_TIMESTAMP,
                updated_at TEXT NOT NULL DEFAULT CURRENT_TIMESTAMP
            );

            CREATE TABLE IF NOT EXISTS materials (
                id TEXT PRIMARY KEY,
                case_id TEXT NOT NULL,
                material_code TEXT NOT NULL,
                title TEXT NOT NULL,
                material_type TEXT NOT NULL,
                integrity_status TEXT NOT NULL DEFAULT 'not_checked',
                is_deleted INTEGER NOT NULL DEFAULT 0,
                include_in_report INTEGER NOT NULL DEFAULT 1,
                created_by_user_id TEXT NOT NULL DEFAULT '',
                created_at TEXT NOT NULL DEFAULT CURRENT_TIMESTAMP,
                updated_at TEXT NOT NULL DEFAULT CURRENT_TIMESTAMP
            );

            CREATE TABLE IF NOT EXISTS relations (
                id TEXT PRIMARY KEY,
                case_id TEXT NOT NULL,
                relation_code TEXT NOT NULL,
                relation_type TEXT NOT NULL,
                source_object_id TEXT NOT NULL,
                target_object_id TEXT NOT NULL,
                is_deleted INTEGER NOT NULL DEFAULT 0,
                include_in_report INTEGER NOT NULL DEFAULT 1,
                created_by_user_id TEXT NOT NULL DEFAULT '',
                created_at TEXT NOT NULL DEFAULT CURRENT_TIMESTAMP,
                updated_at TEXT NOT NULL DEFAULT CURRENT_TIMESTAMP
            );

            CREATE TABLE IF NOT EXISTS timeline_events (
                id TEXT PRIMARY KEY,
                case_id TEXT NOT NULL,
                event_code TEXT NOT NULL,
                event_type TEXT NOT NULL DEFAULT 'fact',
                title TEXT NOT NULL,
                event_date TEXT NOT NULL,
                is_deleted INTEGER NOT NULL DEFAULT 0,
                include_in_report INTEGER NOT NULL DEFAULT 1,
                created_by_user_id TEXT NOT NULL DEFAULT '',
                created_at TEXT NOT NULL DEFAULT CURRENT_TIMESTAMP,
                updated_at TEXT NOT NULL DEFAULT CURRENT_TIMESTAMP
            );
            ",
        )
        .unwrap();

        conn
    }

    /// Вставляет тестовое дело и возвращает его ID
    fn insert_case(conn: &Connection, id: &str, code: &str, title: &str) {
        conn.execute(
            "INSERT INTO cases (id, case_code, title, subject) VALUES (?1, ?2, ?3, ?4)",
            rusqlite::params![id, code, title, "Test subject"],
        )
        .unwrap();
    }

    /// Вставляет тестовый объект
    fn insert_object(
        conn: &Connection,
        id: &str,
        case_id: &str,
        code: &str,
        obj_type: &str,
        title: &str,
        is_key: bool,
        is_deleted: bool,
        updated_at: &str,
    ) {
        conn.execute(
            "INSERT INTO objects (id, case_id, object_code, object_type, title, is_key, is_deleted, updated_at)
             VALUES (?1, ?2, ?3, ?4, ?5, ?6, ?7, ?8)",
            rusqlite::params![id, case_id, code, obj_type, title, is_key as i64, is_deleted as i64, updated_at],
        )
        .unwrap();
    }

    /// Вставляет тестовый материал
    fn insert_material(
        conn: &Connection,
        id: &str,
        case_id: &str,
        code: &str,
        title: &str,
        integrity_status: &str,
        is_deleted: bool,
        updated_at: &str,
    ) {
        conn.execute(
            "INSERT INTO materials (id, case_id, material_code, title, material_type, integrity_status, is_deleted, updated_at)
             VALUES (?1, ?2, ?3, ?4, ?5, ?6, ?7, ?8)",
            rusqlite::params![id, case_id, code, title, "document", integrity_status, is_deleted as i64, updated_at],
        )
        .unwrap();
    }

    /// Вставляет тестовую связь
    fn insert_relation(
        conn: &Connection,
        id: &str,
        case_id: &str,
        code: &str,
        rtype: &str,
        src_id: &str,
        tgt_id: &str,
        is_deleted: bool,
        updated_at: &str,
    ) {
        conn.execute(
            "INSERT INTO relations (id, case_id, relation_code, relation_type, source_object_id, target_object_id, is_deleted, updated_at)
             VALUES (?1, ?2, ?3, ?4, ?5, ?6, ?7, ?8)",
            rusqlite::params![id, case_id, code, rtype, src_id, tgt_id, is_deleted as i64, updated_at],
        )
        .unwrap();
    }

    /// Вставляет тестовое событие хронологии
    fn insert_event(
        conn: &Connection,
        id: &str,
        case_id: &str,
        code: &str,
        title: &str,
        include_in_report: bool,
        is_deleted: bool,
        updated_at: &str,
    ) {
        conn.execute(
            "INSERT INTO timeline_events (id, case_id, event_code, event_type, title, event_date, include_in_report, is_deleted, updated_at)
             VALUES (?1, ?2, ?3, ?4, ?5, ?6, ?7, ?8, ?9)",
            rusqlite::params![id, case_id, code, "fact", title, "2026-07-10", include_in_report as i64, is_deleted as i64, updated_at],
        )
        .unwrap();
    }

    // ──────────────────────────────────────────────────────────
    // Тесты
    // ──────────────────────────────────────────────────────────

    #[test]
    fn test_empty_case_all_counts_zero() {
        let conn = setup_db();
        let case_id = "case-empty-1";
        insert_case(&conn, case_id, "C-001", "Empty Case");

        assert_eq!(CaseSummaryRepository::count_objects(&conn, case_id).unwrap(), 0);
        assert_eq!(CaseSummaryRepository::count_key_objects(&conn, case_id).unwrap(), 0);
        assert_eq!(CaseSummaryRepository::count_materials(&conn, case_id).unwrap(), 0);
        assert_eq!(CaseSummaryRepository::count_materials_with_integrity_issues(&conn, case_id).unwrap(), 0);
        assert_eq!(CaseSummaryRepository::count_relations(&conn, case_id).unwrap(), 0);
        assert_eq!(CaseSummaryRepository::count_events(&conn, case_id).unwrap(), 0);
        assert_eq!(CaseSummaryRepository::count_report_events(&conn, case_id).unwrap(), 0);

        let max_ts = CaseSummaryRepository::get_max_updated_at(&conn, case_id).unwrap();
        assert!(max_ts.is_empty(), "max_updated_at should be empty for empty case");

        let key_objects = CaseSummaryRepository::get_key_objects(&conn, case_id, 10).unwrap();
        assert!(key_objects.is_empty(), "key_objects should be empty for empty case");

        let activity = CaseSummaryRepository::get_recent_activity(&conn, case_id, 10).unwrap();
        assert!(activity.is_empty(), "recent_activity should be empty for empty case");
    }

    #[test]
    fn test_counts_after_inserting_data() {
        let conn = setup_db();
        let case_id = "case-counts-1";
        insert_case(&conn, case_id, "C-002", "Counts Test Case");

        // Добавляем 3 объекта (1 ключевой, 2 обычных)
        insert_object(&conn, "obj-1", case_id, "OBJ-001", "person", "Alice", true, false, "2026-07-10T10:00:00Z");
        insert_object(&conn, "obj-2", case_id, "OBJ-002", "person", "Bob", false, false, "2026-07-10T10:01:00Z");
        insert_object(&conn, "obj-3", case_id, "OBJ-003", "place", "Warehouse", false, false, "2026-07-10T10:02:00Z");

        // Добавляем 2 материала (1 с проблемой целостности)
        insert_material(&conn, "mat-1", case_id, "MAT-001", "Report.pdf", "ok", false, "2026-07-10T11:00:00Z");
        insert_material(&conn, "mat-2", case_id, "MAT-002", "Photo.jpg", "mismatch", false, "2026-07-10T11:01:00Z");

        // Добавляем 1 связь
        insert_relation(&conn, "rel-1", case_id, "REL-001", "knows", "obj-1", "obj-2", false, "2026-07-10T12:00:00Z");

        // Добавляем 2 события (1 в справке, 1 нет)
        insert_event(&conn, "evt-1", case_id, "EVT-001", "Meeting", true, false, "2026-07-10T13:00:00Z");
        insert_event(&conn, "evt-2", case_id, "EVT-002", "Call", false, false, "2026-07-10T13:01:00Z");

        assert_eq!(CaseSummaryRepository::count_objects(&conn, case_id).unwrap(), 3);
        assert_eq!(CaseSummaryRepository::count_key_objects(&conn, case_id).unwrap(), 1);
        assert_eq!(CaseSummaryRepository::count_materials(&conn, case_id).unwrap(), 2);
        assert_eq!(CaseSummaryRepository::count_materials_with_integrity_issues(&conn, case_id).unwrap(), 1);
        assert_eq!(CaseSummaryRepository::count_relations(&conn, case_id).unwrap(), 1);
        assert_eq!(CaseSummaryRepository::count_events(&conn, case_id).unwrap(), 2);
        assert_eq!(CaseSummaryRepository::count_report_events(&conn, case_id).unwrap(), 1);

        let max_ts = CaseSummaryRepository::get_max_updated_at(&conn, case_id).unwrap();
        assert_eq!(max_ts, "2026-07-10T13:01:00Z", "max_updated_at should be the latest timestamp");
    }

    #[test]
    fn test_soft_deleted_are_excluded() {
        let conn = setup_db();
        let case_id = "case-softdel-1";
        insert_case(&conn, case_id, "C-003", "Soft Delete Test");

        // Активный объект
        insert_object(&conn, "obj-active", case_id, "OBJ-001", "person", "Active", true, false, "2026-07-10T10:00:00Z");
        // Удалённый объект
        insert_object(&conn, "obj-deleted", case_id, "OBJ-002", "person", "Deleted", false, true, "2026-07-10T10:01:00Z");

        // Активный материал
        insert_material(&conn, "mat-active", case_id, "MAT-001", "Active.pdf", "ok", false, "2026-07-10T11:00:00Z");
        // Удалённый материал с проблемой
        insert_material(&conn, "mat-deleted", case_id, "MAT-002", "Deleted.jpg", "mismatch", true, "2026-07-10T11:01:00Z");

        // Активная связь
        insert_relation(&conn, "rel-active", case_id, "REL-001", "knows", "obj-active", "obj-active", false, "2026-07-10T12:00:00Z");
        // Удалённая связь
        insert_relation(&conn, "rel-deleted", case_id, "REL-002", "knows", "obj-active", "obj-active", true, "2026-07-10T12:01:00Z");

        // Активное событие
        insert_event(&conn, "evt-active", case_id, "EVT-001", "Active Event", true, false, "2026-07-10T13:00:00Z");
        // Удалённое событие
        insert_event(&conn, "evt-deleted", case_id, "EVT-002", "Deleted Event", true, true, "2026-07-10T13:01:00Z");

        // Soft-deleted записи должны быть исключены из всех счётчиков
        assert_eq!(CaseSummaryRepository::count_objects(&conn, case_id).unwrap(), 1, "should exclude deleted object");
        assert_eq!(CaseSummaryRepository::count_key_objects(&conn, case_id).unwrap(), 1, "deleted key object should be excluded");
        assert_eq!(CaseSummaryRepository::count_materials(&conn, case_id).unwrap(), 1, "should exclude deleted material");
        assert_eq!(CaseSummaryRepository::count_materials_with_integrity_issues(&conn, case_id).unwrap(), 0, "deleted material with issue should be excluded");
        assert_eq!(CaseSummaryRepository::count_relations(&conn, case_id).unwrap(), 1, "should exclude deleted relation");
        assert_eq!(CaseSummaryRepository::count_events(&conn, case_id).unwrap(), 1, "should exclude deleted event");
        assert_eq!(CaseSummaryRepository::count_report_events(&conn, case_id).unwrap(), 1, "deleted report event should be excluded");
    }

    #[test]
    fn test_key_objects_query() {
        let conn = setup_db();
        let case_id = "case-keyobj-1";
        insert_case(&conn, case_id, "C-004", "Key Objects Test");

        // 3 ключевых + 1 обычный
        insert_object(&conn, "ko-1", case_id, "OBJ-001", "person", "Alice", true, false, "2026-07-10T10:00:00Z");
        insert_object(&conn, "ko-2", case_id, "OBJ-002", "person", "Bob", true, false, "2026-07-10T10:01:00Z");
        insert_object(&conn, "ko-3", case_id, "OBJ-003", "place", "Warehouse", true, false, "2026-07-10T10:02:00Z");
        insert_object(&conn, "normal-1", case_id, "OBJ-004", "event", "Party", false, false, "2026-07-10T10:03:00Z");

        let key_objects = CaseSummaryRepository::get_key_objects(&conn, case_id, 10).unwrap();
        assert_eq!(key_objects.len(), 3, "should return exactly 3 key objects");

        // Проверяем структуру
        let alice = key_objects.iter().find(|o| o.object_code == "OBJ-001").unwrap();
        assert_eq!(alice.title, "Alice");
        assert_eq!(alice.object_type, "person");
        assert!(alice.is_key);

        // Проверяем лимит
        let limited = CaseSummaryRepository::get_key_objects(&conn, case_id, 2).unwrap();
        assert_eq!(limited.len(), 2, "should respect limit=2");
    }

    #[test]
    fn test_recent_activity_query() {
        let conn = setup_db();
        let case_id = "case-activity-1";
        insert_case(&conn, case_id, "C-005", "Activity Test");

        // Создаём записи с разными updated_at
        insert_object(&conn, "act-obj", case_id, "OBJ-001", "person", "Alice", false, false, "2026-07-10T10:00:00Z");
        insert_material(&conn, "act-mat", case_id, "MAT-001", "Doc.pdf", "ok", false, "2026-07-10T11:00:00Z");
        insert_relation(&conn, "act-rel", case_id, "REL-001", "knows", "act-obj", "act-obj", false, "2026-07-10T12:00:00Z");

        let activity = CaseSummaryRepository::get_recent_activity(&conn, case_id, 10).unwrap();
        assert_eq!(activity.len(), 3, "should return 3 activity items");

        // Проверяем порядок: сначала самые новые
        assert_eq!(activity[0].code, "REL-001", "newest should be first");
        assert_eq!(activity[1].code, "MAT-001");
        assert_eq!(activity[2].code, "OBJ-001");

        // Проверяем структуру ActivityItemDto
        let obj_activity = activity.iter().find(|a| a.entity_type == "object").unwrap();
        assert_eq!(obj_activity.entity_id, "act-obj");
        assert_eq!(obj_activity.code, "OBJ-001");
        assert_eq!(obj_activity.title, "Alice");
        assert_eq!(obj_activity.timestamp, "2026-07-10T10:00:00Z");
        assert_eq!(obj_activity.action, "updated");

        // Проверяем лимит
        let limited = CaseSummaryRepository::get_recent_activity(&conn, case_id, 2).unwrap();
        assert_eq!(limited.len(), 2, "should respect limit=2");
    }

    #[test]
    fn test_different_case_isolation() {
        let conn = setup_db();

        // Два дела
        insert_case(&conn, "case-a", "C-A", "Case A");
        insert_case(&conn, "case-b", "C-B", "Case B");

        // Объекты только в Case A
        insert_object(&conn, "a-obj", "case-a", "OBJ-A1", "person", "Alice", true, false, "2026-07-10T10:00:00Z");
        // Объекты только в Case B
        insert_object(&conn, "b-obj", "case-b", "OBJ-B1", "person", "Bob", true, false, "2026-07-10T10:00:00Z");

        assert_eq!(CaseSummaryRepository::count_objects(&conn, "case-a").unwrap(), 1);
        assert_eq!(CaseSummaryRepository::count_objects(&conn, "case-b").unwrap(), 1);

        let key_a = CaseSummaryRepository::get_key_objects(&conn, "case-a", 10).unwrap();
        assert_eq!(key_a.len(), 1);
        assert_eq!(key_a[0].object_code, "OBJ-A1");

        let key_b = CaseSummaryRepository::get_key_objects(&conn, "case-b", 10).unwrap();
        assert_eq!(key_b.len(), 1);
        assert_eq!(key_b[0].object_code, "OBJ-B1");
    }
}

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
