import { invokeCommand } from "../../../shared/api/invoke";
import type {
  GetAuditLogByIdPayload,
  GetAuditLogByIdResponse,
  GetAuditLogsPayload,
  GetAuditLogsResponse,
} from "../model/auditTypes";

export function getAuditLogs(payload: GetAuditLogsPayload) {
  return invokeCommand<GetAuditLogsResponse>("get_audit_logs", { payload });
}

export function getAuditLogById(payload: GetAuditLogByIdPayload) {
  return invokeCommand<GetAuditLogByIdResponse>("get_audit_log_by_id", {
    payload,
  });
}
