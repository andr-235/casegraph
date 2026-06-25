pub mod commands;
pub mod db;
pub mod errors;
pub mod repositories;
pub mod security;
pub mod services;

use commands::app_commands::initialize_app;
use commands::auth_commands::{create_first_admin, get_current_user, login, logout};
use security::session::SessionState;

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .manage(SessionState::default())
        .invoke_handler(tauri::generate_handler![
            initialize_app,
            create_first_admin,
            login,
            get_current_user,
            logout
        ])
        .run(tauri::generate_context!())
        .expect("error while running CaseGraph");
}
