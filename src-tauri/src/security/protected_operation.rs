#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ProtectedOperation {
    CaseCreate,
    CaseUpdate,
    MaterialImport,
    MaterialUpdate,
    ObjectCreate,
    ObjectUpdate,
    RelationCreate,
    RelationUpdate,
    TimelineCreate,
    TimelineUpdate,
    ReportDraftGenerate,
    ReportDraftUpdate,
    DocxExport,
    BackupCreate,
    BackupRestore,
    IntegrityCheckRun,
    AuditLogRead,
    SettingsRead,
    SettingsUpdate,
    UserManage,
}

impl ProtectedOperation {
    pub fn action_name(self) -> &'static str {
        match self {
            Self::CaseCreate => "case.create",
            Self::CaseUpdate => "case.update",
            Self::MaterialImport => "material.import",
            Self::MaterialUpdate => "material.update",
            Self::ObjectCreate => "object.create",
            Self::ObjectUpdate => "object.update",
            Self::RelationCreate => "relation.create",
            Self::RelationUpdate => "relation.update",
            Self::TimelineCreate => "timeline.create",
            Self::TimelineUpdate => "timeline.update",
            Self::ReportDraftGenerate => "report.generate",
            Self::ReportDraftUpdate => "report.update",
            Self::DocxExport => "docx.export",
            Self::BackupCreate => "backup.create",
            Self::BackupRestore => "backup.restore",
            Self::IntegrityCheckRun => "integrity.run",
            Self::AuditLogRead => "audit.read",
            Self::SettingsRead => "settings.read",
            Self::SettingsUpdate => "settings.update",
            Self::UserManage => "user.manage",
        }
    }
}
