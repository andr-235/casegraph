use rusqlite::{params, Connection, OptionalExtension};

use crate::errors::app_error::AppErrorDto;

#[derive(Debug)]
pub struct UserAuthRow {
    pub user_id: String,
    pub username: String,
    pub display_name: String,
    pub password_hash: String,
    pub is_active: i64,
    pub role: String,
    pub must_change_password: bool,
}

#[derive(Debug)]
pub struct CreateUserRecord {
    pub user_id: String,
    pub username: String,
    pub display_name: String,
    pub password_hash: String,
    pub role_id: String,
}

pub struct UserRepository;

impl UserRepository {
    pub fn has_active_administrator(conn: &Connection) -> Result<bool, AppErrorDto> {
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
            .map_err(|err| AppErrorDto::database(err.to_string()))?;

        Ok(count > 0)
    }

    pub fn create_user(conn: &Connection, record: CreateUserRecord) -> Result<(), AppErrorDto> {
        conn.execute(
            r#"
            INSERT INTO users (
                id,
                username,
                display_name,
                password_hash,
                role_id,
                is_active,
                must_change_password
            )
            VALUES (?1, ?2, ?3, ?4, ?5, 1, 0)
            "#,
            params![
                record.user_id,
                record.username,
                record.display_name,
                record.password_hash,
                record.role_id
            ],
        )
        .map_err(|err| AppErrorDto::database(err.to_string()))?;

        Ok(())
    }

    pub fn find_auth_row_by_username(
        conn: &Connection,
        username: &str,
    ) -> Result<Option<UserAuthRow>, AppErrorDto> {
        conn.query_row(
            r#"
            SELECT
                u.id,
                u.username,
                u.display_name,
                u.password_hash,
                u.is_active,
                r.code,
                COALESCE(u.must_change_password, 0) AS must_change_password
            FROM users u
            JOIN roles r ON r.id = u.role_id
            WHERE u.username = ?1
            LIMIT 1
            "#,
            params![username],
            |row| {
                Ok(UserAuthRow {
                    user_id: row.get(0)?,
                    username: row.get(1)?,
                    display_name: row.get(2)?,
                    password_hash: row.get(3)?,
                    is_active: row.get(4)?,
                    role: row.get(5)?,
                    must_change_password: row.get::<_, i64>(6)? == 1,
                })
            },
        )
        .optional()
        .map_err(|err| AppErrorDto::database(err.to_string()))
    }
}
