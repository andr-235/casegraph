use tauri::AppHandle;

use crate::db::connection::open_connection;
use crate::domain::graph::{GetGraphDataPayload, GetGraphDataResponse};
use crate::errors::app_error::AppErrorDto;
use crate::repositories::graph_repository::GraphRepository;
use crate::security::session::SessionState;
use crate::services::protected_service_guard::ProtectedServiceGuard;

pub struct GraphService;

impl GraphService {
    pub fn get_graph_data(
        app: &AppHandle,
        session: &SessionState,
        payload: GetGraphDataPayload,
    ) -> Result<GetGraphDataResponse, AppErrorDto> {
        let current_user = session.get_current_user().ok_or_else(|| {
            AppErrorDto::new("ERR_UNAUTHORIZED", "Пользователь не авторизован.", None)
        })?;

        let case_id =
            normalize_required_id(&payload.case_id, "ERR_CASE_REQUIRED", "Не выбрано дело.")?;

        let conn = open_connection(app)?;
        ProtectedServiceGuard::require_password_change_resolved(&conn, &current_user)?;

        let nodes = GraphRepository::get_nodes(&conn, &case_id)?;
        let edges = GraphRepository::get_edges(&conn, &case_id)?;

        Ok(GetGraphDataResponse { nodes, edges })
    }
}

fn normalize_required_id(value: &str, code: &str, message: &str) -> Result<String, AppErrorDto> {
    let normalized = value.trim().to_string();

    if normalized.is_empty() {
        return Err(AppErrorDto::new(code, message, None));
    }

    Ok(normalized)
}
