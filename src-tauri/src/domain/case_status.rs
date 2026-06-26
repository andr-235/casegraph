pub const CASE_STATUS_DRAFT: &str = "draft";
pub const CASE_STATUS_IN_PROGRESS: &str = "in_progress";
pub const CASE_STATUS_PREPARED: &str = "prepared";
pub const CASE_STATUS_COMPLETED: &str = "completed";
pub const CASE_STATUS_ARCHIVED: &str = "archived";

pub const EDITABLE_CASE_STATUSES: [&str; 4] = [
    CASE_STATUS_DRAFT,
    CASE_STATUS_IN_PROGRESS,
    CASE_STATUS_PREPARED,
    CASE_STATUS_COMPLETED,
];

pub fn is_editable_case_status(status: &str) -> bool {
    EDITABLE_CASE_STATUSES.contains(&status)
}
