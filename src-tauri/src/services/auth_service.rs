use serde::{Deserialize, Serialize};
use tauri::AppHandle;
use uuid::Uuid;

use crate::db::connection::open_connection;
use crate::errors::app_error::AppErrorDto;
use crate::repositories::role_repository::RoleRepository;
use crate::repositories::user_repository::{CreateUserRecord, UserRepository};
use crate::security::password::{hash_password, verify_password};
use crate::security::session::{CurrentUserDto, SessionState};

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CreateFirstAdminPayload {
    pub username: String,
    pub display_name: String,
    pub password: String,
}

#[derive(Debug, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct CreateFirstAdminResponse {
    pub user_id: String,
    pub username: String,
    pub role: String,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct LoginPayload {
    pub username: String,
    pub password: String,
}

#[derive(Debug, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct LoginResponse {
    pub user: CurrentUserDto,
}

pub struct AuthService;

impl AuthService {
    pub fn create_first_admin(
        app: &AppHandle,
        payload: CreateFirstAdminPayload,
    ) -> Result<CreateFirstAdminResponse, AppErrorDto> {
        let username = payload.username.trim().to_lowercase();
        let display_name = payload.display_name.trim().to_string();

        validate_username(&username)?;
        validate_display_name(&display_name)?;
        validate_password(&payload.password)?;

        let conn = open_connection(app)?;

        if UserRepository::has_active_administrator(&conn)? {
            return Err(AppErrorDto::new(
                "ERR_ADMIN_ALREADY_EXISTS",
                "Первый администратор уже создан.",
                None,
            ));
        }

        let role_id = RoleRepository::get_role_id_by_code(&conn, "administrator")?;
        let user_id = Uuid::new_v4().to_string();
        let password_hash = hash_password(&payload.password)?;

        UserRepository::create_user(
            &conn,
            CreateUserRecord {
                user_id: user_id.clone(),
                username: username.clone(),
                display_name,
                password_hash,
                role_id,
            },
        )?;

        Ok(CreateFirstAdminResponse {
            user_id,
            username,
            role: "administrator".to_string(),
        })
    }

    pub fn login(
        app: &AppHandle,
        session: &SessionState,
        payload: LoginPayload,
    ) -> Result<LoginResponse, AppErrorDto> {
        let username = payload.username.trim().to_lowercase();

        if username.is_empty() || payload.password.is_empty() {
            return Err(AppErrorDto::new(
                "ERR_VALIDATION",
                "Введите логин и пароль.",
                None,
            ));
        }

        let conn = open_connection(app)?;

        let Some(user_row) = UserRepository::find_auth_row_by_username(&conn, &username)? else {
            return Err(invalid_credentials_error());
        };

        if user_row.is_active != 1 {
            return Err(AppErrorDto::new(
                "ERR_USER_BLOCKED",
                "Пользователь заблокирован.",
                None,
            ));
        }

        let password_ok = verify_password(&payload.password, &user_row.password_hash)?;

        if !password_ok {
            return Err(invalid_credentials_error());
        }

        let user = CurrentUserDto {
            user_id: user_row.user_id,
            username: user_row.username,
            display_name: user_row.display_name,
            role: user_row.role,
            is_active: user_row.is_active == 1,
            must_change_password: user_row.must_change_password,
        };

        session.set_current_user(user.clone());

        Ok(LoginResponse { user })
    }

    pub fn get_current_user(session: &SessionState) -> Option<CurrentUserDto> {
        session.get_current_user()
    }

    pub fn logout(session: &SessionState) -> bool {
        session.clear_current_user();
        true
    }
}

fn validate_username(username: &str) -> Result<(), AppErrorDto> {
    if username.len() < 3 {
        return Err(AppErrorDto::new(
            "ERR_VALIDATION",
            "Логин должен содержать минимум 3 символа.",
            None,
        ));
    }

    Ok(())
}

fn validate_display_name(display_name: &str) -> Result<(), AppErrorDto> {
    if display_name.len() < 2 {
        return Err(AppErrorDto::new(
            "ERR_VALIDATION",
            "Имя пользователя должно содержать минимум 2 символа.",
            None,
        ));
    }

    Ok(())
}

fn validate_password(password: &str) -> Result<(), AppErrorDto> {
    if password.len() < 8 {
        return Err(AppErrorDto::new(
            "ERR_VALIDATION",
            "Пароль должен содержать минимум 8 символов.",
            None,
        ));
    }

    Ok(())
}

fn invalid_credentials_error() -> AppErrorDto {
    AppErrorDto::new(
        "ERR_INVALID_CREDENTIALS",
        "Неверный логин или пароль.",
        None,
    )
}
