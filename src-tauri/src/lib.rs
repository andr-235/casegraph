pub mod commands;
pub mod db;
pub mod domain;
pub mod errors;
pub mod repositories;
pub mod security;
pub mod services;

use commands::app_commands::initialize_app;
use commands::auth_commands::{create_first_admin, get_current_user, login, logout};
use commands::case_commands::{create_case, get_case_by_id, get_cases, update_case};
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
            logout,
            get_cases,
            create_case,
            get_case_by_id,
            update_case
        ])
        .run(tauri::generate_context!())
        .expect("error while running CaseGraph");
}
