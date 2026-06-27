use tauri::AppHandle;

use crate::db::connection::open_connection;
use crate::domain::user_management::{
    CreateUserPayload, CreateUserResponse, GetRolesResponse, GetUsersPayload, GetUsersResponse,
    UserListItemDto,
};
use crate::errors::app_error::AppErrorDto;
use crate::repositories::user_management_repository::UserManagementRepository;
use crate::security::password::hash_password;
use crate::security::session::{CurrentUserDto, SessionState};
use crate::services::user_management_validation::normalize_create_user_payload;

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

    pub fn create_user(
        app: &AppHandle,
        session: &SessionState,
        payload: CreateUserPayload,
    ) -> Result<CreateUserResponse, AppErrorDto> {
        let current_user = require_user_management_admin(session)?;
        let input = normalize_create_user_payload(payload)?;

        let conn = open_connection(app)?;

        if UserManagementRepository::username_exists(&conn, &input.username)? {
            return Err(AppErrorDto::validation(
                "Пользователь с таким логином уже существует",
            ));
        }

        let role_id = UserManagementRepository::get_role_id_by_code(&conn, &input.role_code)?;
        let password_hash = hash_password(&input.password)?;

        let user_id = UserManagementRepository::create_user(
            &conn,
            &input.username,
            input.display_name.as_deref(),
            &role_id,
            &password_hash,
            input.must_change_password,
        )?;

        let user = UserManagementRepository::get_user_by_id(&conn, &user_id)?;

        write_user_created_audit_best_effort(app, &current_user, &user);

        Ok(CreateUserResponse { user })
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

fn write_user_created_audit_best_effort(
    app: &AppHandle,
    current_user: &CurrentUserDto,
    created_user: &UserListItemDto,
) {
    use crate::services::audit_service::{to_json_value, AuditService, AuditSuccessInput};

    let new_value = match to_json_value(&serde_json::json!({
        "id": created_user.id,
        "username": created_user.username,
        "roleCode": created_user.role_code,
    })) {
        Ok(value) => Some(value),
        Err(_) => None,
    };

    let input = AuditSuccessInput::new(
        current_user,
        "USER_CREATED",
        "user",
        Some(&created_user.id),
        None,
        None,
        new_value,
        None,
    );

    AuditService::write_success_non_blocking(app.clone(), input);
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
