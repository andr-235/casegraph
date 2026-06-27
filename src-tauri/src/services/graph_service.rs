use tauri::AppHandle;

use crate::domain::graph::{GetGraphDataPayload, GetGraphDataResponse};
use crate::errors::app_error::AppErrorDto;
use crate::repositories::graph_repository::GraphRepository;
use crate::security::session::SessionState;
use crate::services::protected_service_context::require_protected_user_for;

pub struct GraphService;

impl GraphService {
    pub fn get_graph_data(
        app: &AppHandle,
        session: &SessionState,
        payload: GetGraphDataPayload,
    ) -> Result<GetGraphDataResponse, AppErrorDto> {
        let context = require_protected_user_for(app, session, "GET_GRAPH_DATA")?;
        let conn = &context.conn;

        let case_id =
            normalize_required_id(&payload.case_id, "ERR_CASE_REQUIRED", "Не выбрано дело.")?;

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
