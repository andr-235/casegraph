import { invokeCommand } from "../../../shared/api/invoke";
import type {
  CreateObjectPayload,
  CreateObjectResponse,
  GetObjectsResponse,
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
