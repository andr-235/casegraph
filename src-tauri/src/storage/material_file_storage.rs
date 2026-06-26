use std::fs;
use std::io::{BufReader, Read};
use std::path::{Path, PathBuf};

use sha2::{Digest, Sha256};
use tauri::{AppHandle, Manager};

use crate::errors::app_error::AppErrorDto;

#[derive(Debug)]
pub struct ImportedMaterialFile {
    pub original_file_name: String,
    pub original_path: String,
    pub stored_file_path: String,
    pub file_size: i64,
    pub mime_type: Option<String>,
    pub sha256: String,
}

pub fn import_material_file(
    app: &AppHandle,
    case_id: &str,
    material_id: &str,
    source_file_path: &str,
) -> Result<ImportedMaterialFile, AppErrorDto> {
    let source_path = PathBuf::from(source_file_path);

    if !source_path.exists() {
        return Err(AppErrorDto::new(
            "ERR_FILE_NOT_FOUND",
            "Файл не найден.",
            Some(source_file_path.to_string()),
        ));
    }

    if !source_path.is_file() {
        return Err(AppErrorDto::new(
            "ERR_FILE_NOT_FILE",
            "Указанный путь не является файлом.",
            Some(source_file_path.to_string()),
        ));
    }

    let original_file_name = source_path
        .file_name()
        .and_then(|value| value.to_str())
        .ok_or_else(|| {
            AppErrorDto::new(
                "ERR_FILE_NAME",
                "Не удалось прочитать имя файла.",
                Some(source_file_path.to_string()),
            )
        })?
        .to_string();

    let file_size = fs::metadata(&source_path)
        .map_err(|err| {
            AppErrorDto::new(
                "ERR_FILE_METADATA",
                "Не удалось прочитать сведения о файле.",
                Some(err.to_string()),
            )
        })?
        .len() as i64;

    let storage_dir = get_material_storage_dir(app, case_id, material_id)?;

    fs::create_dir_all(&storage_dir).map_err(|err| {
        AppErrorDto::new(
            "ERR_STORAGE_CREATE",
            "Не удалось создать папку хранения материала.",
            Some(err.to_string()),
        )
    })?;

    let stored_path = storage_dir.join(&original_file_name);

    fs::copy(&source_path, &stored_path).map_err(|err| {
        AppErrorDto::new(
            "ERR_FILE_COPY",
            "Не удалось скопировать файл во внутреннее хранилище.",
            Some(err.to_string()),
        )
    })?;

    let sha256 = calculate_sha256(&stored_path)?;
    let mime_type = guess_mime_type(&stored_path);

    Ok(ImportedMaterialFile {
        original_file_name,
        original_path: source_path.to_string_lossy().to_string(),
        stored_file_path: stored_path.to_string_lossy().to_string(),
        file_size,
        mime_type,
        sha256,
    })
}

fn get_material_storage_dir(
    app: &AppHandle,
    case_id: &str,
    material_id: &str,
) -> Result<PathBuf, AppErrorDto> {
    let app_data_dir = app.path().app_data_dir().map_err(|err| {
        AppErrorDto::new(
            "ERR_APP_DATA_DIR",
            "Не удалось определить папку данных приложения.",
            Some(err.to_string()),
        )
    })?;

    Ok(app_data_dir
        .join("storage")
        .join("cases")
        .join(case_id)
        .join("materials")
        .join(material_id))
}

fn calculate_sha256(path: &Path) -> Result<String, AppErrorDto> {
    let file = fs::File::open(path).map_err(|err| {
        AppErrorDto::new(
            "ERR_FILE_READ",
            "Не удалось открыть файл для расчёта SHA-256.",
            Some(err.to_string()),
        )
    })?;

    let mut reader = BufReader::new(file);
    let mut hasher = Sha256::new();
    let mut buffer = [0_u8; 8192];

    loop {
        let read_count = reader.read(&mut buffer).map_err(|err| {
            AppErrorDto::new(
                "ERR_FILE_READ",
                "Не удалось прочитать файл для расчёта SHA-256.",
                Some(err.to_string()),
            )
        })?;

        if read_count == 0 {
            break;
        }

        hasher.update(&buffer[..read_count]);
    }

    Ok(format!("{:x}", hasher.finalize()))
}

fn guess_mime_type(path: &Path) -> Option<String> {
    let extension = path
        .extension()
        .and_then(|value| value.to_str())
        .unwrap_or_default()
        .to_lowercase();

    let mime = match extension.as_str() {
        "png" => "image/png",
        "jpg" | "jpeg" => "image/jpeg",
        "pdf" => "application/pdf",
        "docx" => "application/vnd.openxmlformats-officedocument.wordprocessingml.document",
        "xlsx" => "application/vnd.openxmlformats-officedocument.spreadsheetml.sheet",
        "txt" => "text/plain",
        "csv" => "text/csv",
        "html" | "htm" => "text/html",
        _ => return None,
    };

    Some(mime.to_string())
}
