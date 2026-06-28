import { invokeCommand } from "../../../shared/api/invoke";
import type { BackupHistoryItemDto } from "../model/backupTypes";

export function getBackupHistory(): Promise<BackupHistoryItemDto[]> {
  return invokeCommand<BackupHistoryItemDto[]>("get_backup_history");
}
