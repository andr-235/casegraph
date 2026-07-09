use tauri::AppHandle;

use crate::domain::case_overview::CaseOverviewDto;
use crate::domain::case_summary::CaseSummaryDto;
use crate::errors::app_error::AppErrorDto;
use crate::repositories::case_repository::CaseRepository;
use crate::repositories::case_summary_repository::CaseSummaryRepository;
use crate::security::session::SessionState;
use crate::security::ProtectedOperation;
use crate::security::ProtectedServiceContext;

pub struct CaseSummaryService;

impl CaseSummaryService {
    /// Сводка по делу — счётчики для боковой панели
    pub fn get_case_summary(
        app: &AppHandle,
        _session: &SessionState,
        case_id: &str,
    ) -> Result<CaseSummaryDto, AppErrorDto> {
        log::debug!(
            "[CaseSummaryService] get_case_summary START case_id={}",
            case_id
        );

        let case_id = case_id.trim();
        if case_id.is_empty() {
            return Err(AppErrorDto::validation("Не указан идентификатор дела."));
        }

        let context =
            ProtectedServiceContext::require_operation(app, ProtectedOperation::CaseRead)?;
        let conn = &context.conn;

        let object_count = CaseSummaryRepository::count_objects(conn, case_id)?;
        let key_object_count = CaseSummaryRepository::count_key_objects(conn, case_id)?;
        let material_count = CaseSummaryRepository::count_materials(conn, case_id)?;
        let integrity_issue_count =
            CaseSummaryRepository::count_materials_with_integrity_issues(conn, case_id)?;
        let relation_count = CaseSummaryRepository::count_relations(conn, case_id)?;
        let event_count = CaseSummaryRepository::count_events(conn, case_id)?;
        let report_event_count = CaseSummaryRepository::count_report_events(conn, case_id)?;
        let updated_at = CaseSummaryRepository::get_max_updated_at(conn, case_id)?;

        let summary = CaseSummaryDto {
            object_count,
            key_object_count,
            material_count,
            integrity_issue_count,
            relation_count,
            event_count,
            report_event_count,
            updated_at,
        };

        log::debug!(
            "[CaseSummaryService] get_case_summary END case_id={} objects={} materials={} relations={} events={}",
            case_id,
            object_count,
            material_count,
            relation_count,
            event_count,
        );

        Ok(summary)
    }

    /// Карточка дела — сводка + ключевые объекты + последняя активность
    pub fn get_case_overview(
        app: &AppHandle,
        _session: &SessionState,
        case_id: &str,
    ) -> Result<CaseOverviewDto, AppErrorDto> {
        log::debug!(
            "[CaseSummaryService] get_case_overview START case_id={}",
            case_id
        );

        let case_id = case_id.trim();
        if case_id.is_empty() {
            return Err(AppErrorDto::validation("Не указан идентификатор дела."));
        }

        let context =
            ProtectedServiceContext::require_operation(app, ProtectedOperation::CaseRead)?;
        let conn = &context.conn;

        let summary = Self::get_case_summary(app, _session, case_id)?;

        let case_row = CaseRepository::get_case_by_id(conn, case_id)?
            .ok_or_else(|| AppErrorDto::not_found("Дело не найдено."))?;

        let case_item = crate::services::case_service::case_row_to_dto(case_row);

        let key_objects = CaseSummaryRepository::get_key_objects(conn, case_id, 10)?;
        let recent_activity = CaseSummaryRepository::get_recent_activity(conn, case_id, 7)?;

        let overview = CaseOverviewDto {
            case_item,
            summary,
            key_objects,
            recent_activity,
        };

        log::debug!(
            "[CaseSummaryService] get_case_overview END case_id={} key_objects={} activities={}",
            case_id,
            overview.key_objects.len(),
            overview.recent_activity.len(),
        );

        Ok(overview)
    }
}
