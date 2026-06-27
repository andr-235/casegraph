import { invokeCommand } from "../../../shared/api/invoke";
import type {
  GetRolesResponse,
  GetUsersPayload,
  GetUsersResponse,
} from "../model/userTypes";

export function getUsers(payload: GetUsersPayload): Promise<GetUsersResponse> {
  return invokeCommand<GetUsersResponse>("get_users", { payload });
}

export function getRoles(): Promise<GetRolesResponse> {
  return invokeCommand<GetRolesResponse>("get_roles");
}
