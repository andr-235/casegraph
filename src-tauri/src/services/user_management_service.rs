use tauri::AppHandle;

use crate::db::connection::open_connection;
use crate::domain::audit_action;
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
use crate::services::protected_service_context::require_protected_administrator_for;
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
        let context = require_protected_administrator_for(app, session, "GET_USERS")?;
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
        let context = require_protected_administrator_for(app, session, "CREATE_USER")?;
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
        let context = require_protected_administrator_for(app, session, "GET_USER_BY_ID")?;
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
        let context = require_protected_administrator_for(app, session, "UPDATE_USER")?;
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
        let context = require_protected_administrator_for(app, session, "BLOCK_USER")?;
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

        write_user_activity_audit_best_effort(
            app,
            current_user,
            audit_action::user::BLOCKED,
            &old_user,
            &user,
        );

        Ok(BlockUserResponse { user })
    }

    pub fn unblock_user(
        app: &AppHandle,
        session: &SessionState,
        payload: UnblockUserPayload,
    ) -> Result<UnblockUserResponse, AppErrorDto> {
        let context = require_protected_administrator_for(app, session, "UNBLOCK_USER")?;
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
            audit_action::user::UNBLOCKED,
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

        // Get old user before password update
        let old_user = UserManagementRepository::get_user_by_id(&conn, &current_user.user_id)?;

        let new_password_hash = hash_password(&input.new_password)?;

        UserManagementRepository::update_own_password_hash(
            &conn,
            &current_user.user_id,
            &new_password_hash,
        )?;

        let updated_user = UserManagementRepository::get_user_by_id(&conn, &current_user.user_id)?;

        let updated_dto = CurrentUserDto {
            user_id: auth_user.id,
            username: auth_user.username,
            display_name: auth_user.display_name,
            role: auth_user.role_code,
            is_active: auth_user.is_active,
            must_change_password: false,
        };

        session.set_current_user(updated_dto.clone());

        write_own_password_changed_audit_best_effort(app, &current_user, &old_user, &updated_user);

        Ok(ChangeOwnPasswordResponse { user: updated_dto })
    }

    pub fn reset_user_password(
        app: &AppHandle,
        session: &SessionState,
        payload: ResetUserPasswordPayload,
    ) -> Result<ResetUserPasswordResponse, AppErrorDto> {
        let context = require_protected_administrator_for(app, session, "RESET_USER_PASSWORD")?;
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
        let context = require_protected_administrator_for(app, session, "GET_ROLES")?;
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
    use crate::audit::audit_metadata;
    use crate::services::audit_service::{AuditService, AuditWriteInput};

    let result = (|| {
        let technical_details = audit_metadata::user_created(
            &created_user.id,
            &created_user.username,
            &created_user.role_code,
        )?;

        let new_value = audit_metadata::safe_user_snapshot(
            &created_user.username,
            created_user.display_name.as_deref().unwrap_or(""),
            &created_user.role_code,
            created_user.is_active,
            created_user.must_change_password,
        )?;

        AuditService::write_best_effort(
            app,
            AuditWriteInput::success(current_user, audit_action::user::CREATED)
                .with_entity("user", created_user.id.clone())
                .with_snapshots(None, Some(new_value))
                .with_details(technical_details),
        );
        Ok::<(), crate::errors::app_error::AppErrorDto>(())
    })();

    if let Err(err) = result {
        eprintln!("[audit] write_user_created_audit failed: {}", err.message);
    }
}

fn write_user_updated_audit_best_effort(
    app: &AppHandle,
    current_user: &CurrentUserDto,
    old_user: &UserListItemDto,
    new_user: &UserListItemDto,
) {
    use crate::audit::audit_metadata;
    use crate::services::audit_service::{AuditService, AuditWriteInput};

    let result = (|| {
        let mut changed = Vec::new();
        audit_metadata::push_changed(
            &mut changed,
            "username",
            &old_user.username,
            &new_user.username,
        );
        audit_metadata::push_changed(
            &mut changed,
            "displayName",
            &old_user.display_name,
            &new_user.display_name,
        );
        audit_metadata::push_changed(
            &mut changed,
            "roleCode",
            &old_user.role_code,
            &new_user.role_code,
        );
        audit_metadata::push_changed(
            &mut changed,
            "isActive",
            &old_user.is_active,
            &new_user.is_active,
        );
        audit_metadata::push_changed(
            &mut changed,
            "mustChangePassword",
            &old_user.must_change_password,
            &new_user.must_change_password,
        );

        let technical_details =
            audit_metadata::user_updated(&new_user.id, &new_user.username, &changed)?;

        let old_val = audit_metadata::safe_user_snapshot(
            &old_user.username,
            old_user.display_name.as_deref().unwrap_or(""),
            &old_user.role_code,
            old_user.is_active,
            old_user.must_change_password,
        )?;

        let new_val = audit_metadata::safe_user_snapshot(
            &new_user.username,
            new_user.display_name.as_deref().unwrap_or(""),
            &new_user.role_code,
            new_user.is_active,
            new_user.must_change_password,
        )?;

        AuditService::write_best_effort(
            app,
            AuditWriteInput::success(current_user, audit_action::user::UPDATED)
                .with_entity("user", new_user.id.clone())
                .with_snapshots(Some(old_val), Some(new_val))
                .with_details(technical_details),
        );
        Ok::<(), crate::errors::app_error::AppErrorDto>(())
    })();

    if let Err(err) = result {
        eprintln!("[audit] write_user_updated_audit failed: {}", err.message);
    }
}

fn write_user_activity_audit_best_effort(
    app: &AppHandle,
    current_user: &CurrentUserDto,
    action: &str,
    old_user: &UserListItemDto,
    new_user: &UserListItemDto,
) {
    use crate::audit::audit_metadata;
    use crate::services::audit_service::{AuditService, AuditWriteInput};

    let result = (|| {
        let technical_details = if action == audit_action::user::BLOCKED {
            audit_metadata::user_blocked(&new_user.id, &new_user.username)?
        } else {
            audit_metadata::user_unblocked(&new_user.id, &new_user.username)?
        };

        let old_val = audit_metadata::safe_user_snapshot(
            &old_user.username,
            old_user.display_name.as_deref().unwrap_or(""),
            &old_user.role_code,
            old_user.is_active,
            old_user.must_change_password,
        )?;

        let new_val = audit_metadata::safe_user_snapshot(
            &new_user.username,
            new_user.display_name.as_deref().unwrap_or(""),
            &new_user.role_code,
            new_user.is_active,
            new_user.must_change_password,
        )?;

        AuditService::write_best_effort(
            app,
            AuditWriteInput::success(current_user, action)
                .with_entity("user", new_user.id.clone())
                .with_snapshots(Some(old_val), Some(new_val))
                .with_details(technical_details),
        );
        Ok::<(), crate::errors::app_error::AppErrorDto>(())
    })();

    if let Err(err) = result {
        eprintln!("[audit] write_user_activity_audit failed: {}", err.message);
    }
}

fn write_own_password_changed_audit_best_effort(
    app: &AppHandle,
    current_user: &CurrentUserDto,
    old_user: &UserListItemDto,
    new_user: &UserListItemDto,
) {
    use crate::audit::audit_metadata;
    use crate::services::audit_service::{AuditService, AuditWriteInput};

    let result = (|| {
        let technical_details =
            audit_metadata::user_password_changed(&current_user.user_id, &current_user.username)?;

        let old_val = audit_metadata::safe_user_snapshot(
            &old_user.username,
            old_user.display_name.as_deref().unwrap_or(""),
            &old_user.role_code,
            old_user.is_active,
            old_user.must_change_password,
        )?;

        let new_val = audit_metadata::safe_user_snapshot(
            &new_user.username,
            new_user.display_name.as_deref().unwrap_or(""),
            &new_user.role_code,
            new_user.is_active,
            new_user.must_change_password,
        )?;

        AuditService::write_best_effort(
            app,
            AuditWriteInput::success(current_user, audit_action::user::PASSWORD_CHANGED)
                .with_entity("user", current_user.user_id.clone())
                .with_snapshots(Some(old_val), Some(new_val))
                .with_details(technical_details),
        );
        Ok::<(), crate::errors::app_error::AppErrorDto>(())
    })();

    if let Err(err) = result {
        eprintln!(
            "[audit] write_own_password_changed_audit failed: {}",
            err.message
        );
    }
}

fn write_user_password_reset_audit_best_effort(
    app: &AppHandle,
    current_user: &CurrentUserDto,
    old_user: &UserListItemDto,
    new_user: &UserListItemDto,
) {
    use crate::audit::audit_metadata;
    use crate::services::audit_service::{AuditService, AuditWriteInput};

    let result = (|| {
        let technical_details = audit_metadata::user_password_reset(
            &new_user.id,
            &new_user.username,
            new_user.must_change_password,
        )?;

        let old_val = audit_metadata::safe_user_snapshot(
            &old_user.username,
            old_user.display_name.as_deref().unwrap_or(""),
            &old_user.role_code,
            old_user.is_active,
            old_user.must_change_password,
        )?;

        let new_val = audit_metadata::safe_user_snapshot(
            &new_user.username,
            new_user.display_name.as_deref().unwrap_or(""),
            &new_user.role_code,
            new_user.is_active,
            new_user.must_change_password,
        )?;

        AuditService::write_best_effort(
            app,
            AuditWriteInput::success(current_user, audit_action::user::PASSWORD_RESET)
                .with_entity("user", new_user.id.clone())
                .with_snapshots(Some(old_val), Some(new_val))
                .with_details(technical_details),
        );
        Ok::<(), crate::errors::app_error::AppErrorDto>(())
    })();

    if let Err(err) = result {
        eprintln!(
            "[audit] write_user_password_reset_audit failed: {}",
            err.message
        );
    }
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
