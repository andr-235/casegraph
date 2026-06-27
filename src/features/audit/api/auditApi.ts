import { invokeCommand } from "../../../shared/api/invoke";
import type {
  GetAuditLogsPayload,
  GetAuditLogsResponse,
} from "../model/auditTypes";

export function getAuditLogs(payload: GetAuditLogsPayload) {
  return invokeCommand<GetAuditLogsResponse>("get_audit_logs", { payload });
}
