use rusqlite::{Connection, OptionalExtension};

use crate::errors::app_error::AppErrorDto;

pub fn apply_migrations(conn: &Connection) -> Result<(), AppErrorDto> {
    conn.execute_batch(
        r#"
        CREATE TABLE IF NOT EXISTS schema_migrations (
            id INTEGER PRIMARY KEY AUTOINCREMENT,
            version TEXT NOT NULL UNIQUE,
            applied_at TEXT NOT NULL DEFAULT CURRENT_TIMESTAMP
        );

        CREATE TABLE IF NOT EXISTS roles (
            id TEXT PRIMARY KEY,
            code TEXT NOT NULL UNIQUE,
            title TEXT NOT NULL,
            created_at TEXT NOT NULL DEFAULT CURRENT_TIMESTAMP
        );

        CREATE TABLE IF NOT EXISTS users (
            id TEXT PRIMARY KEY,
            username TEXT NOT NULL UNIQUE,
            display_name TEXT NOT NULL,
            password_hash TEXT NOT NULL,
            role_id TEXT NOT NULL,
            is_active INTEGER NOT NULL DEFAULT 1,
            must_change_password INTEGER NOT NULL DEFAULT 0,
            created_at TEXT NOT NULL DEFAULT CURRENT_TIMESTAMP,
            updated_at TEXT NOT NULL DEFAULT CURRENT_TIMESTAMP,
            FOREIGN KEY (role_id) REFERENCES roles(id)
        );

        CREATE TABLE IF NOT EXISTS app_settings (
            key TEXT PRIMARY KEY,
            value TEXT NOT NULL,
            value_type TEXT NOT NULL,
            category TEXT NOT NULL,
            description TEXT,
            is_system INTEGER NOT NULL DEFAULT 0,
            updated_at TEXT NOT NULL DEFAULT CURRENT_TIMESTAMP
        );

        CREATE TABLE IF NOT EXISTS audit_logs (
            id TEXT PRIMARY KEY,
            created_at TEXT NOT NULL DEFAULT CURRENT_TIMESTAMP,
            user_id TEXT,
            action TEXT NOT NULL,
            result TEXT NOT NULL,
            severity TEXT NOT NULL DEFAULT 'info',
            entity_type TEXT,
            entity_id TEXT,
            description TEXT NOT NULL,
            error_code TEXT
        );

        CREATE TABLE IF NOT EXISTS cases (
            id TEXT PRIMARY KEY,
            case_code TEXT NOT NULL UNIQUE,
            title TEXT NOT NULL,
            subject TEXT NOT NULL,
            description TEXT NOT NULL DEFAULT '',
            status TEXT NOT NULL DEFAULT 'draft',
            period_start TEXT,
            period_end TEXT,
            created_by_user_id TEXT NOT NULL,
            created_at TEXT NOT NULL DEFAULT CURRENT_TIMESTAMP,
            updated_at TEXT NOT NULL DEFAULT CURRENT_TIMESTAMP,
            archived_at TEXT,

            FOREIGN KEY (created_by_user_id) REFERENCES users(id)
        );

        CREATE TABLE IF NOT EXISTS materials (
            id TEXT PRIMARY KEY,
            case_id TEXT NOT NULL,
            material_code TEXT NOT NULL,
            title TEXT NOT NULL,
            material_type TEXT NOT NULL,
            source_name TEXT NOT NULL DEFAULT '',
            description TEXT NOT NULL DEFAULT '',
            captured_at TEXT,
            include_in_report INTEGER NOT NULL DEFAULT 1,

            original_file_name TEXT,
            original_path TEXT,
            stored_file_path TEXT,
            file_size INTEGER,
            mime_type TEXT,
            sha256 TEXT,
            integrity_status TEXT NOT NULL DEFAULT 'not_checked',

            created_by_user_id TEXT NOT NULL,
            created_at TEXT NOT NULL DEFAULT CURRENT_TIMESTAMP,
            updated_at TEXT NOT NULL DEFAULT CURRENT_TIMESTAMP,
            archived_at TEXT,

            FOREIGN KEY (case_id) REFERENCES cases(id),
            FOREIGN KEY (created_by_user_id) REFERENCES users(id),
            UNIQUE(case_id, material_code)
        );

        CREATE INDEX IF NOT EXISTS idx_materials_case_id ON materials(case_id);
        CREATE INDEX IF NOT EXISTS idx_materials_material_code ON materials(material_code);
        CREATE INDEX IF NOT EXISTS idx_materials_created_at ON materials(created_at);
        CREATE INDEX IF NOT EXISTS idx_materials_archived_at ON materials(archived_at);

        CREATE TABLE IF NOT EXISTS object_nodes (
            id TEXT PRIMARY KEY,
            case_id TEXT NOT NULL,
            object_code TEXT NOT NULL,
            object_type TEXT NOT NULL,
            title TEXT NOT NULL,
            value TEXT,
            description TEXT NOT NULL DEFAULT '',
            is_key INTEGER NOT NULL DEFAULT 0,
            confidence_note TEXT NOT NULL DEFAULT '',
            include_in_report INTEGER NOT NULL DEFAULT 1,

            created_by_user_id TEXT NOT NULL,
            created_at TEXT NOT NULL DEFAULT CURRENT_TIMESTAMP,
            updated_at TEXT NOT NULL DEFAULT CURRENT_TIMESTAMP,
            archived_at TEXT,

            FOREIGN KEY (case_id) REFERENCES cases(id),
            FOREIGN KEY (created_by_user_id) REFERENCES users(id),
            UNIQUE(case_id, object_code)
        );

        CREATE INDEX IF NOT EXISTS idx_object_nodes_case_id ON object_nodes(case_id);
        CREATE INDEX IF NOT EXISTS idx_object_nodes_object_code ON object_nodes(object_code);
        CREATE INDEX IF NOT EXISTS idx_object_nodes_object_type ON object_nodes(object_type);
        CREATE INDEX IF NOT EXISTS idx_object_nodes_title ON object_nodes(title);
        CREATE INDEX IF NOT EXISTS idx_object_nodes_is_key ON object_nodes(is_key);
        CREATE INDEX IF NOT EXISTS idx_object_nodes_include_in_report ON object_nodes(include_in_report);
        CREATE INDEX IF NOT EXISTS idx_object_nodes_archived_at ON object_nodes(archived_at);

        CREATE TABLE IF NOT EXISTS object_materials (
            id TEXT PRIMARY KEY,
            object_id TEXT NOT NULL,
            material_id TEXT NOT NULL,
            link_reason TEXT NOT NULL DEFAULT '',
            created_by_user_id TEXT NOT NULL,
            created_at TEXT NOT NULL DEFAULT CURRENT_TIMESTAMP,

            FOREIGN KEY (object_id) REFERENCES object_nodes(id),
            FOREIGN KEY (material_id) REFERENCES materials(id),
            FOREIGN KEY (created_by_user_id) REFERENCES users(id),
            UNIQUE(object_id, material_id)
        );

        CREATE INDEX IF NOT EXISTS idx_object_materials_object_id ON object_materials(object_id);
        CREATE INDEX IF NOT EXISTS idx_object_materials_material_id ON object_materials(material_id);

        CREATE INDEX IF NOT EXISTS idx_cases_case_code ON cases(case_code);
        CREATE INDEX IF NOT EXISTS idx_cases_status ON cases(status);
        CREATE INDEX IF NOT EXISTS idx_cases_created_at ON cases(created_at);

        CREATE TABLE IF NOT EXISTS relations (
            id TEXT PRIMARY KEY,
            case_id TEXT NOT NULL,
            relation_code TEXT NOT NULL,

            source_object_id TEXT NOT NULL,
            target_object_id TEXT NOT NULL,

            relation_type TEXT NOT NULL,
            title TEXT,
            basis TEXT NOT NULL,
            confidence_level TEXT NOT NULL,
            supporting_material_id TEXT,
            analyst_comment TEXT,
            include_in_report INTEGER NOT NULL DEFAULT 1,

            created_by_user_id TEXT NOT NULL,
            created_at TEXT NOT NULL DEFAULT CURRENT_TIMESTAMP,
            updated_at TEXT NOT NULL DEFAULT CURRENT_TIMESTAMP,
            archived_at TEXT,

            FOREIGN KEY (case_id) REFERENCES cases(id),
            FOREIGN KEY (source_object_id) REFERENCES object_nodes(id),
            FOREIGN KEY (target_object_id) REFERENCES object_nodes(id),
            FOREIGN KEY (supporting_material_id) REFERENCES materials(id),
            FOREIGN KEY (created_by_user_id) REFERENCES users(id),

            CHECK (source_object_id <> target_object_id),
            UNIQUE(case_id, relation_code)
        );

        CREATE INDEX IF NOT EXISTS idx_relations_case_id ON relations(case_id);
        CREATE INDEX IF NOT EXISTS idx_relations_relation_code ON relations(relation_code);
        CREATE INDEX IF NOT EXISTS idx_relations_source_object_id ON relations(source_object_id);
        CREATE INDEX IF NOT EXISTS idx_relations_target_object_id ON relations(target_object_id);
        CREATE INDEX IF NOT EXISTS idx_relations_relation_type ON relations(relation_type);
        CREATE INDEX IF NOT EXISTS idx_relations_confidence_level ON relations(confidence_level);
        CREATE INDEX IF NOT EXISTS idx_relations_supporting_material_id ON relations(supporting_material_id);
        CREATE INDEX IF NOT EXISTS idx_relations_include_in_report ON relations(include_in_report);
        CREATE INDEX IF NOT EXISTS idx_relations_archived_at ON relations(archived_at);

        CREATE TABLE IF NOT EXISTS events (
            id TEXT PRIMARY KEY,
            case_id TEXT NOT NULL,
            event_code TEXT NOT NULL,

            event_type TEXT NOT NULL DEFAULT 'fact',
            title TEXT NOT NULL,
            description TEXT NOT NULL DEFAULT '',

            event_date TEXT NOT NULL,
            event_time TEXT,
            date_precision TEXT NOT NULL DEFAULT 'day',

            period_start TEXT,
            period_end TEXT,

            source_note TEXT NOT NULL DEFAULT '',
            analyst_comment TEXT NOT NULL DEFAULT '',
            include_in_report INTEGER NOT NULL DEFAULT 1,

            created_by_user_id TEXT NOT NULL,
            created_at TEXT NOT NULL DEFAULT CURRENT_TIMESTAMP,
            updated_at TEXT NOT NULL DEFAULT CURRENT_TIMESTAMP,
            archived_at TEXT,

            FOREIGN KEY (case_id) REFERENCES cases(id),
            FOREIGN KEY (created_by_user_id) REFERENCES users(id),
            UNIQUE(case_id, event_code)
        );

        CREATE INDEX IF NOT EXISTS idx_events_case_id ON events(case_id);
        CREATE INDEX IF NOT EXISTS idx_events_event_code ON events(event_code);
        CREATE INDEX IF NOT EXISTS idx_events_event_type ON events(event_type);
        CREATE INDEX IF NOT EXISTS idx_events_event_date ON events(event_date);
        CREATE INDEX IF NOT EXISTS idx_events_include_in_report ON events(include_in_report);
        CREATE INDEX IF NOT EXISTS idx_events_archived_at ON events(archived_at);

        CREATE TABLE IF NOT EXISTS event_objects (
            id TEXT PRIMARY KEY,
            case_id TEXT NOT NULL,
            event_id TEXT NOT NULL,
            object_id TEXT NOT NULL,
            link_note TEXT NOT NULL DEFAULT '',
            created_by_user_id TEXT NOT NULL,
            created_at TEXT NOT NULL DEFAULT CURRENT_TIMESTAMP,

            FOREIGN KEY (case_id) REFERENCES cases(id),
            FOREIGN KEY (event_id) REFERENCES events(id),
            FOREIGN KEY (object_id) REFERENCES object_nodes(id),
            FOREIGN KEY (created_by_user_id) REFERENCES users(id),
            UNIQUE(event_id, object_id)
        );

        CREATE INDEX IF NOT EXISTS idx_event_objects_case_id ON event_objects(case_id);
        CREATE INDEX IF NOT EXISTS idx_event_objects_event_id ON event_objects(event_id);
        CREATE INDEX IF NOT EXISTS idx_event_objects_object_id ON event_objects(object_id);

        CREATE TABLE IF NOT EXISTS event_materials (
            id TEXT PRIMARY KEY,
            case_id TEXT NOT NULL,
            event_id TEXT NOT NULL,
            material_id TEXT NOT NULL,
            link_note TEXT NOT NULL DEFAULT '',
            created_by_user_id TEXT NOT NULL,
            created_at TEXT NOT NULL DEFAULT CURRENT_TIMESTAMP,

            FOREIGN KEY (case_id) REFERENCES cases(id),
            FOREIGN KEY (event_id) REFERENCES events(id),
            FOREIGN KEY (material_id) REFERENCES materials(id),
            FOREIGN KEY (created_by_user_id) REFERENCES users(id),
            UNIQUE(event_id, material_id)
        );

        CREATE INDEX IF NOT EXISTS idx_event_materials_case_id ON event_materials(case_id);
        CREATE INDEX IF NOT EXISTS idx_event_materials_event_id ON event_materials(event_id);
        CREATE INDEX IF NOT EXISTS idx_event_materials_material_id ON event_materials(material_id);
        "#,
    )
    .map_err(|err| AppErrorDto::database(err.to_string()))?;

    seed_roles(conn)?;

    Ok(())
}

fn seed_roles(conn: &Connection) -> Result<(), AppErrorDto> {
    let roles = [
        ("role-administrator", "administrator", "Администратор"),
        ("role-analyst", "analyst", "Аналитик"),
        ("role-viewer", "viewer", "Наблюдатель"),
    ];

    for (id, code, title) in roles {
        conn.execute(
            r#"
            INSERT INTO roles (id, code, title)
            VALUES (?1, ?2, ?3)
            ON CONFLICT(code) DO NOTHING
            "#,
            (id, code, title),
        )
        .map_err(|err| AppErrorDto::database(err.to_string()))?;
    }

    Ok(())
}

pub fn has_administrator(conn: &Connection) -> Result<bool, AppErrorDto> {
    let count: i64 = conn
        .query_row(
            r#"
            SELECT COUNT(*)
            FROM users u
            JOIN roles r ON r.id = u.role_id
            WHERE r.code = 'administrator'
              AND u.is_active = 1
            "#,
            [],
            |row| row.get(0),
        )
        .optional()
        .map_err(|err| AppErrorDto::database(err.to_string()))?
        .unwrap_or(0);

    Ok(count > 0)
}
