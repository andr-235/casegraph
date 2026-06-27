import { invokeCommand } from "../../../shared/api/invoke";
import type {
  GetAuditActionsResponse,
  GetAuditLogByIdPayload,
  GetAuditLogByIdResponse,
  GetAuditLogsPayload,
  GetAuditLogsResponse,
  GetAuditUsersResponse,
} from "../model/auditTypes";

export function getAuditLogs(payload: GetAuditLogsPayload) {
  return invokeCommand<GetAuditLogsResponse>("get_audit_logs", { payload });
}

export function getAuditLogById(payload: GetAuditLogByIdPayload) {
  return invokeCommand<GetAuditLogByIdResponse>("get_audit_log_by_id", {
    payload,
  });
}

export function getAuditActions() {
  return invokeCommand<GetAuditActionsResponse>("get_audit_actions", {});
}

export function getAuditUsers() {
  return invokeCommand<GetAuditUsersResponse>("get_audit_users", {});
}
