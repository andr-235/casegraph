pub const DATE_PRECISION_DAY: &str = "day";
pub const DATE_PRECISION_MONTH: &str = "month";
pub const DATE_PRECISION_YEAR: &str = "year";
pub const DATE_PRECISION_APPROXIMATE: &str = "approximate";
pub const DATE_PRECISION_PERIOD: &str = "period";

pub const DATE_PRECISIONS: [&str; 5] = [
    DATE_PRECISION_DAY,
    DATE_PRECISION_MONTH,
    DATE_PRECISION_YEAR,
    DATE_PRECISION_APPROXIMATE,
    DATE_PRECISION_PERIOD,
];

pub fn is_valid_date_precision(value: &str) -> bool {
    DATE_PRECISIONS.contains(&value)
}
