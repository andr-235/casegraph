pub mod auth {
    pub const LOGIN_SUCCEEDED: &str = "AUTH_LOGIN_SUCCEEDED";
    pub const LOGIN_FAILED: &str = "AUTH_LOGIN_FAILED";
    pub const LOGOUT: &str = "AUTH_LOGOUT";
}

pub mod case {
    pub const CREATED: &str = "CASE_CREATED";
    pub const UPDATED: &str = "CASE_UPDATED";
    pub const STATUS_CHANGED: &str = "CASE_STATUS_CHANGED";
}

pub mod timeline {
    pub const EVENT_CREATED: &str = "TIMELINE_EVENT_CREATED";
    pub const EVENT_UPDATED: &str = "TIMELINE_EVENT_UPDATED";
    pub const EVENT_DELETED: &str = "TIMELINE_EVENT_DELETED";
    pub const EVENT_REPORT_INCLUDE_CHANGED: &str = "TIMELINE_EVENT_REPORT_INCLUDE_CHANGED";
}

pub mod user {
    pub const CREATED: &str = "USER_CREATED";
    pub const UPDATED: &str = "USER_UPDATED";
    pub const BLOCKED: &str = "USER_BLOCKED";
    pub const UNBLOCKED: &str = "USER_UNBLOCKED";
    pub const PASSWORD_RESET: &str = "USER_PASSWORD_RESET";
    pub const PASSWORD_CHANGED: &str = "USER_PASSWORD_CHANGED";
}

pub mod audit {
    pub const ACCESS_DENIED: &str = "ACCESS_DENIED";
    pub const LOG_EXPORTED: &str = "AUDIT_LOG_EXPORTED";
}

pub mod relation {
    pub const CREATED: &str = "RELATION_CREATED";
    pub const UPDATED: &str = "RELATION_UPDATED";
    pub const REPORT_INCLUDE_CHANGED: &str = "RELATION_REPORT_INCLUDE_CHANGED";
    pub const DELETED: &str = "RELATION_DELETED";
}

pub mod material {
    pub const IMPORTED: &str = "MATERIAL_IMPORTED";
    pub const UPDATED: &str = "MATERIAL_UPDATED";
    pub const REPORT_INCLUDE_CHANGED: &str = "MATERIAL_REPORT_INCLUDE_CHANGED";
    pub const HASH_VERIFIED: &str = "MATERIAL_HASH_VERIFIED";
    pub const HASH_MISMATCH: &str = "MATERIAL_HASH_MISMATCH";
    pub const DELETED: &str = "MATERIAL_DELETED";
}

pub mod object {
    pub const CREATED: &str = "OBJECT_CREATED";
    pub const UPDATED: &str = "OBJECT_UPDATED";
    pub const MATERIAL_LINKS_CHANGED: &str = "OBJECT_MATERIAL_LINKS_CHANGED";
    pub const KEY_FLAG_CHANGED: &str = "OBJECT_KEY_FLAG_CHANGED";
    pub const DELETED: &str = "OBJECT_DELETED";
}

pub mod report {
    pub const DRAFT_GENERATED: &str = "REPORT_DRAFT_GENERATED";
    pub const DRAFT_UPDATED: &str = "REPORT_DRAFT_UPDATED";
    pub const DRAFT_VALIDATED: &str = "REPORT_DRAFT_VALIDATED";
    pub const DRAFT_DELETED: &str = "REPORT_DRAFT_DELETED";
}

pub mod settings {
    pub const UPDATED: &str = "SETTINGS_UPDATED";
    pub const RESET_TO_DEFAULT: &str = "SETTINGS_RESET_TO_DEFAULT";
}

pub mod backup {
    pub const CREATED: &str = "BACKUP_CREATED";
    pub const CASE_CREATED: &str = "BACKUP_CASE_CREATED";
    pub const SAFETY_CREATED: &str = "BACKUP_SAFETY_CREATED";
    pub const VERIFIED: &str = "BACKUP_VERIFIED";
    pub const VERIFICATION_FAILED: &str = "BACKUP_VERIFICATION_FAILED";
    pub const RESTORE_PREFLIGHT_CHECKED: &str = "RESTORE_PREFLIGHT_CHECKED";
    pub const RESTORE_PREFLIGHT_FAILED: &str = "RESTORE_PREFLIGHT_FAILED";
    pub const RESTORE_STARTED: &str = "RESTORE_STARTED";
    pub const RESTORE_COMPLETED: &str = "RESTORE_COMPLETED";
    pub const RESTORE_FAILED: &str = "RESTORE_FAILED";
}

pub mod integrity {
    pub const CHECK_STARTED: &str = "INTEGRITY_CHECK_STARTED";
    pub const CHECK_COMPLETED: &str = "INTEGRITY_CHECK_COMPLETED";
    pub const MATERIAL_VERIFIED: &str = "INTEGRITY_MATERIAL_VERIFIED";
    pub const PROBLEM_DETECTED: &str = "INTEGRITY_PROBLEM_DETECTED";
}
