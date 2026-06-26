import { invokeCommand } from "../../../shared/api/invoke";
import type {
  CaseDto,
  CreateCasePayload,
  CreateCaseResponse,
  UpdateCasePayload,
  UpdateCaseResponse,
  UpdateCaseStatusPayload,
  UpdateCaseStatusResponse,
} from "../model/caseTypes";
export function getCases(): Promise<CaseDto[]> {
  return invokeCommand<CaseDto[]>("get_cases");
}

export function createCase(
  payload: CreateCasePayload
): Promise<CreateCaseResponse> {
  return invokeCommand<CreateCaseResponse>("create_case", {
    payload,
  });
}

export function getCaseById(caseId: string): Promise<CaseDto> {
  return invokeCommand<CaseDto>("get_case_by_id", {
    payload: {
      caseId,
    },
  });
}

export function updateCase(
  payload: UpdateCasePayload
): Promise<UpdateCaseResponse> {
  return invokeCommand<UpdateCaseResponse>("update_case", {
    payload,
  });
}

export function updateCaseStatus(
  payload: UpdateCaseStatusPayload
): Promise<UpdateCaseStatusResponse> {
  return invokeCommand<UpdateCaseStatusResponse>("update_case_status", {
    payload,
  });
}