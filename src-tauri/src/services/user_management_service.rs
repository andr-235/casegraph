use tauri::AppHandle;

use crate::db::connection::open_connection;
use crate::domain::user_management::{
    BlockUserPayload, BlockUserResponse, ChangeOwnPasswordPayload, ChangeOwnPasswordResponse,
    CreateUserPayload, CreateUserResponse, GetRolesResponse, GetUserByIdPayload,
    GetUserByIdResponse, GetUsersPayload, GetUsersResponse, ResetUserPasswordPayload,
    ResetUserPasswordResponse, UnblockUserPayload, UnblockUserResponse, UpdateUserPayload,
    UpdateUserResponse, UserListItemDto,
};
use crate::errors::app_error::AppErrorDto;
use crate::repositories::user_management_repository::UserManagementRepository;
use crate::security::password::{hash_password, verify_password};
use crate::security::session::{CurrentUserDto, SessionState};
use crate::services::protected_service_context::require_protected_administrator;
use crate::services::user_management_validation::{
    normalize_block_user_payload, normalize_change_own_password_payload,
    normalize_create_user_payload, normalize_reset_user_password_payload,
    normalize_unblock_user_payload, normalize_update_user_payload,
};

const DEFAULT_USERS_LIMIT: i64 = 50;
const MAX_USERS_LIMIT: i64 = 200;

pub struct UserManagementService;

impl UserManagementService {
    pub fn get_users(
        app: &AppHandle,
        session: &SessionState,
        payload: GetUsersPayload,
    ) -> Result<GetUsersResponse, AppErrorDto> {
        let context = require_protected_administrator(app, session)?;
        let conn = &context.conn;

        let limit = normalize_limit(payload.limit);
        let offset = payload.offset.unwrap_or(0).max(0);

        let query = payload.query;
        let role = payload.role;
        let status = payload.status;

        validate_role_filter(role.as_deref())?;
        validate_status_filter(status.as_deref())?;

        let users = UserManagementRepository::get_users(
            conn,
            query.clone(),
            role.clone(),
            status.clone(),
            limit,
            offset,
        )?;

        let total = UserManagementRepository::count_users(conn, query, role, status)?;

        Ok(GetUsersResponse { users, total })
    }

    pub fn create_user(
        app: &AppHandle,
        session: &SessionState,
        payload: CreateUserPayload,
    ) -> Result<CreateUserResponse, AppErrorDto> {
        let context = require_protected_administrator(app, session)?;
        let current_user = &context.current_user;
        let conn = &context.conn;
        let input = normalize_create_user_payload(payload)?;

        if UserManagementRepository::username_exists(conn, &input.username)? {
            return Err(AppErrorDto::validation(
                "Пользователь с таким логином уже существует",
            ));
        }

        let role_id = UserManagementRepository::get_role_id_by_code(conn, &input.role_code)?;
        let password_hash = hash_password(&input.password)?;

        let user_id = UserManagementRepository::create_user(
            conn,
            &input.username,
            input.display_name.as_deref(),
            &role_id,
            &password_hash,
            input.must_change_password,
        )?;

        let user = UserManagementRepository::get_user_by_id(conn, &user_id)?;

        write_user_created_audit_best_effort(app, current_user, &user);

        Ok(CreateUserResponse { user })
    }

    pub fn get_user_by_id(
        app: &AppHandle,
        session: &SessionState,
        payload: GetUserByIdPayload,
    ) -> Result<GetUserByIdResponse, AppErrorDto> {
        let context = require_protected_administrator(app, session)?;
        let conn = &context.conn;

        let user_id = payload.user_id.trim();

        if user_id.is_empty() {
            return Err(AppErrorDto::validation("Не указан пользователь"));
        }

        let user = UserManagementRepository::get_user_by_id(conn, user_id)?;

        Ok(GetUserByIdResponse { user })
    }

    pub fn update_user(
        app: &AppHandle,
        session: &SessionState,
        payload: UpdateUserPayload,
    ) -> Result<UpdateUserResponse, AppErrorDto> {
        let context = require_protected_administrator(app, session)?;
        let current_user = &context.current_user;
        let conn = &context.conn;
        let input = normalize_update_user_payload(payload)?;

        if !UserManagementRepository::user_exists(conn, &input.user_id)? {
            return Err(AppErrorDto::validation("Пользователь не найден"));
        }

        let old_user = UserManagementRepository::get_user_by_id(conn, &input.user_id)?;
        let old_role_code =
            UserManagementRepository::get_user_role_code_by_id(conn, &input.user_id)?;

        if old_role_code == "administrator" && input.role_code != "administrator" {
            let active_admin_count = UserManagementRepository::count_active_administrators(conn)?;

            if active_admin_count <= 1 {
                return Err(AppErrorDto::validation(
                    "Нельзя изменить роль последнего активного администратора",
                ));
            }
        }

        let role_id = UserManagementRepository::get_role_id_by_code(conn, &input.role_code)?;

        UserManagementRepository::update_user(
            conn,
            &input.user_id,
            input.display_name.as_deref(),
            &role_id,
            input.must_change_password,
        )?;

        let user = UserManagementRepository::get_user_by_id(conn, &input.user_id)?;

        write_user_updated_audit_best_effort(app, current_user, &old_user, &user);

        Ok(UpdateUserResponse { user })
    }

    pub fn block_user(
        app: &AppHandle,
        session: &SessionState,
        payload: BlockUserPayload,
    ) -> Result<BlockUserResponse, AppErrorDto> {
        let context = require_protected_administrator(app, session)?;
        let current_user = &context.current_user;
        let conn = &context.conn;
        let user_id = normalize_block_user_payload(payload)?;

        if user_id == current_user.user_id {
            return Err(AppErrorDto::validation(
                "Нельзя заблокировать собственную учётную запись",
            ));
        }

        if !UserManagementRepository::user_exists(conn, &user_id)? {
            return Err(AppErrorDto::validation("Пользователь не найден"));
        }

        let old_user = UserManagementRepository::get_user_by_id(conn, &user_id)?;

        if old_user.role_code == "administrator" && old_user.is_active {
            let active_admin_count = UserManagementRepository::count_active_administrators(conn)?;

            if active_admin_count <= 1 {
                return Err(AppErrorDto::validation(
                    "Нельзя заблокировать последнего активного администратора",
                ));
            }
        }

        UserManagementRepository::set_user_active(conn, &user_id, false)?;

        let user = UserManagementRepository::get_user_by_id(conn, &user_id)?;

        write_user_activity_audit_best_effort(app, current_user, "USER_BLOCKED", &old_user, &user);

        Ok(BlockUserResponse { user })
    }

    pub fn unblock_user(
        app: &AppHandle,
        session: &SessionState,
        payload: UnblockUserPayload,
    ) -> Result<UnblockUserResponse, AppErrorDto> {
        let context = require_protected_administrator(app, session)?;
        let current_user = &context.current_user;
        let conn = &context.conn;
        let user_id = normalize_unblock_user_payload(payload)?;

        if !UserManagementRepository::user_exists(conn, &user_id)? {
            return Err(AppErrorDto::validation("Пользователь не найден"));
        }

        let old_user = UserManagementRepository::get_user_by_id(conn, &user_id)?;

        UserManagementRepository::set_user_active(conn, &user_id, true)?;

        let user = UserManagementRepository::get_user_by_id(conn, &user_id)?;

        write_user_activity_audit_best_effort(
            app,
            current_user,
            "USER_UNBLOCKED",
            &old_user,
            &user,
        );

        Ok(UnblockUserResponse { user })
    }

    pub fn change_own_password(
        app: &AppHandle,
        session: &SessionState,
        payload: ChangeOwnPasswordPayload,
    ) -> Result<ChangeOwnPasswordResponse, AppErrorDto> {
        let current_user = session.require_current_user()?;
        let input = normalize_change_own_password_payload(payload)?;

        let conn = open_connection(app)?;

        let auth_user =
            UserManagementRepository::get_auth_user_by_id(&conn, &current_user.user_id)?;

        if !auth_user.is_active {
            return Err(AppErrorDto::validation("Пользователь заблокирован"));
        }

        let is_valid_current_password =
            verify_password(&input.current_password, &auth_user.password_hash)?;

        if !is_valid_current_password {
            return Err(AppErrorDto::validation("Текущий пароль указан неверно"));
        }

        let new_password_hash = hash_password(&input.new_password)?;

        UserManagementRepository::update_own_password_hash(
            &conn,
            &current_user.user_id,
            &new_password_hash,
        )?;

        let updated_dto = CurrentUserDto {
            user_id: auth_user.id,
            username: auth_user.username,
            display_name: auth_user.display_name,
            role: auth_user.role_code,
            is_active: auth_user.is_active,
            must_change_password: false,
        };

        session.set_current_user(updated_dto.clone());

        write_own_password_changed_audit_best_effort(app, &current_user);

        Ok(ChangeOwnPasswordResponse { user: updated_dto })
    }

    pub fn reset_user_password(
        app: &AppHandle,
        session: &SessionState,
        payload: ResetUserPasswordPayload,
    ) -> Result<ResetUserPasswordResponse, AppErrorDto> {
        let context = require_protected_administrator(app, session)?;
        let current_user = &context.current_user;
        let conn = &context.conn;
        let input = normalize_reset_user_password_payload(payload)?;

        if !UserManagementRepository::user_exists(conn, &input.user_id)? {
            return Err(AppErrorDto::validation("Пользователь не найден"));
        }

        let old_user = UserManagementRepository::get_user_by_id(conn, &input.user_id)?;

        let password_hash = hash_password(&input.temporary_password)?;

        UserManagementRepository::update_user_password_hash(conn, &input.user_id, &password_hash)?;

        let user = UserManagementRepository::get_user_by_id(conn, &input.user_id)?;

        write_user_password_reset_audit_best_effort(app, current_user, &old_user, &user);

        Ok(ResetUserPasswordResponse { user })
    }

    pub fn get_roles(
        app: &AppHandle,
        session: &SessionState,
    ) -> Result<GetRolesResponse, AppErrorDto> {
        let context = require_protected_administrator(app, session)?;
        let conn = &context.conn;
        let roles = UserManagementRepository::get_roles(conn)?;

        Ok(GetRolesResponse { roles })
    }
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

fn write_user_updated_audit_best_effort(
    app: &AppHandle,
    current_user: &CurrentUserDto,
    old_user: &UserListItemDto,
    new_user: &UserListItemDto,
) {
    use crate::services::audit_service::{to_json_value, AuditService, AuditSuccessInput};

    let old_value = to_json_value(&serde_json::json!({
        "id": old_user.id,
        "username": old_user.username,
        "displayName": old_user.display_name,
        "roleCode": old_user.role_code,
        "isActive": old_user.is_active,
        "mustChangePassword": old_user.must_change_password,
    }))
    .ok();

    let new_value = to_json_value(&serde_json::json!({
        "id": new_user.id,
        "username": new_user.username,
        "displayName": new_user.display_name,
        "roleCode": new_user.role_code,
        "isActive": new_user.is_active,
        "mustChangePassword": new_user.must_change_password,
    }))
    .ok();

    let input = AuditSuccessInput::new(
        current_user,
        "USER_UPDATED",
        "user",
        Some(&new_user.id),
        None,
        old_value,
        new_value,
        None,
    );

    AuditService::write_success_non_blocking(app.clone(), input);
}

fn write_user_activity_audit_best_effort(
    app: &AppHandle,
    current_user: &CurrentUserDto,
    action: &str,
    old_user: &UserListItemDto,
    new_user: &UserListItemDto,
) {
    use crate::services::audit_service::{to_json_value, AuditService, AuditSuccessInput};

    let old_value = to_json_value(&serde_json::json!({
        "id": old_user.id,
        "username": old_user.username,
        "displayName": old_user.display_name,
        "roleCode": old_user.role_code,
        "isActive": old_user.is_active,
        "mustChangePassword": old_user.must_change_password,
    }))
    .ok();

    let new_value = to_json_value(&serde_json::json!({
        "id": new_user.id,
        "username": new_user.username,
        "displayName": new_user.display_name,
        "roleCode": new_user.role_code,
        "isActive": new_user.is_active,
        "mustChangePassword": new_user.must_change_password,
    }))
    .ok();

    let input = AuditSuccessInput::new(
        current_user,
        action,
        "user",
        Some(&new_user.id),
        None,
        old_value,
        new_value,
        None,
    );

    AuditService::write_success_non_blocking(app.clone(), input);
}

fn write_own_password_changed_audit_best_effort(app: &AppHandle, current_user: &CurrentUserDto) {
    use crate::services::audit_service::AuditService;

    if let Err(error) = AuditService::write_success_str(
        app,
        &current_user.user_id,
        &current_user.username,
        &current_user.role,
        "USER_PASSWORD_CHANGED",
        "user",
        Some(&current_user.user_id),
        None,
        None,
        Some(r#"{"passwordChanged":true,"mustChangePassword":false}"#),
    ) {
        eprintln!(
            "Failed to write USER_PASSWORD_CHANGED audit event: {:?}",
            error
        );
    }
}

fn write_user_password_reset_audit_best_effort(
    app: &AppHandle,
    current_user: &CurrentUserDto,
    old_user: &UserListItemDto,
    new_user: &UserListItemDto,
) {
    use crate::services::audit_service::AuditService;

    let old_value = format!(
        r#"{{"id":"{}","username":"{}","mustChangePassword":{}}}"#,
        escape_json(&old_user.id),
        escape_json(&old_user.username),
        old_user.must_change_password,
    );

    let new_value = format!(
        r#"{{"id":"{}","username":"{}","mustChangePassword":{},"passwordReset":true}}"#,
        escape_json(&new_user.id),
        escape_json(&new_user.username),
        new_user.must_change_password,
    );

    if let Err(error) = AuditService::write_success_str(
        app,
        &current_user.user_id,
        &current_user.username,
        &current_user.role,
        "USER_PASSWORD_RESET",
        "user",
        Some(&new_user.id),
        None,
        Some(&old_value),
        Some(&new_value),
    ) {
        eprintln!(
            "Failed to write USER_PASSWORD_RESET audit event: {:?}",
            error
        );
    }
}

fn escape_json(value: &str) -> String {
    value
        .replace('\\', "\\\\")
        .replace('"', "\\\"")
        .replace('\n', "\\n")
        .replace('\r', "\\r")
        .replace('\t', "\\t")
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
