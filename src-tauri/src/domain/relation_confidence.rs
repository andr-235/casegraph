pub const CONFIDENCE_HIGH: &str = "high";
pub const CONFIDENCE_MEDIUM: &str = "medium";
pub const CONFIDENCE_LOW: &str = "low";
pub const CONFIDENCE_REQUIRES_CHECK: &str = "requires_check";

pub const CONFIDENCE_LEVELS: [&str; 4] = [
    CONFIDENCE_HIGH,
    CONFIDENCE_MEDIUM,
    CONFIDENCE_LOW,
    CONFIDENCE_REQUIRES_CHECK,
];

pub fn is_valid_confidence_level(value: &str) -> bool {
    CONFIDENCE_LEVELS.contains(&value)
}
