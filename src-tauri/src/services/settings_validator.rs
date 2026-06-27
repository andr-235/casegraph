/// Strict per-key validator driven by the settings catalog.
///
/// Both `update_settings` and `choose_settings_directory` (folder picker)
/// must call `validate_setting_pair` or `validate_path` so that allowed
/// values are consistent regardless of entry point.
use crate::errors::app_error::AppErrorDto;
use crate::models::settings_catalog::{setting_definition, SettingKind};

const ALLOWED_TEMPLATES: &[&str] = &[
    "analytical-report",
    "operational-summary",
    "extended-report",
];

/// Validate a single (key, value) pair against its catalog definition.
/// Returns `ERR_VALIDATION` if the key is unknown or the value is invalid.
pub fn validate_setting_pair(key: &str, value: &str) -> Result<(), AppErrorDto> {
    let def = setting_definition(key).ok_or_else(|| {
        let msg = format!("Неизвестный ключ настройки: {key}");
        AppErrorDto::validation(msg.as_str())
    })?;

    match def.kind {
        SettingKind::Bool => validate_bool(value),
        SettingKind::Str => validate_str(value),
        SettingKind::Path => validate_path(value),
        SettingKind::Template => validate_template(value),
    }
}

pub fn validate_bool(value: &str) -> Result<(), AppErrorDto> {
    match value {
        "true" | "false" => Ok(()),
        _ => Err(AppErrorDto::validation(
            "Значение булевого параметра должно быть 'true' или 'false'.",
        )),
    }
}

pub fn validate_str(value: &str) -> Result<(), AppErrorDto> {
    if value.len() > 512 {
        return Err(AppErrorDto::validation(
            "Значение параметра слишком длинное.",
        ));
    }
    if has_control_chars(value) {
        return Err(AppErrorDto::validation(
            "Значение параметра содержит недопустимые символы.",
        ));
    }
    Ok(())
}

pub fn validate_template(value: &str) -> Result<(), AppErrorDto> {
    if ALLOWED_TEMPLATES.contains(&value) {
        return Ok(());
    }
    let msg = format!(
        "Недопустимый шаблон DOCX: '{value}'. Разрешены: {allowed}.",
        allowed = ALLOWED_TEMPLATES.join(", ")
    );
    Err(AppErrorDto::validation(msg.as_str()))
}

/// Path validator shared by both the folder-picker command and update_settings.
/// Empty string is allowed (means "unset").
pub fn validate_path(value: &str) -> Result<(), AppErrorDto> {
    let trimmed = value.trim();

    if trimmed.is_empty() {
        return Ok(()); // unset is valid
    }

    if trimmed.len() > 512 {
        return Err(AppErrorDto::validation("Путь слишком длинный."));
    }

    if has_control_chars(trimmed) {
        return Err(AppErrorDto::validation(
            "Путь содержит недопустимые символы.",
        ));
    }

    let lower = trimmed.to_lowercase();

    if lower.starts_with("http://") || lower.starts_with("https://") {
        return Err(AppErrorDto::validation(
            "Сетевые URL не допускаются в настройках локального приложения.",
        ));
    }

    if trimmed.starts_with("\\\\") {
        return Err(AppErrorDto::validation(
            "UNC-сетевые пути не допускаются в MVP.",
        ));
    }

    Ok(())
}

fn has_control_chars(s: &str) -> bool {
    s.chars().any(|c| c.is_control())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn bool_accepts_true_false() {
        assert!(validate_bool("true").is_ok());
        assert!(validate_bool("false").is_ok());
        assert!(validate_bool("1").is_err());
        assert!(validate_bool("yes").is_err());
    }

    #[test]
    fn template_accepts_known_values() {
        assert!(validate_template("analytical-report").is_ok());
        assert!(validate_template("operational-summary").is_ok());
        assert!(validate_template("extended-report").is_ok());
        assert!(validate_template("custom-template").is_err());
    }

    #[test]
    fn path_accepts_empty_string() {
        assert!(validate_path("").is_ok());
    }

    #[test]
    fn path_rejects_http_url() {
        assert!(validate_path("http://example.com/backups").is_err());
        assert!(validate_path("https://example.com/backups").is_err());
    }

    #[test]
    fn path_rejects_unc_paths() {
        assert!(validate_path("\\\\server\\share").is_err());
    }

    #[test]
    fn path_rejects_over_512_chars() {
        let long = "a".repeat(513);
        assert!(validate_path(&long).is_err());
    }

    #[test]
    fn path_accepts_windows_absolute_path() {
        assert!(validate_path("C:\\Users\\admin\\backups").is_ok());
    }
}
