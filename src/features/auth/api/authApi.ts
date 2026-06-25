import { invokeCommand } from "../../../shared/api/invoke";
import type {
  CreateFirstAdminPayload,
  CreateFirstAdminResponse,
} from "../model/authTypes";

export function createFirstAdmin(
  payload: CreateFirstAdminPayload
): Promise<CreateFirstAdminResponse> {
  return invokeCommand<CreateFirstAdminResponse>("create_first_admin", {
    payload,
  });
}