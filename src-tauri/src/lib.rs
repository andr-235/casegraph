pub mod commands;
pub mod db;
pub mod errors;
pub mod security;

use commands::app_commands::initialize_app;
use commands::auth_commands::create_first_admin;

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .invoke_handler(tauri::generate_handler![initialize_app, create_first_admin])
        .run(tauri::generate_context!())
        .expect("error while running CaseGraph");
}
