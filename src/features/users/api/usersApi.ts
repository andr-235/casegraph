import { invokeCommand } from "../../../shared/api/invoke";
import type {
  BlockUserPayload,
  BlockUserResponse,
  CreateUserPayload,
  CreateUserResponse,
  GetRolesResponse,
  GetUserByIdPayload,
  GetUserByIdResponse,
  GetUsersPayload,
  GetUsersResponse,
  ResetUserPasswordPayload,
  ResetUserPasswordResponse,
  UnblockUserPayload,
  UnblockUserResponse,
  UpdateUserPayload,
  UpdateUserResponse,
} from "../model/userTypes";

export function getUsers(payload: GetUsersPayload): Promise<GetUsersResponse> {
  return invokeCommand<GetUsersResponse>("get_users", { payload });
}

export function createUser(
  payload: CreateUserPayload,
): Promise<CreateUserResponse> {
  return invokeCommand<CreateUserResponse>("create_user", { payload });
}

export function blockUser(
  payload: BlockUserPayload,
): Promise<BlockUserResponse> {
  return invokeCommand<BlockUserResponse>("block_user", { payload });
}

export function unblockUser(
  payload: UnblockUserPayload,
): Promise<UnblockUserResponse> {
  return invokeCommand<UnblockUserResponse>("unblock_user", { payload });
}

export function getUserById(
  payload: GetUserByIdPayload,
): Promise<GetUserByIdResponse> {
  return invokeCommand<GetUserByIdResponse>("get_user_by_id", { payload });
}

export function updateUser(
  payload: UpdateUserPayload,
): Promise<UpdateUserResponse> {
  return invokeCommand<UpdateUserResponse>("update_user", { payload });
}

export function resetUserPassword(
  payload: ResetUserPasswordPayload,
): Promise<ResetUserPasswordResponse> {
  return invokeCommand<ResetUserPasswordResponse>("reset_user_password", {
    payload,
  });
}

export function getRoles(): Promise<GetRolesResponse> {
  return invokeCommand<GetRolesResponse>("get_roles");
}
