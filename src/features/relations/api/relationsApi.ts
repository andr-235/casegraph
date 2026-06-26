import { invokeCommand } from "../../../shared/api/invoke";
import type {
  CreateRelationPayload,
  CreateRelationResponse,
  GetRelationByIdResponse,
  GetRelationsResponse,
  SoftDeleteRelationPayload,
  SoftDeleteRelationResponse,
  UpdateRelationPayload,
  UpdateRelationResponse,
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

export function getRelationById(
  caseId: string,
  relationId: string,
): Promise<GetRelationByIdResponse> {
  return invokeCommand<GetRelationByIdResponse>("get_relation_by_id", {
    payload: { caseId, relationId },
  });
}

export function softDeleteRelation(
  payload: SoftDeleteRelationPayload,
): Promise<SoftDeleteRelationResponse> {
  return invokeCommand<SoftDeleteRelationResponse>("soft_delete_relation", {
    payload,
  });
}

export function updateRelation(
  payload: UpdateRelationPayload,
): Promise<UpdateRelationResponse> {
  return invokeCommand<UpdateRelationResponse>("update_relation", {
    payload,
  });
}
