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
