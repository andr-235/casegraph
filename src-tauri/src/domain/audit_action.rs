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
