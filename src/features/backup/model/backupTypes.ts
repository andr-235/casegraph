export type BackupType = "full" | "case" | "safety";

export type BackupStatus =
  | "created"
  | "verified"
  | "failed"
  | "restored";

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
