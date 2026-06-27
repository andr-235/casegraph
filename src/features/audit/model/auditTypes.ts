export type AuditResult = "success" | "error" | "denied";

export type AuditSeverity = "info" | "warning" | "error" | "critical";

export type AuditLogDto = {
  id: string;

  userId?: string | null;
  username: string;
  userRole: string;

  action: string;
  entityType: string;
  entityId?: string | null;
  caseId?: string | null;

  result: string;
  severity: string;

  oldValue?: string | null;
  newValue?: string | null;
  technicalDetails?: string | null;

  appVersion: string;
  createdAt: string;
};

export type GetAuditLogsPayload = {
  action?: string;
  result?: string;
  severity?: string;
  caseId?: string;
  entityType?: string;
  dateFrom?: string;
  dateTo?: string;
  page?: number;
  pageSize?: number;
};

export type GetAuditLogsResponse = {
  items: AuditLogDto[];
  total: number;
  page: number;
  pageSize: number;
};

export type AuditJsonValue =
  | null
  | string
  | number
  | boolean
  | AuditJsonValue[]
  | { [key: string]: AuditJsonValue };

export type AuditLogDetailsDto = Omit<
  AuditLogDto,
  "oldValue" | "newValue" | "technicalDetails"
> & {
  oldValue?: AuditJsonValue;
  newValue?: AuditJsonValue;
  technicalDetails?: AuditJsonValue;
};

export type GetAuditLogByIdPayload = {
  auditLogId: string;
};

export type GetAuditLogByIdResponse = {
  item: AuditLogDetailsDto;
};
