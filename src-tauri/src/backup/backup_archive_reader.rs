use std::fs::File;
use std::io::{Read, Seek};
use std::path::Path;

use serde::{Deserialize, Serialize};
use sha2::{Digest, Sha256};
use zip::ZipArchive;

use crate::backup::{BackupChecksumsDto, BackupManifestDto, BackupMetadataDto};
use crate::errors::app_error::AppErrorDto;

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum BackupVerificationIssueSeverity {
    Warning,
    Error,
}

#[derive(Debug, Clone, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct BackupVerificationIssueDto {
    pub code: String,
    pub message: String,
    pub severity: BackupVerificationIssueSeverity,
    pub archive_path: Option<String>,
}

#[derive(Debug, Clone, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct BackupVerificationSummaryDto {
    pub metadata_ok: bool,
    pub manifest_ok: bool,
    pub checksums_ok: bool,
    pub total_manifest_entries: usize,
    pub total_checksum_entries: usize,
    pub checked_entries: usize,
    pub missing_entries: usize,
    pub mismatched_entries: usize,
    pub error_count: usize,
}

pub struct BackupArchiveVerification {
    pub archive_sha256: String,
    pub metadata: Option<BackupMetadataDto>,
    pub manifest: Option<BackupManifestDto>,
    pub checksums: Option<BackupChecksumsDto>,
    pub summary: BackupVerificationSummaryDto,
    pub issues: Vec<BackupVerificationIssueDto>,
}

pub struct BackupArchiveReader;

impl BackupArchiveReader {
    pub fn verify(path: &Path) -> Result<BackupArchiveVerification, AppErrorDto> {
        if !path.exists() {
            return Err(AppErrorDto::validation("Backup-файл не найден"));
        }

        if !path.is_file() {
            return Err(AppErrorDto::validation("Выбранный путь не является файлом"));
        }

        if path.extension().and_then(|value| value.to_str()) != Some("zip") {
            return Err(AppErrorDto::validation("Backup должен быть ZIP-архивом"));
        }

        let archive_sha256 = Self::sha256_file(path)?;

        let file = File::open(path).map_err(|err| AppErrorDto::filesystem(err.to_string()))?;
        let mut archive =
            ZipArchive::new(file).map_err(|_| AppErrorDto::validation("Не удалось открыть ZIP"))?;

        let mut issues = Vec::new();

        Self::validate_archive_entry_names(&mut archive, &mut issues);

        let metadata: Option<BackupMetadataDto> = Self::read_json_entry(
            &mut archive,
            "metadata/backup-metadata.json",
            "ERR_BACKUP_METADATA_MISSING",
            "В архиве отсутствует backup-metadata.json",
            &mut issues,
        );

        let manifest: Option<BackupManifestDto> = Self::read_json_entry(
            &mut archive,
            "metadata/manifest.json",
            "ERR_BACKUP_MANIFEST_MISSING",
            "В архиве отсутствует manifest.json",
            &mut issues,
        );

        let checksums: Option<BackupChecksumsDto> = Self::read_json_entry(
            &mut archive,
            "metadata/checksums.json",
            "ERR_BACKUP_CHECKSUMS_MISSING",
            "В архиве отсутствует checksums.json",
            &mut issues,
        );

        let mut total_manifest_entries = 0;
        let mut total_checksum_entries = 0;
        let mut checked_entries = 0;
        let mut missing_entries = 0;
        let mut mismatched_entries = 0;

        if let Some(manifest) = &manifest {
            total_manifest_entries = manifest.files.len();

            for item in &manifest.files {
                if archive.by_name(&item.path).is_err() {
                    missing_entries += 1;

                    issues.push(BackupVerificationIssueDto {
                        code: "ERR_BACKUP_MANIFEST_ENTRY_MISSING".to_owned(),
                        message: "Файл из manifest отсутствует в архиве".to_owned(),
                        severity: BackupVerificationIssueSeverity::Error,
                        archive_path: Some(item.path.clone()),
                    });
                }
            }
        }

        if let Some(checksums) = &checksums {
            total_checksum_entries = checksums.items.len();

            if checksums.algorithm.to_uppercase() != "SHA-256" {
                issues.push(BackupVerificationIssueDto {
                    code: "ERR_BACKUP_UNSUPPORTED_CHECKSUM_ALGORITHM".to_owned(),
                    message: "Неподдерживаемый алгоритм checksum".to_owned(),
                    severity: BackupVerificationIssueSeverity::Error,
                    archive_path: None,
                });
            }

            for item in &checksums.items {
                match Self::sha256_zip_entry(&mut archive, &item.path) {
                    Ok(actual_sha256) => {
                        checked_entries += 1;

                        if actual_sha256 != item.sha256 {
                            mismatched_entries += 1;

                            issues.push(BackupVerificationIssueDto {
                                code: "ERR_BACKUP_CHECKSUM_MISMATCH".to_owned(),
                                message: "SHA-256 файла внутри backup не совпадает".to_owned(),
                                severity: BackupVerificationIssueSeverity::Error,
                                archive_path: Some(item.path.clone()),
                            });
                        }
                    }
                    Err(_) => {
                        missing_entries += 1;

                        issues.push(BackupVerificationIssueDto {
                            code: "ERR_BACKUP_CHECKSUM_ENTRY_MISSING".to_owned(),
                            message: "Файл из checksums отсутствует в архиве".to_owned(),
                            severity: BackupVerificationIssueSeverity::Error,
                            archive_path: Some(item.path.clone()),
                        });
                    }
                }
            }
        }

        let error_count = issues
            .iter()
            .filter(|issue| matches!(issue.severity, BackupVerificationIssueSeverity::Error))
            .count();

        let summary = BackupVerificationSummaryDto {
            metadata_ok: metadata.is_some(),
            manifest_ok: manifest.is_some(),
            checksums_ok: checksums.is_some() && mismatched_entries == 0 && missing_entries == 0,
            total_manifest_entries,
            total_checksum_entries,
            checked_entries,
            missing_entries,
            mismatched_entries,
            error_count,
        };

        Ok(BackupArchiveVerification {
            archive_sha256,
            metadata,
            manifest,
            checksums,
            summary,
            issues,
        })
    }

    fn validate_archive_entry_names<R: Read + Seek>(
        archive: &mut ZipArchive<R>,
        issues: &mut Vec<BackupVerificationIssueDto>,
    ) {
        for index in 0..archive.len() {
            let entry = match archive.by_index(index) {
                Ok(entry) => entry,
                Err(_) => continue,
            };

            let name = entry.name().replace('\\', "/");

            if name.starts_with('/')
                || name.contains("../")
                || name.contains("..\\")
                || name.contains(':')
            {
                issues.push(BackupVerificationIssueDto {
                    code: "ERR_BACKUP_UNSAFE_ENTRY_NAME".to_owned(),
                    message: "Архив содержит небезопасное имя файла".to_owned(),
                    severity: BackupVerificationIssueSeverity::Error,
                    archive_path: Some(name),
                });
            }
        }
    }

    fn read_json_entry<T, R>(
        archive: &mut ZipArchive<R>,
        entry_path: &str,
        missing_code: &str,
        missing_message: &str,
        issues: &mut Vec<BackupVerificationIssueDto>,
    ) -> Option<T>
    where
        T: serde::de::DeserializeOwned,
        R: Read + Seek,
    {
        let mut file = match archive.by_name(entry_path) {
            Ok(file) => file,
            Err(_) => {
                issues.push(BackupVerificationIssueDto {
                    code: missing_code.to_owned(),
                    message: missing_message.to_owned(),
                    severity: BackupVerificationIssueSeverity::Error,
                    archive_path: Some(entry_path.to_owned()),
                });

                return None;
            }
        };

        let mut content = String::new();

        if file.read_to_string(&mut content).is_err() {
            issues.push(BackupVerificationIssueDto {
                code: "ERR_BACKUP_JSON_READ_FAILED".to_owned(),
                message: "Не удалось прочитать JSON-файл backup".to_owned(),
                severity: BackupVerificationIssueSeverity::Error,
                archive_path: Some(entry_path.to_owned()),
            });

            return None;
        }

        match serde_json::from_str::<T>(&content) {
            Ok(value) => Some(value),
            Err(_) => {
                issues.push(BackupVerificationIssueDto {
                    code: "ERR_BACKUP_JSON_INVALID".to_owned(),
                    message: "Некорректный JSON-файл backup".to_owned(),
                    severity: BackupVerificationIssueSeverity::Error,
                    archive_path: Some(entry_path.to_owned()),
                });

                None
            }
        }
    }

    fn sha256_zip_entry<R: Read + Seek>(
        archive: &mut ZipArchive<R>,
        entry_path: &str,
    ) -> Result<String, AppErrorDto> {
        let mut file = archive
            .by_name(entry_path)
            .map_err(|_| AppErrorDto::not_found("Файл из checksums отсутствует в архиве"))?;

        let mut hasher = Sha256::new();
        let mut buffer = [0_u8; 1024 * 64];

        loop {
            let read = file
                .read(&mut buffer)
                .map_err(|err| AppErrorDto::filesystem(err.to_string()))?;

            if read == 0 {
                break;
            }

            hasher.update(&buffer[..read]);
        }

        Ok(format!("{:x}", hasher.finalize()))
    }

    fn sha256_file(path: &Path) -> Result<String, AppErrorDto> {
        let file = File::open(path).map_err(|err| AppErrorDto::filesystem(err.to_string()))?;
        let mut reader = std::io::BufReader::new(file);
        let mut hasher = Sha256::new();
        let mut buffer = [0_u8; 1024 * 64];

        loop {
            let bytes_read = reader
                .read(&mut buffer)
                .map_err(|err| AppErrorDto::filesystem(err.to_string()))?;

            if bytes_read == 0 {
                break;
            }

            hasher.update(&buffer[..bytes_read]);
        }

        Ok(format!("{:x}", hasher.finalize()))
    }
}
