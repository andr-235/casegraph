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
