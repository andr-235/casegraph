#[cfg(test)]
mod tests {
    use crate::audit::audit_metadata::*;
    use crate::audit::audit_snapshot_assertions::{
        assert_audit_value_is_safe, assert_optional_audit_value_is_safe,
    };

    // ── user ─────────────────────────────────────────────────────────────────

    #[test]
    fn user_snapshot_does_not_expose_password_fields() {
        let snapshot = user_snapshot("Иванов И.И.", "Иванов И.И.", "analyst", true, false);
        let value = serde_json::to_value(snapshot).unwrap();

        assert_audit_value_is_safe("user_snapshot", &value);
        assert!(value.get("password").is_none());
        assert!(value.get("passwordHash").is_none());
        assert!(value.get("temporaryPassword").is_none());
    }

    // ── timeline ─────────────────────────────────────────────────────────────

    #[test]
    fn timeline_event_snapshot_is_safe() {
        let snapshot = timeline_event_snapshot(
            "EVT-001",
            "Контрольная встреча",
            Some("Описание встречи"),
            "2026-06-27",
            true,
        );
        let value = serde_json::to_value(snapshot).unwrap();

        assert_audit_value_is_safe("timeline_event_snapshot", &value);
        assert!(value.get("body").is_none());
        assert!(value.get("content").is_none());
        assert!(value.get("fileContent").is_none());
    }

    // ── case / relation ───────────────────────────────────────────────────────

    #[test]
    fn case_snapshot_is_safe() {
        let snapshot = case_snapshot(
            "CASE-001",
            "Тестовое дело",
            Some("Иванов И.И."),
            "in_progress",
            Some("2026-01-01"),
            Some("2026-01-31"),
            Some("Описание дела"),
        );
        let value = serde_json::to_value(snapshot).unwrap();

        assert_audit_value_is_safe("case_snapshot", &value);
    }

    #[test]
    fn relation_snapshot_is_safe() {
        let snapshot = relation_snapshot(
            "REL-001",
            "P-001",
            "TEL-001",
            "phone_contact",
            "confirmed",
            Some("Основание связи"),
            Some("MAT-001"),
            true,
        );
        let value = serde_json::to_value(snapshot).unwrap();

        assert_audit_value_is_safe("relation_snapshot", &value);
        assert!(value.get("storedPath").is_none());
        assert!(value.get("originalPath").is_none());
    }

    // ── material ─────────────────────────────────────────────────────────────

    #[test]
    fn material_snapshot_does_not_expose_local_paths() {
        let snapshot = material_snapshot(
            "MAT-001",
            "photo.png",
            "image",
            Some(1024),
            Some("a3f6d7"),
            Some("ok"),
            Some("Описание"),
            Some("2026-06-27T10:00:00"),
            true,
        );
        let value = serde_json::to_value(snapshot).unwrap();

        assert_audit_value_is_safe("material_snapshot", &value);
        assert!(value.get("originalPath").is_none());
        assert!(value.get("storedPath").is_none());
        assert!(value.get("storagePath").is_none());
        assert!(value.get("thumbnailPath").is_none());
        assert!(value.get("fileContent").is_none());
    }

    #[test]
    #[should_panic(expected = "matches forbidden key storedPath")]
    fn denylist_rejects_path_keys() {
        let value = serde_json::json!({
            "materialCode": "MAT-001",
            "storedPath": "C:\\Users\\Ivan\\Desktop\\photo.png"
        });
        assert_audit_value_is_safe("unsafe_material_snapshot", &value);
    }

    #[test]
    #[should_panic(expected = "looks like an absolute local path")]
    fn denylist_rejects_windows_absolute_path_values() {
        let value = serde_json::json!({
            "materialCode": "MAT-001",
            "note": "C:\\Users\\Ivan\\Desktop\\photo.png"
        });
        assert_audit_value_is_safe("unsafe_material_snapshot", &value);
    }

    // ── object ───────────────────────────────────────────────────────────────

    #[test]
    fn object_snapshot_is_safe() {
        let snapshot = object_snapshot(
            "P-001",
            "person",
            "Иванов Иван",
            Some("Ключевой объект"),
            true,
            Some(true),
        );
        let value = serde_json::to_value(snapshot).unwrap();

        assert_audit_value_is_safe("object_snapshot", &value);
        assert!(value.get("payload").is_none());
        assert!(value.get("rawDto").is_none());
    }

    // ── report draft ──────────────────────────────────────────────────────────

    #[test]
    fn report_draft_snapshot_does_not_expose_report_content() {
        let snapshot = report_draft_snapshot(
            Some("DRF-001"),
            "Аналитическая справка",
            "analytical_report",
            Some("draft"),
            8,
            12500,
            10,
            6,
            4,
            5,
            None,
        );
        let value = serde_json::to_value(snapshot).unwrap();

        assert_audit_value_is_safe("report_draft_snapshot", &value);
        assert!(value.get("content").is_none());
        assert!(value.get("body").is_none());
        assert!(value.get("sections").is_none());
        assert!(value.get("html").is_none());
        assert!(value.get("markdown").is_none());
        assert!(value.get("editorState").is_none());
    }

    #[test]
    #[should_panic(expected = "matches forbidden key")]
    fn denylist_rejects_report_sections() {
        let value = serde_json::json!({
            "draftCode": "DRF-001",
            "sections": [
                { "title": "Выводы", "sectionContent": "Полный текст справки..." }
            ]
        });
        assert_audit_value_is_safe("unsafe_report_snapshot", &value);
    }

    // ── settings ──────────────────────────────────────────────────────────────

    #[test]
    fn settings_snapshot_redacts_sensitive_and_path_values() {
        let changes = vec![
            setting_change_snapshot(
                "backup_path",
                "storage",
                &serde_json::json!("[redacted:path]"),
                &serde_json::json!("[redacted:path]"),
            ),
            setting_change_snapshot(
                "api_token",
                "security",
                &serde_json::json!("[redacted:secret]"),
                &serde_json::json!("[redacted:secret]"),
            ),
            setting_change_snapshot(
                "viewer_can_export_docx",
                "access",
                &serde_json::json!(false),
                &serde_json::json!(true),
            ),
        ];

        let snapshot = settings_snapshot(changes);
        let value = serde_json::to_value(snapshot).unwrap();

        assert_audit_value_is_safe("settings_snapshot", &value);

        let text = value.to_string();
        assert!(
            !text.contains("C:\\"),
            "text must not contain Windows paths"
        );
        assert!(
            !text.contains("/Users/"),
            "text must not contain Unix paths"
        );
        assert!(
            !text.contains("secret-token-value"),
            "text must not contain raw secrets"
        );
    }

    // ── backup ────────────────────────────────────────────────────────────────

    #[test]
    fn backup_snapshot_does_not_expose_paths_or_archive_content() {
        let snapshot = backup_snapshot(
            Some("BCK-001"),
            "full",
            "completed",
            None,
            Some("0.1.0"),
            Some("202606270001"),
            Some(2048),
            Some("abc123"),
            None,
            Some("2026-06-27T10:00:00"),
            Some("2026-06-27T10:01:00"),
        );
        let value = serde_json::to_value(snapshot).unwrap();

        assert_audit_value_is_safe("backup_snapshot", &value);
        assert!(value.get("backupPath").is_none());
        assert!(value.get("archivePath").is_none());
        assert!(value.get("fileList").is_none());
        assert!(value.get("zipBytes").is_none());
        assert!(value.get("sqliteDump").is_none());
    }

    #[test]
    #[should_panic(expected = "matches forbidden key")]
    fn denylist_rejects_backup_archive_path() {
        let value = serde_json::json!({
            "backupCode": "BCK-001",
            "archivePath": "D:\\Backups\\casegraph.zip"
        });
        assert_audit_value_is_safe("unsafe_backup_snapshot", &value);
    }

    // ── restore ───────────────────────────────────────────────────────────────

    #[test]
    fn restore_snapshot_is_safe() {
        let restored_counts = BackupEntityCounts {
            cases: 1,
            materials: 3,
            objects: 2,
            relations: 1,
            events: 1,
            report_drafts: 1,
            audit_logs: 20,
            integrity_results: 3,
        };

        let snapshot = restore_snapshot(
            Some("BCK-001"),
            "full",
            "completed",
            Some("BCK-SAFE-001"),
            Some("0.1.0"),
            Some("202606270001"),
            Some(restored_counts),
            Some("2026-06-27T10:00:00"),
            Some("2026-06-27T10:02:00"),
        );
        let value = serde_json::to_value(snapshot).unwrap();

        assert_audit_value_is_safe("restore_snapshot", &value);
        assert!(value.get("sourcePath").is_none());
        assert!(value.get("targetPath").is_none());
        assert!(value.get("archiveContent").is_none());
    }

    // ── integrity ─────────────────────────────────────────────────────────────

    #[test]
    fn integrity_run_snapshot_is_safe() {
        let snapshot = integrity_run_snapshot(
            "run-001",
            "case",
            Some("CASE-001"),
            10,
            8,
            1,
            1,
            0,
            vec!["MAT-001".to_string(), "MAT-002".to_string()],
            Some("2026-06-27T10:00:00"),
            Some("2026-06-27T10:01:00"),
        );
        let value = serde_json::to_value(snapshot).unwrap();

        assert_audit_value_is_safe("integrity_run_snapshot", &value);
        assert!(value.get("originalPath").is_none());
        assert!(value.get("storedPath").is_none());
        assert!(value.get("fileContent").is_none());
    }

    #[test]
    fn integrity_material_snapshot_is_safe() {
        let snapshot = integrity_material_snapshot(
            "MAT-001",
            "photo.png",
            Some("ok"),
            "mismatch",
            Some("expected-sha"),
            Some("actual-sha"),
            Some("2026-06-27T10:00:00"),
        );
        let value = serde_json::to_value(snapshot).unwrap();

        assert_audit_value_is_safe("integrity_material_snapshot", &value);
    }

    // ── technical_details builders ────────────────────────────────────────────

    #[test]
    fn technical_details_builders_are_safe() {
        let values: Vec<serde_json::Value> = vec![
            // case_updated takes (case_id, case_code, &[&str])
            case_updated("case-1", "CASE-1", &["title", "status"]).unwrap().into_value(),
            // relation_updated takes (relation_id, relation_code, &[&str])
            relation_updated("relation-1", "REL-1", &["confidenceLevel"]).unwrap().into_value(),
            // report_draft_updated takes (draft_id, case_id, &[&str])
            report_draft_updated("draft-1", "case-1", &["title"]).unwrap().into_value(),
            // backup_created
            backup_created("backup-1", "full", None, Some(1024)).unwrap().into_value(),
            // integrity_check_completed
            integrity_check_completed("run-1", "case", Some("case-1"), 10, 2).unwrap().into_value(),
        ];

        for value in &values {
            assert_audit_value_is_safe("technical_details", value);
        }
    }

    // ── old / new pair ────────────────────────────────────────────────────────

    #[test]
    fn old_new_values_are_checked_as_independent_audit_values() {
        let before = user_snapshot("Иванов И.И.", "Иванов И.И.", "viewer", true, false);
        let after = user_snapshot("Иванов И.И.", "Иванов И.И.", "analyst", true, false);

        let (old_value, new_value) = old_new(before, after);

        assert_optional_audit_value_is_safe("old_value", &old_value);
        assert_optional_audit_value_is_safe("new_value", &new_value);
    }
}
