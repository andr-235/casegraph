import { invokeCommand } from "../../../shared/api/invoke";
import type {
  BackupHistoryItemDto,
  CreateBackupPayload,
  CreateBackupResponse,
  SelectBackupOutputFolderResponse,
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
