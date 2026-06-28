use std::fs::{self, File};
use std::io::{Read, Write};
use std::path::{Path, PathBuf};

use sha2::{Digest, Sha256};
use walkdir::WalkDir;
use zip::write::FileOptions;

use crate::backup::{
    BackupChecksumItemDto, BackupManifestItemDto, BackupMetadataDto, BackupMetadataService,
};
use crate::errors::app_error::AppErrorDto;

pub struct BackupArchiveInput {
    pub output_file_path: PathBuf,
    pub database_path: PathBuf,
    pub data_dir: PathBuf,
    pub templates_dir: Option<PathBuf>,
    pub metadata: BackupMetadataDto,
    pub include_templates: bool,
    pub include_exports: bool,
    pub include_audit_log: bool,
}

pub struct BackupArchiveResult {
    pub file_size: i64,
    pub archive_sha256: String,
    pub metadata_json: String,
}

pub struct BackupArchiveWriter;

impl BackupArchiveWriter {
    pub fn create_full_backup(
        input: BackupArchiveInput,
    ) -> Result<BackupArchiveResult, AppErrorDto> {
        if let Some(parent) = input.output_file_path.parent() {
            fs::create_dir_all(parent).map_err(|err| AppErrorDto::filesystem(err.to_string()))?;
        }

        let file = File::create(&input.output_file_path)
            .map_err(|err| AppErrorDto::filesystem(err.to_string()))?;
        let mut zip = zip::ZipWriter::new(file);

        let options = FileOptions::default()
            .compression_method(zip::CompressionMethod::Deflated)
            .unix_permissions(0o644);

        let mut manifest_items = Vec::new();
        let mut checksum_items = Vec::new();

        Self::add_file(
            &mut zip,
            &input.database_path,
            "database/casegraph.sqlite",
            "database",
            true,
            options,
            &mut manifest_items,
            &mut checksum_items,
        )?;

        Self::add_dir(
            &mut zip,
            &input.data_dir,
            "data",
            options,
            &mut manifest_items,
            &mut checksum_items,
            &input,
        )?;

        if input.include_templates {
            if let Some(templates_dir) = &input.templates_dir {
                if templates_dir.exists() {
                    Self::add_dir(
                        &mut zip,
                        templates_dir,
                        "templates",
                        options,
                        &mut manifest_items,
                        &mut checksum_items,
                        &input,
                    )?;
                }
            }
        }

        let manifest = BackupMetadataService::build_manifest(manifest_items);
        let checksums = BackupMetadataService::build_checksums(checksum_items);

        let metadata_json = serde_json::to_string_pretty(&input.metadata)
            .map_err(|err| AppErrorDto::internal(err.to_string()))?;
        let manifest_json = serde_json::to_string_pretty(&manifest)
            .map_err(|err| AppErrorDto::internal(err.to_string()))?;
        let checksums_json = serde_json::to_string_pretty(&checksums)
            .map_err(|err| AppErrorDto::internal(err.to_string()))?;

        Self::add_json(
            &mut zip,
            "metadata/backup-metadata.json",
            &metadata_json,
            options,
        )?;
        Self::add_json(&mut zip, "metadata/manifest.json", &manifest_json, options)?;
        Self::add_json(
            &mut zip,
            "metadata/checksums.json",
            &checksums_json,
            options,
        )?;

        zip.finish()
            .map_err(|err| AppErrorDto::filesystem(err.to_string()))?;

        let file_size = fs::metadata(&input.output_file_path)
            .map_err(|err| AppErrorDto::filesystem(err.to_string()))?
            .len() as i64;

        let archive_sha256 = Self::sha256_file(&input.output_file_path)?;

        Ok(BackupArchiveResult {
            file_size,
            archive_sha256,
            metadata_json,
        })
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

    fn add_json<W: Write + std::io::Seek>(
        zip: &mut zip::ZipWriter<W>,
        archive_path: &str,
        content: &str,
        options: FileOptions,
    ) -> Result<(), AppErrorDto> {
        zip.start_file(archive_path, options)
            .map_err(|err| AppErrorDto::filesystem(err.to_string()))?;
        zip.write_all(content.as_bytes())
            .map_err(|err| AppErrorDto::filesystem(err.to_string()))?;
        Ok(())
    }

    fn add_file<W: Write + std::io::Seek>(
        zip: &mut zip::ZipWriter<W>,
        source_path: &Path,
        archive_path: &str,
        kind: &str,
        required: bool,
        options: FileOptions,
        manifest_items: &mut Vec<BackupManifestItemDto>,
        checksum_items: &mut Vec<BackupChecksumItemDto>,
    ) -> Result<(), AppErrorDto> {
        if !source_path.exists() {
            if required {
                return Err(AppErrorDto::new(
                    "ERR_FILE_NOT_FOUND",
                    "Обязательный файл backup не найден",
                    Some(source_path.to_string_lossy().to_string()),
                ));
            }

            return Ok(());
        }

        let mut file =
            File::open(source_path).map_err(|err| AppErrorDto::filesystem(err.to_string()))?;

        let normalized_path = archive_path.replace('\\', "/");
        zip.start_file(&normalized_path, options)
            .map_err(|err| AppErrorDto::filesystem(err.to_string()))?;

        let mut buffer = Vec::new();
        file.read_to_end(&mut buffer)
            .map_err(|err| AppErrorDto::filesystem(err.to_string()))?;
        zip.write_all(&buffer)
            .map_err(|err| AppErrorDto::filesystem(err.to_string()))?;

        let sha256 = Self::sha256_file(source_path)?;

        manifest_items.push(BackupManifestItemDto {
            path: normalized_path.clone(),
            kind: kind.to_owned(),
            required,
        });

        checksum_items.push(BackupChecksumItemDto {
            path: normalized_path,
            sha256,
        });

        Ok(())
    }

    #[allow(clippy::too_many_arguments)]
    fn add_dir<W: Write + std::io::Seek>(
        zip: &mut zip::ZipWriter<W>,
        source_dir: &Path,
        archive_prefix: &str,
        options: FileOptions,
        manifest_items: &mut Vec<BackupManifestItemDto>,
        checksum_items: &mut Vec<BackupChecksumItemDto>,
        input: &BackupArchiveInput,
    ) -> Result<(), AppErrorDto> {
        if !source_dir.exists() {
            return Ok(());
        }

        for entry in WalkDir::new(source_dir).into_iter().filter_map(Result::ok) {
            let path = entry.path();

            if !path.is_file() {
                continue;
            }

            let relative = match path.strip_prefix(source_dir) {
                Ok(value) => value,
                Err(_) => continue,
            };

            let relative_str = relative.to_string_lossy().replace('\\', "/");

            if !input.include_exports && relative_str.contains("/exports/") {
                continue;
            }

            if !input.include_audit_log && relative_str.contains("audit") {
                continue;
            }

            if relative_str.contains("/temp/") || relative_str.starts_with("temp/") {
                continue;
            }

            let archive_path = format!("{}/{}", archive_prefix, relative_str);

            Self::add_file(
                zip,
                path,
                &archive_path,
                "data",
                false,
                options,
                manifest_items,
                checksum_items,
            )?;
        }

        Ok(())
    }
}
