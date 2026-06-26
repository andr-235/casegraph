import { invokeCommand } from "../../../shared/api/invoke";
import type {
  CreateObjectPayload,
  CreateObjectResponse,
  GetObjectByIdResponse,
  GetObjectsResponse,
  UpdateObjectPayload,
  UpdateObjectResponse,
} from "../model/objectTypes";

export function getObjects(caseId: string): Promise<GetObjectsResponse> {
  return invokeCommand<GetObjectsResponse>("get_objects", {
    payload: { caseId },
  });
}

export function createObject(
  payload: CreateObjectPayload,
): Promise<CreateObjectResponse> {
  return invokeCommand<CreateObjectResponse>("create_object", {
    payload,
  });
}

export function getObjectById(
  caseId: string,
  objectId: string,
): Promise<GetObjectByIdResponse> {
  return invokeCommand<GetObjectByIdResponse>("get_object_by_id", {
    payload: { caseId, objectId },
  });
}

export function updateObject(
  payload: UpdateObjectPayload,
): Promise<UpdateObjectResponse> {
  return invokeCommand<UpdateObjectResponse>("update_object", {
    payload,
  });
}
