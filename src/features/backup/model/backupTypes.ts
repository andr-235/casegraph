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
