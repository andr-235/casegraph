export type BackupType = "full" | "case" | "safety";

export type BackupStatus =
  | "created"
  | "verified"
  | "failed"
  | "restored";

export type BackupCreateType = "full";

export type CreateBackupPayload = {
  backupType: BackupCreateType;
  outputFolderPath: string;
  includeExports: boolean;
  includeAuditLog: boolean;
  includeTemplates: boolean;
};

export type CreateBackupResponse = {
  backupId: string;
  backupCode: string;
  fileName: string;
  fileSize: number;
  sha256: string;
  createdAt: string;
};

export type SelectBackupOutputFolderResponse = {
  folderPath: string | null;
};

export type BackupHistoryItemDto = {
  id: string;
  backupCode: string;
  backupType: BackupType;
  status: BackupStatus;
  fileName: string;
  fileSize?: number | null;
  sha256?: string | null;
  caseId?: string | null;
  caseCode?: string | null;
  appVersion: string;
  schemaVersion: number;
  createdBy: string;
  createdAt: string;
  verifiedAt?: string | null;
  restoredAt?: string | null;
};

export type SelectBackupFileResponse = {
  filePath: string | null;
};

export type VerifyBackupPayload = {
  backupId?: string | null;
  filePath?: string | null;
};

export type VerifyBackupResponse = {
  backupId: string | null;
  backupCode: string | null;
  fileName: string;
  archiveSha256: string;
  checkedAt: string;
  isValid: boolean;
  summary: BackupVerificationSummaryDto;
  issues: BackupVerificationIssueDto[];
};

export type BackupVerificationSummaryDto = {
  metadataOk: boolean;
  manifestOk: boolean;
  checksumsOk: boolean;
  totalManifestEntries: number;
  totalChecksumEntries: number;
  checkedEntries: number;
  missingEntries: number;
  mismatchedEntries: number;
  errorCount: number;
};

export type BackupVerificationIssueDto = {
  code: string;
  message: string;
  severity: "warning" | "error";
  archivePath: string | null;
};

export type SelectRestoreBackupFileResponse = {
  filePath: string | null;
};

export type RestoreBackupPreflightPayload = {
  backupId?: string | null;
  filePath?: string | null;
};

export type RestoreBackupPreflightResponse = {
  backupId: string | null;
  backupCode: string | null;
  fileName: string;
  archiveSha256: string;
  checkedAt: string;
  canRestore: boolean;
  requiresSafetyBackup: boolean;
  metadata: RestoreBackupMetadataPreviewDto;
  compatibility: RestoreCompatibilityDto;
  verification: BackupVerificationSummaryDto;
  warnings: RestorePreflightIssueDto[];
  errors: RestorePreflightIssueDto[];
};

export type RestoreBackupMetadataPreviewDto = {
  backupType: string;
  appVersion: string;
  schemaVersion: number;
  createdAt: string;
  createdBy: string | null;
  caseId: string | null;
  caseCode: string | null;
  fileCount: number;
};

export type RestoreCompatibilityDto = {
  appVersionOk: boolean;
  schemaVersionOk: boolean;
  backupTypeOk: boolean;
  currentAppVersion: string;
  backupAppVersion: string;
  currentSchemaVersion: number;
  backupSchemaVersion: number;
};

export type RestorePreflightIssueDto = {
  code: string;
  message: string;
  severity: "warning" | "error";
};
