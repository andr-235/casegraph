import { invokeCommand } from "../../../shared/api/invoke";
import type {
  CreateRelationPayload,
  CreateRelationResponse,
  GetRelationsResponse,
} from "../model/relationTypes";

export function getRelations(caseId: string): Promise<GetRelationsResponse> {
  return invokeCommand<GetRelationsResponse>("get_relations", {
    payload: { caseId },
  });
}

export function createRelation(
  payload: CreateRelationPayload,
): Promise<CreateRelationResponse> {
  return invokeCommand<CreateRelationResponse>("create_relation", {
    payload,
  });
}
