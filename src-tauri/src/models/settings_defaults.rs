// Thin compatibility wrapper.
// Callers should prefer crate::models::settings_catalog::default_setting_pairs() directly.
pub use crate::models::settings_catalog::default_setting_pairs;

/// A named pair kept for call-sites that destructure by field name.
#[derive(Debug, Clone)]
pub struct DefaultSettingPair {
    pub key: String,
    pub value: String,
}

pub fn default_settings_pairs() -> Vec<DefaultSettingPair> {
    crate::models::settings_catalog::default_setting_pairs()
        .into_iter()
        .map(|(key, value)| DefaultSettingPair { key, value })
        .collect()
}
