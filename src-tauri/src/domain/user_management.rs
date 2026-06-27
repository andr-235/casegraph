use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct GetUserByIdPayload {
    pub user_id: String,
}

#[derive(Debug, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct GetUserByIdResponse {
    pub user: UserListItemDto,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct UpdateUserPayload {
    pub user_id: String,
    pub display_name: Option<String>,
    pub role_code: String,
    pub must_change_password: bool,
}

#[derive(Debug, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct UpdateUserResponse {
    pub user: UserListItemDto,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CreateUserPayload {
    pub username: String,
    pub display_name: Option<String>,
    pub role_code: String,
    pub password: String,
    pub must_change_password: Option<bool>,
}

#[derive(Debug, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct CreateUserResponse {
    pub user: UserListItemDto,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct GetUsersPayload {
    pub query: Option<String>,
    pub role: Option<String>,
    pub status: Option<String>,
    pub limit: Option<i64>,
    pub offset: Option<i64>,
}

#[derive(Debug, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct UserListItemDto {
    pub id: String,
    pub username: String,
    pub display_name: Option<String>,
    pub role_code: String,
    pub role_title: String,
    pub is_active: bool,
    pub must_change_password: bool,
    pub last_login_at: Option<String>,
    pub created_at: String,
    pub updated_at: String,
}

#[derive(Debug, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct GetUsersResponse {
    pub users: Vec<UserListItemDto>,
    pub total: i64,
}

#[derive(Debug, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct RoleOptionDto {
    pub id: String,
    pub role_code: String,
    pub title: String,
}

#[derive(Debug, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct GetRolesResponse {
    pub roles: Vec<RoleOptionDto>,
}
