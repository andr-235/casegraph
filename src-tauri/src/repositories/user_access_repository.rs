use rusqlite::{params, Connection, OptionalExtension};

use crate::errors::app_error::AppErrorDto;

#[derive(Debug, Clone)]
pub struct UserAccessFlags {
    pub id: String,
    pub username: String,
    pub role_code: String,
    pub is_active: bool,
    pub must_change_password: bool,
}

pub struct UserAccessRepository;

impl UserAccessRepository {
    pub fn get_user_access_flags(
        conn: &Connection,
        user_id: &str,
    ) -> Result<UserAccessFlags, AppErrorDto> {
        let user = conn
            .query_row(
                r#"
                SELECT
                    u.id,
                    u.username,
                    r.code AS role_code,
                    u.is_active,
                    u.must_change_password
                FROM users u
                JOIN roles r ON r.id = u.role_id
                WHERE u.id = ?1
                LIMIT 1
                "#,
                params![user_id],
                |row| {
                    Ok(UserAccessFlags {
                        id: row.get(0)?,
                        username: row.get(1)?,
                        role_code: row.get(2)?,
                        is_active: row.get::<_, i64>(3)? == 1,
                        must_change_password: row.get::<_, i64>(4)? == 1,
                    })
                },
            )
            .optional()
            .map_err(|err| AppErrorDto::database(err.to_string()))?;

        user.ok_or_else(|| AppErrorDto::access_denied("Пользователь не найден"))
    }
}
