import type { ExportAuditLogPayload, GetAuditLogsPayload } from "./auditTypes";

export type AuditLogFilters = {
  action: string;
  result: string;
  severity: string;
  userId: string;
  dateFrom: string;
  dateTo: string;
};

export const emptyAuditLogFilters: AuditLogFilters = {
  action: "",
  result: "",
  severity: "",
  userId: "",
  dateFrom: "",
  dateTo: "",
};

type BuildAuditLogsPayloadParams = {
  filters: AuditLogFilters;
  page: number;
  pageSize: number;
  isAdministrator: boolean;
};

function emptyToUndefined(value: string) {
  const normalized = value.trim();
  return normalized.length > 0 ? normalized : undefined;
}

export function buildAuditLogsPayload({
  filters,
  page,
  pageSize,
  isAdministrator,
}: BuildAuditLogsPayloadParams): GetAuditLogsPayload {
  return {
    action: emptyToUndefined(filters.action),
    result: emptyToUndefined(filters.result),
    severity: emptyToUndefined(filters.severity),
    userId:
      isAdministrator && filters.userId
        ? emptyToUndefined(filters.userId)
        : undefined,
    dateFrom: emptyToUndefined(filters.dateFrom),
    dateTo: emptyToUndefined(filters.dateTo),
    page,
    pageSize,
  };
}

type BuildAuditLogExportPayloadParams = {
  filters: AuditLogFilters;
  isAdministrator: boolean;
};

export function buildAuditLogExportPayload({
  filters,
  isAdministrator,
}: BuildAuditLogExportPayloadParams): ExportAuditLogPayload {
  return {
    action: emptyToUndefined(filters.action),
    result: emptyToUndefined(filters.result),
    severity: emptyToUndefined(filters.severity),
    userId:
      isAdministrator && filters.userId
        ? emptyToUndefined(filters.userId)
        : undefined,
    dateFrom: emptyToUndefined(filters.dateFrom),
    dateTo: emptyToUndefined(filters.dateTo),
  };
}
