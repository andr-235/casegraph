pub mod commands;
pub mod db;
pub mod errors;

use commands::app_commands::initialize_app;

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .invoke_handler(tauri::generate_handler![initialize_app])
        .run(tauri::generate_context!())
        .expect("error while running CaseGraph");
}
