pub mod commands;
pub mod db;
pub mod domain;
pub mod errors;
pub mod repositories;
pub mod security;
pub mod services;
pub mod storage;

use commands::app_commands::initialize_app;
use commands::auth_commands::{create_first_admin, get_current_user, login, logout};
use commands::case_commands::{
    create_case, get_case_by_id, get_cases, update_case, update_case_status,
};
use commands::material_commands::{
    create_material, delete_material, get_materials, update_material,
};
use commands::object_commands::{
    create_object, get_object_by_id, get_objects, link_object_to_materials, soft_delete_object,
    update_object,
};
use commands::relation_commands::{create_relation, get_relations};
use security::session::SessionState;

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .plugin(tauri_plugin_dialog::init())
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
            update_case,
            update_case_status,
            get_materials,
            create_material,
            update_material,
            delete_material,
            create_object,
            get_objects,
            get_object_by_id,
            update_object,
            link_object_to_materials,
            soft_delete_object,
            create_relation,
            get_relations
        ])
        .run(tauri::generate_context!())
        .expect("error while running CaseGraph");
}
