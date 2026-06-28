import { invokeCommand } from "../../../shared/api/invoke";
import type {
  BackupHistoryItemDto,
  CreateBackupPayload,
  CreateBackupResponse,
  RestoreBackupPreflightPayload,
  RestoreBackupPreflightResponse,
  SelectBackupFileResponse,
  SelectBackupOutputFolderResponse,
  SelectRestoreBackupFileResponse,
  VerifyBackupPayload,
  VerifyBackupResponse,
} from "../model/backupTypes";

export function getBackupHistory(): Promise<BackupHistoryItemDto[]> {
  return invokeCommand<BackupHistoryItemDto[]>("get_backup_history");
}

export function chooseBackupFolder(): Promise<SelectBackupOutputFolderResponse> {
  return invokeCommand<SelectBackupOutputFolderResponse>("choose_backup_folder");
}

export function createBackup(
  payload: CreateBackupPayload,
): Promise<CreateBackupResponse> {
  return invokeCommand<CreateBackupResponse>("create_backup", { payload });
}

export function chooseBackupFile(): Promise<SelectBackupFileResponse> {
  return invokeCommand<SelectBackupFileResponse>("choose_backup_file");
}

export function verifyBackup(
  payload: VerifyBackupPayload,
): Promise<VerifyBackupResponse> {
  return invokeCommand<VerifyBackupResponse>("verify_backup", { payload });
}

export function chooseRestoreBackupFile(): Promise<SelectRestoreBackupFileResponse> {
  return invokeCommand<SelectRestoreBackupFileResponse>(
    "choose_restore_backup_file",
  );
}

export function restoreBackupPreflight(
  payload: RestoreBackupPreflightPayload,
): Promise<RestoreBackupPreflightResponse> {
  return invokeCommand<RestoreBackupPreflightResponse>(
    "restore_backup_preflight",
    { payload },
  );
}
