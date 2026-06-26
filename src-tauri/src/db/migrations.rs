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

        CREATE INDEX IF NOT EXISTS idx_cases_case_code ON cases(case_code);
        CREATE INDEX IF NOT EXISTS idx_cases_status ON cases(status);
        CREATE INDEX IF NOT EXISTS idx_cases_created_at ON cases(created_at);
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
