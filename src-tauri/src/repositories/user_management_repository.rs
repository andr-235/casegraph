use rusqlite::{params, Connection};

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
