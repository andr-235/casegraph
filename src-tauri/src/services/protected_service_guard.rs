use rusqlite::Connection;

use crate::errors::app_error::AppErrorDto;
use crate::repositories::user_access_repository::UserAccessRepository;
use crate::security::session::CurrentUserDto;

pub struct ProtectedServiceGuard;

impl ProtectedServiceGuard {
    pub fn require_password_change_resolved(
        conn: &Connection,
        current_user: &CurrentUserDto,
    ) -> Result<(), AppErrorDto> {
        let access = UserAccessRepository::get_user_access_flags(conn, &current_user.user_id)?;

        if !access.is_active {
            return Err(AppErrorDto::access_denied("Пользователь заблокирован"));
        }

        if access.must_change_password {
            return Err(AppErrorDto::password_change_required());
        }

        Ok(())
    }
}
