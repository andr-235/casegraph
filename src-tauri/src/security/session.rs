use std::sync::Mutex;

use serde::Serialize;

#[derive(Debug, Clone, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct CurrentUserDto {
    pub user_id: String,
    pub username: String,
    pub display_name: String,
    pub role: String,
}

#[derive(Default)]
pub struct SessionState {
    current_user: Mutex<Option<CurrentUserDto>>,
}

impl SessionState {
    pub fn get_current_user(&self) -> Option<CurrentUserDto> {
        self.current_user
            .lock()
            .ok()
            .and_then(|guard| guard.clone())
    }

    pub fn set_current_user(&self, user: CurrentUserDto) {
        if let Ok(mut guard) = self.current_user.lock() {
            *guard = Some(user);
        }
    }

    pub fn clear_current_user(&self) {
        if let Ok(mut guard) = self.current_user.lock() {
            *guard = None;
        }
    }
}
