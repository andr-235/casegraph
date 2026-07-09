export type CaseStatus =
  | "draft"
  | "in_progress"
  | "prepared"
  | "completed"
  | "archived";

export type CaseDto = {
  id: string;
  caseCode: string;
  title: string;
  subject: string;
  description: string;
  status: CaseStatus;
  periodStart: string | null;
  periodEnd: string | null;
  createdByUserId: string;
  createdAt: string;
  updatedAt: string;
};

export type CreateCasePayload = {
  title: string;
  subject: string;
  description?: string;
  periodStart?: string | null;
  periodEnd?: string | null;
};

export type CreateCaseResponse = {
  caseItem: CaseDto;
};

export type GetCaseByIdPayload = {
  caseId: string;
};

export type UpdateCasePayload = {
  caseId: string;
  title: string;
  subject: string;
  description?: string;
  periodStart?: string | null;
  periodEnd?: string | null;
};

export type UpdateCaseResponse = {
  caseItem: CaseDto;
};

export type EditableCaseStatus =
  | "draft"
  | "in_progress"
  | "prepared"
  | "completed";

export type UpdateCaseStatusPayload = {
  caseId: string;
  status: EditableCaseStatus;
};

export type UpdateCaseStatusResponse = {
  caseItem: CaseDto;
};

// ===================================================================
// Типы для workspace-редизайна (case summary / overview)
// ===================================================================

export type GetCaseSummaryPayload = {
  caseId: string;
};

export type GetCaseOverviewPayload = {
  caseId: string;
};

// Реэкспорт DTO из shared типов для удобства импорта
export type { CaseSummaryDto, ObjectPreviewDto, ActivityItemDto, CaseOverviewDto } from "../../../shared/types/workspaceTypes";