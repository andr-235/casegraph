use crate::backup::{
    BackupChecksumItemDto, BackupChecksumsDto, BackupManifestDto, BackupManifestItemDto,
    BackupMetadataDto,
};

pub struct BackupMetadataService;

impl BackupMetadataService {
    pub fn build_metadata(
        backup_id: &str,
        backup_code: &str,
        backup_type: &str,
        created_at: &str,
        created_by_user_id: &str,
        created_by_username: &str,
        created_by_role: &str,
        app_version: &str,
        schema_version: i64,
        safety_reason: Option<String>,
        restore_target_backup_id: Option<String>,
        restore_target_backup_code: Option<String>,
        restore_target_archive_sha256: Option<String>,
    ) -> BackupMetadataDto {
        BackupMetadataDto {
            backup_id: backup_id.to_owned(),
            backup_code: backup_code.to_owned(),
            backup_type: backup_type.to_owned(),
            created_at: created_at.to_owned(),
            created_by_user_id: created_by_user_id.to_owned(),
            created_by_username: created_by_username.to_owned(),
            created_by_role: created_by_role.to_owned(),
            app_version: app_version.to_owned(),
            schema_version,
            archive_sha256: None,
            safety_reason,
            restore_target_backup_id,
            restore_target_backup_code,
            restore_target_archive_sha256,
        }
    }

    pub fn build_manifest(files: Vec<BackupManifestItemDto>) -> BackupManifestDto {
        BackupManifestDto { files }
    }

    pub fn build_checksums(items: Vec<BackupChecksumItemDto>) -> BackupChecksumsDto {
        BackupChecksumsDto {
            algorithm: "SHA-256".to_owned(),
            items,
        }
    }
}
