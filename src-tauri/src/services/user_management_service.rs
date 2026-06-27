use tauri::AppHandle;

use crate::db::connection::open_connection;
use crate::domain::user_management::{GetRolesResponse, GetUsersPayload, GetUsersResponse};
use crate::errors::app_error::AppErrorDto;
use crate::repositories::user_management_repository::UserManagementRepository;
use crate::security::session::{CurrentUserDto, SessionState};

const DEFAULT_USERS_LIMIT: i64 = 50;
const MAX_USERS_LIMIT: i64 = 200;

pub struct UserManagementService;

impl UserManagementService {
    pub fn get_users(
        app: &AppHandle,
        session: &SessionState,
        payload: GetUsersPayload,
    ) -> Result<GetUsersResponse, AppErrorDto> {
        require_user_management_admin(session)?;

        let limit = normalize_limit(payload.limit);
        let offset = payload.offset.unwrap_or(0).max(0);

        let query = payload.query;
        let role = payload.role;
        let status = payload.status;

        validate_role_filter(role.as_deref())?;
        validate_status_filter(status.as_deref())?;

        let conn = open_connection(app)?;

        let users = UserManagementRepository::get_users(
            &conn,
            query.clone(),
            role.clone(),
            status.clone(),
            limit,
            offset,
        )?;

        let total = UserManagementRepository::count_users(&conn, query, role, status)?;

        Ok(GetUsersResponse { users, total })
    }

    pub fn get_roles(
        app: &AppHandle,
        session: &SessionState,
    ) -> Result<GetRolesResponse, AppErrorDto> {
        require_user_management_admin(session)?;

        let conn = open_connection(app)?;
        let roles = UserManagementRepository::get_roles(&conn)?;

        Ok(GetRolesResponse { roles })
    }
}

fn require_user_management_admin(session: &SessionState) -> Result<CurrentUserDto, AppErrorDto> {
    let current_user = session
        .get_current_user()
        .ok_or_else(|| AppErrorDto::unauthorized("Требуется аутентификация"))?;

    if current_user.role != "administrator" {
        return Err(AppErrorDto::access_denied(
            "Доступ запрещён. Требуется роль администратора.",
        ));
    }

    Ok(current_user)
}

fn normalize_limit(limit: Option<i64>) -> i64 {
    limit
        .unwrap_or(DEFAULT_USERS_LIMIT)
        .clamp(1, MAX_USERS_LIMIT)
}

fn validate_role_filter(role: Option<&str>) -> Result<(), AppErrorDto> {
    if let Some(role) = role {
        let is_valid = matches!(role, "administrator" | "analyst" | "viewer");

        if !is_valid {
            return Err(AppErrorDto::validation("Некорректный фильтр роли"));
        }
    }

    Ok(())
}

fn validate_status_filter(status: Option<&str>) -> Result<(), AppErrorDto> {
    if let Some(status) = status {
        let is_valid = matches!(status, "all" | "active" | "blocked");

        if !is_valid {
            return Err(AppErrorDto::validation(
                "Некорректный фильтр статуса пользователя",
            ));
        }
    }

    Ok(())
}
