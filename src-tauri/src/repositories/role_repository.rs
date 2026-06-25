use rusqlite::{params, Connection};

use crate::errors::app_error::AppErrorDto;

pub struct RoleRepository;

impl RoleRepository {
    pub fn get_role_id_by_code(conn: &Connection, code: &str) -> Result<String, AppErrorDto> {
        conn.query_row(
            "SELECT id FROM roles WHERE code = ?1 LIMIT 1",
            params![code],
            |row| row.get(0),
        )
        .map_err(|err| AppErrorDto::database(err.to_string()))
    }
}
