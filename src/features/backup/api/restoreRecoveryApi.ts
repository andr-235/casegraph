import { invokeCommand } from "../../../shared/api/invoke";
import type {
  ResolveRestoreRecoveryPayload,
  ResolveRestoreRecoveryResponse,
  RestoreRecoveryStatus,
} from "../model/restoreRecoveryTypes";

export function getRestoreRecoveryStatus(): Promise<RestoreRecoveryStatus> {
  return invokeCommand<RestoreRecoveryStatus>("get_restore_recovery_status");
}

export function resolveRestoreRecovery(
  payload: ResolveRestoreRecoveryPayload,
): Promise<ResolveRestoreRecoveryResponse> {
  return invokeCommand<ResolveRestoreRecoveryResponse>(
    "resolve_restore_recovery",
    { payload },
  );
}
