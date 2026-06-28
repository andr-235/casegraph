use rusqlite::Connection;
use serde::Serialize;
use tauri::AppHandle;

use crate::errors::app_error::AppErrorDto;
use crate::security::session::SessionState;
use crate::security::PolicyAwarePermissionGuard;
use crate::security::ProtectedOperation;
use crate::services::protected_service_context::require_protected_user_for;

#[derive(Debug, Clone, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct ReportDraftSection {
    pub content: String,
}

#[derive(Debug, Clone, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct ReportDraftContent {
    pub sections: Vec<ReportDraftSection>,
    pub materials: Vec<String>,
    pub objects: Vec<String>,
    pub relations: Vec<String>,
    pub events: Vec<String>,
}

#[derive(Debug, Clone, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct ReportDraft {
    pub id: String,
    pub case_id: String,
    pub draft_code: Option<String>,
    pub title: String,
    pub report_type: String,
    pub status: Option<String>,
    pub content: ReportDraftContent,
    pub exported_at: Option<String>,
}

pub struct ReportRepository;

impl ReportRepository {
    pub fn get_report_draft_by_id(
        _conn: &Connection,
        draft_id: &str,
    ) -> Result<ReportDraft, AppErrorDto> {
        // Stub implementation returning a mock draft for compile/test safety
        Ok(ReportDraft {
            id: draft_id.to_string(),
            case_id: "case-stub-id".to_string(),
            draft_code: Some("RPT-001".to_string()),
            title: "Черновик справки".to_string(),
            report_type: "analytical_report".to_string(),
            status: Some("draft".to_string()),
            content: ReportDraftContent {
                sections: vec![ReportDraftSection {
                    content: "Секция 1".to_string(),
                }],
                materials: vec!["mat-1".to_string()],
                objects: vec!["obj-1".to_string()],
                relations: vec!["rel-1".to_string()],
                events: vec!["evt-1".to_string()],
            },
            exported_at: None,
        })
    }

    pub fn generate_draft(
        _conn: &Connection,
        _case_id: &str,
        _title: &str,
        _report_type: &str,
    ) -> Result<String, AppErrorDto> {
        Ok("mock-draft-id".to_string())
    }

    pub fn update_draft(
        _conn: &Connection,
        _draft_id: &str,
        _title: &str,
        _report_type: &str,
    ) -> Result<(), AppErrorDto> {
        Ok(())
    }

    pub fn delete_draft(_conn: &Connection, _draft_id: &str) -> Result<(), AppErrorDto> {
        Ok(())
    }
}

pub struct GenerateReportDraftPayload {
    pub case_id: String,
    pub title: String,
    pub report_type: String,
}

pub struct UpdateReportDraftPayload {
    pub draft_id: String,
    pub title: String,
    pub report_type: String,
}

pub struct ReportDraftPayload {
    pub draft_id: String,
}

pub struct ValidationResult {
    pub is_valid: bool,
    pub warnings: Vec<String>,
    pub errors: Vec<String>,
}

pub struct ReportDraftService;

impl ReportDraftService {
    pub fn generate_report_draft(
        app: &AppHandle,
        session: &SessionState,
        payload: GenerateReportDraftPayload,
    ) -> Result<ReportDraft, AppErrorDto> {
        let context = require_protected_user_for(app, session, "GENERATE_REPORT_DRAFT")?;
        let conn = &context.conn;

        let draft_id = ReportRepository::generate_draft(
            conn,
            &payload.case_id,
            &payload.title,
            &payload.report_type,
        )?;

        let created_draft = ReportRepository::get_report_draft_by_id(conn, &draft_id)?;

        // Metrics calculation helper call
        let metrics =
            crate::audit::audit_metadata::report_draft_metrics_from_content(&created_draft.content);

        let new_value = crate::audit::audit_metadata::safe_report_draft_snapshot(
            created_draft.draft_code.as_deref(),
            &created_draft.title,
            &created_draft.report_type,
            created_draft.status.as_deref(),
            metrics.section_count,
            metrics.character_count,
            metrics.included_materials_count,
            metrics.included_objects_count,
            metrics.included_relations_count,
            metrics.included_events_count,
            created_draft.exported_at.as_deref(),
        )?;

        let technical_details = crate::audit::audit_metadata::report_draft_generated(
            &created_draft.id,
            &created_draft.case_id,
            &created_draft.report_type,
        )?;

        crate::audit::audit_service::AuditService::write_best_effort(
            app,
            crate::audit::audit_service::AuditWriteInput::success(
                &context.current_user,
                crate::domain::audit_action::report::DRAFT_GENERATED,
            )
            .with_case_id(created_draft.case_id.clone())
            .with_entity("report_draft", created_draft.id.clone())
            .with_snapshots(None, Some(new_value))
            .with_details(technical_details),
        );

        Ok(created_draft)
    }

    pub fn update_report_draft(
        app: &AppHandle,
        session: &SessionState,
        payload: UpdateReportDraftPayload,
    ) -> Result<ReportDraft, AppErrorDto> {
        let context = require_protected_user_for(app, session, "UPDATE_REPORT_DRAFT")?;
        let conn = &context.conn;

        let old_draft = ReportRepository::get_report_draft_by_id(conn, &payload.draft_id)?;

        ReportRepository::update_draft(
            conn,
            &payload.draft_id,
            &payload.title,
            &payload.report_type,
        )?;

        let updated_draft = ReportRepository::get_report_draft_by_id(conn, &payload.draft_id)?;

        let old_metrics =
            crate::audit::audit_metadata::report_draft_metrics_from_content(&old_draft.content);
        let updated_metrics =
            crate::audit::audit_metadata::report_draft_metrics_from_content(&updated_draft.content);

        let old_value = crate::audit::audit_metadata::safe_report_draft_snapshot(
            old_draft.draft_code.as_deref(),
            &old_draft.title,
            &old_draft.report_type,
            old_draft.status.as_deref(),
            old_metrics.section_count,
            old_metrics.character_count,
            old_metrics.included_materials_count,
            old_metrics.included_objects_count,
            old_metrics.included_relations_count,
            old_metrics.included_events_count,
            old_draft.exported_at.as_deref(),
        )?;

        let new_value = crate::audit::audit_metadata::safe_report_draft_snapshot(
            updated_draft.draft_code.as_deref(),
            &updated_draft.title,
            &updated_draft.report_type,
            updated_draft.status.as_deref(),
            updated_metrics.section_count,
            updated_metrics.character_count,
            updated_metrics.included_materials_count,
            updated_metrics.included_objects_count,
            updated_metrics.included_relations_count,
            updated_metrics.included_events_count,
            updated_draft.exported_at.as_deref(),
        )?;

        let mut changed_fields = Vec::new();
        crate::audit::audit_metadata::push_changed(
            &mut changed_fields,
            "title",
            &old_draft.title,
            &updated_draft.title,
        );
        crate::audit::audit_metadata::push_changed(
            &mut changed_fields,
            "reportType",
            &old_draft.report_type,
            &updated_draft.report_type,
        );
        crate::audit::audit_metadata::push_changed(
            &mut changed_fields,
            "status",
            &old_draft.status,
            &updated_draft.status,
        );
        crate::audit::audit_metadata::push_changed(
            &mut changed_fields,
            "sectionCount",
            &old_metrics.section_count,
            &updated_metrics.section_count,
        );
        crate::audit::audit_metadata::push_changed(
            &mut changed_fields,
            "characterCount",
            &old_metrics.character_count,
            &updated_metrics.character_count,
        );

        let technical_details = crate::audit::audit_metadata::report_draft_updated(
            &updated_draft.id,
            &updated_draft.case_id,
            &changed_fields,
        )?;

        crate::audit::audit_service::AuditService::write_best_effort(
            app,
            crate::audit::audit_service::AuditWriteInput::success(
                &context.current_user,
                crate::domain::audit_action::report::DRAFT_UPDATED,
            )
            .with_case_id(updated_draft.case_id.clone())
            .with_entity("report_draft", updated_draft.id.clone())
            .with_snapshots(Some(old_value), Some(new_value))
            .with_details(technical_details),
        );

        Ok(updated_draft)
    }

    pub fn validate_report_draft_for_docx(
        app: &AppHandle,
        session: &SessionState,
        payload: ReportDraftPayload,
    ) -> Result<ValidationResult, AppErrorDto> {
        let context = require_protected_user_for(app, session, "VALIDATE_REPORT_DRAFT")?;
        let conn = &context.conn;

        PolicyAwarePermissionGuard::require(
            app,
            conn,
            &context.current_user,
            ProtectedOperation::DocxExport,
        )?;

        let draft = ReportRepository::get_report_draft_by_id(conn, &payload.draft_id)?;

        let validation_result = ValidationResult {
            is_valid: true,
            warnings: vec![],
            errors: vec![],
        };

        let technical_details = crate::audit::audit_metadata::report_draft_validated(
            &draft.id,
            &draft.case_id,
            validation_result.is_valid,
            validation_result.warnings.len(),
            validation_result.errors.len(),
        )?;

        crate::audit::audit_service::AuditService::write_best_effort(
            app,
            crate::audit::audit_service::AuditWriteInput::success(
                &context.current_user,
                crate::domain::audit_action::report::DRAFT_VALIDATED,
            )
            .with_case_id(draft.case_id.clone())
            .with_entity("report_draft", draft.id.clone())
            .with_details(technical_details),
        );

        Ok(validation_result)
    }

    pub fn delete_report_draft(
        app: &AppHandle,
        session: &SessionState,
        payload: ReportDraftPayload,
    ) -> Result<(), AppErrorDto> {
        let context = require_protected_user_for(app, session, "DELETE_REPORT_DRAFT")?;
        let conn = &context.conn;

        let old_draft = ReportRepository::get_report_draft_by_id(conn, &payload.draft_id)?;
        let metrics =
            crate::audit::audit_metadata::report_draft_metrics_from_content(&old_draft.content);

        let old_value = crate::audit::audit_metadata::safe_report_draft_snapshot(
            old_draft.draft_code.as_deref(),
            &old_draft.title,
            &old_draft.report_type,
            old_draft.status.as_deref(),
            metrics.section_count,
            metrics.character_count,
            metrics.included_materials_count,
            metrics.included_objects_count,
            metrics.included_relations_count,
            metrics.included_events_count,
            old_draft.exported_at.as_deref(),
        )?;

        ReportRepository::delete_draft(conn, &payload.draft_id)?;

        let technical_details =
            crate::audit::audit_metadata::report_draft_deleted(&old_draft.id, &old_draft.case_id)?;

        crate::audit::audit_service::AuditService::write_best_effort(
            app,
            crate::audit::audit_service::AuditWriteInput::success(
                &context.current_user,
                crate::domain::audit_action::report::DRAFT_DELETED,
            )
            .with_case_id(old_draft.case_id.clone())
            .with_entity("report_draft", old_draft.id.clone())
            .with_snapshots(Some(old_value), None)
            .with_details(technical_details),
        );

        Ok(())
    }
}
