use rusqlite::{params, Connection};
use uuid::Uuid;

use crate::domain::user_management::{RoleOptionDto, UserListItemDto};
use crate::errors::app_error::AppErrorDto;

pub struct UserManagementRepository;

impl UserManagementRepository {
    pub fn get_users(
        conn: &Connection,
        query: Option<String>,
        role: Option<String>,
        status: Option<String>,
        limit: i64,
        offset: i64,
    ) -> Result<Vec<UserListItemDto>, AppErrorDto> {
        let normalized_query = normalize_optional_string(query);
        let normalized_role = normalize_optional_string(role);
        let normalized_status = normalize_optional_string(status);

        let mut stmt = conn
            .prepare(
                r#"
                SELECT
                    u.id,
                    u.username,
                    u.display_name,
                    r.code AS role_code,
                    COALESCE(r.title, r.code) AS role_title,
                    u.is_active,
                    COALESCE(u.must_change_password, 0) AS must_change_password,
                    NULL AS last_login_at,
                    u.created_at,
                    u.updated_at
                FROM users u
                INNER JOIN roles r ON r.id = u.role_id
                WHERE
                    (
                        ?1 IS NULL
                        OR LOWER(u.username) LIKE '%' || LOWER(?1) || '%'
                        OR LOWER(COALESCE(u.display_name, '')) LIKE '%' || LOWER(?1) || '%'
                    )
                    AND (?2 IS NULL OR r.code = ?2)
                    AND (
                        ?3 IS NULL
                        OR ?3 = 'all'
                        OR (?3 = 'active' AND u.is_active = 1)
                        OR (?3 = 'blocked' AND u.is_active = 0)
                    )
                ORDER BY u.created_at DESC
                LIMIT ?4 OFFSET ?5
                "#,
            )
            .map_err(|err| AppErrorDto::database(err.to_string()))?;

        let rows = stmt
            .query_map(
                params![
                    normalized_query,
                    normalized_role,
                    normalized_status,
                    limit,
                    offset
                ],
                |row| {
                    Ok(UserListItemDto {
                        id: row.get(0)?,
                        username: row.get(1)?,
                        display_name: row.get(2)?,
                        role_code: row.get(3)?,
                        role_title: row.get(4)?,
                        is_active: row.get::<_, i64>(5)? == 1,
                        must_change_password: row.get::<_, i64>(6)? == 1,
                        last_login_at: row.get(7)?,
                        created_at: row.get(8)?,
                        updated_at: row.get(9)?,
                    })
                },
            )
            .map_err(|err| AppErrorDto::database(err.to_string()))?;

        let mut users = Vec::new();

        for row in rows {
            users.push(row.map_err(|err| AppErrorDto::database(err.to_string()))?);
        }

        Ok(users)
    }

    pub fn count_users(
        conn: &Connection,
        query: Option<String>,
        role: Option<String>,
        status: Option<String>,
    ) -> Result<i64, AppErrorDto> {
        let normalized_query = normalize_optional_string(query);
        let normalized_role = normalize_optional_string(role);
        let normalized_status = normalize_optional_string(status);

        conn.query_row(
            r#"
            SELECT COUNT(*)
            FROM users u
            INNER JOIN roles r ON r.id = u.role_id
            WHERE
                (
                    ?1 IS NULL
                    OR LOWER(u.username) LIKE '%' || LOWER(?1) || '%'
                    OR LOWER(COALESCE(u.display_name, '')) LIKE '%' || LOWER(?1) || '%'
                )
                AND (?2 IS NULL OR r.code = ?2)
                AND (
                    ?3 IS NULL
                    OR ?3 = 'all'
                    OR (?3 = 'active' AND u.is_active = 1)
                    OR (?3 = 'blocked' AND u.is_active = 0)
                )
            "#,
            params![normalized_query, normalized_role, normalized_status],
            |row| row.get(0),
        )
        .map_err(|err| AppErrorDto::database(err.to_string()))
    }

    pub fn username_exists(conn: &Connection, username: &str) -> Result<bool, AppErrorDto> {
        let count: i64 = conn
            .query_row(
                "SELECT COUNT(*) FROM users WHERE LOWER(username) = LOWER(?1)",
                params![username],
                |row| row.get(0),
            )
            .map_err(|err| AppErrorDto::database(err.to_string()))?;

        Ok(count > 0)
    }

    pub fn user_exists(conn: &Connection, user_id: &str) -> Result<bool, AppErrorDto> {
        let count: i64 = conn
            .query_row(
                "SELECT COUNT(*) FROM users WHERE id = ?1",
                params![user_id],
                |row| row.get(0),
            )
            .map_err(|err| AppErrorDto::database(err.to_string()))?;

        Ok(count > 0)
    }

    pub fn get_user_role_code_by_id(
        conn: &Connection,
        user_id: &str,
    ) -> Result<String, AppErrorDto> {
        conn.query_row(
            r#"
            SELECT r.code
            FROM users u
            INNER JOIN roles r ON r.id = u.role_id
            WHERE u.id = ?1
            LIMIT 1
            "#,
            params![user_id],
            |row| row.get(0),
        )
        .map_err(|err| AppErrorDto::database(err.to_string()))
    }

    pub fn count_active_administrators(conn: &Connection) -> Result<i64, AppErrorDto> {
        conn.query_row(
            r#"
            SELECT COUNT(*)
            FROM users u
            INNER JOIN roles r ON r.id = u.role_id
            WHERE r.code = 'administrator'
              AND u.is_active = 1
            "#,
            [],
            |row| row.get(0),
        )
        .map_err(|err| AppErrorDto::database(err.to_string()))
    }

    pub fn update_user(
        conn: &Connection,
        user_id: &str,
        display_name: Option<&str>,
        role_id: &str,
        must_change_password: bool,
    ) -> Result<(), AppErrorDto> {
        conn.execute(
            r#"
            UPDATE users
            SET
                display_name = ?2,
                role_id = ?3,
                must_change_password = ?4,
                updated_at = CURRENT_TIMESTAMP
            WHERE id = ?1
            "#,
            params![
                user_id,
                display_name,
                role_id,
                if must_change_password { 1 } else { 0 },
            ],
        )
        .map_err(|err| AppErrorDto::database(err.to_string()))?;

        Ok(())
    }

    pub fn get_role_id_by_code(conn: &Connection, role_code: &str) -> Result<String, AppErrorDto> {
        conn.query_row(
            "SELECT id FROM roles WHERE code = ?1 LIMIT 1",
            params![role_code],
            |row| row.get(0),
        )
        .map_err(|err| AppErrorDto::database(err.to_string()))
    }

    pub fn create_user(
        conn: &Connection,
        username: &str,
        display_name: Option<&str>,
        role_id: &str,
        password_hash: &str,
        must_change_password: bool,
    ) -> Result<String, AppErrorDto> {
        let user_id = Uuid::new_v4().to_string();

        conn.execute(
            r#"
            INSERT INTO users (
                id,
                username,
                display_name,
                role_id,
                password_hash,
                is_active,
                must_change_password,
                created_at,
                updated_at
            )
            VALUES (?1, ?2, ?3, ?4, ?5, 1, ?6, CURRENT_TIMESTAMP, CURRENT_TIMESTAMP)
            "#,
            params![
                user_id,
                username,
                display_name,
                role_id,
                password_hash,
                if must_change_password { 1 } else { 0 },
            ],
        )
        .map_err(|err| AppErrorDto::database(err.to_string()))?;

        Ok(user_id)
    }

    pub fn get_user_by_id(
        conn: &Connection,
        user_id: &str,
    ) -> Result<UserListItemDto, AppErrorDto> {
        conn.query_row(
            r#"
            SELECT
                u.id,
                u.username,
                u.display_name,
                r.code AS role_code,
                COALESCE(r.title, r.code) AS role_title,
                u.is_active,
                COALESCE(u.must_change_password, 0) AS must_change_password,
                NULL AS last_login_at,
                u.created_at,
                u.updated_at
            FROM users u
            INNER JOIN roles r ON r.id = u.role_id
            WHERE u.id = ?1
            LIMIT 1
            "#,
            params![user_id],
            |row| {
                Ok(UserListItemDto {
                    id: row.get(0)?,
                    username: row.get(1)?,
                    display_name: row.get(2)?,
                    role_code: row.get(3)?,
                    role_title: row.get(4)?,
                    is_active: row.get::<_, i64>(5)? == 1,
                    must_change_password: row.get::<_, i64>(6)? == 1,
                    last_login_at: row.get(7)?,
                    created_at: row.get(8)?,
                    updated_at: row.get(9)?,
                })
            },
        )
        .map_err(|err| AppErrorDto::database(err.to_string()))
    }

    pub fn get_roles(conn: &Connection) -> Result<Vec<RoleOptionDto>, AppErrorDto> {
        let mut stmt = conn
            .prepare(
                r#"
                SELECT
                    id,
                    code AS role_code,
                    COALESCE(title, code) AS title
                FROM roles
                ORDER BY
                    CASE code
                        WHEN 'administrator' THEN 1
                        WHEN 'analyst' THEN 2
                        WHEN 'viewer' THEN 3
                        ELSE 99
                    END
                "#,
            )
            .map_err(|err| AppErrorDto::database(err.to_string()))?;

        let rows = stmt
            .query_map([], |row| {
                Ok(RoleOptionDto {
                    id: row.get(0)?,
                    role_code: row.get(1)?,
                    title: row.get(2)?,
                })
            })
            .map_err(|err| AppErrorDto::database(err.to_string()))?;

        let mut roles = Vec::new();

        for row in rows {
            roles.push(row.map_err(|err| AppErrorDto::database(err.to_string()))?);
        }

        Ok(roles)
    }
}

fn normalize_optional_string(value: Option<String>) -> Option<String> {
    value
        .map(|item| item.trim().to_string())
        .filter(|item| !item.is_empty())
}
