use crate::errors::app_error::AppErrorDto;

pub fn validate_selected_settings_path(path: &str) -> Result<(), AppErrorDto> {
    let trimmed = path.trim();

    if trimmed.is_empty() {
        return Err(AppErrorDto::validation("Путь не может быть пустым."));
    }

    if trimmed.len() > 512 {
        return Err(AppErrorDto::validation("Путь слишком длинный."));
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
