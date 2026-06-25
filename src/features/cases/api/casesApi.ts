import { invokeCommand } from "../../../shared/api/invoke";
import type {
  CaseDto,
  CreateCasePayload,
  CreateCaseResponse,
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