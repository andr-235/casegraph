import { invokeCommand } from "../../../shared/api/invoke";
import type {
  CreateFirstAdminPayload,
  CreateFirstAdminResponse,
  CurrentUserDto,
  LoginPayload,
  LoginResponse,
} from "../model/authTypes";

export function createFirstAdmin(
  payload: CreateFirstAdminPayload
): Promise<CreateFirstAdminResponse> {
  return invokeCommand<CreateFirstAdminResponse>("create_first_admin", {
    payload,
  });
}

export function login(payload: LoginPayload): Promise<LoginResponse> {
  return invokeCommand<LoginResponse>("login", {
    payload,
  });
}

export function getCurrentUser(): Promise<CurrentUserDto | null> {
  return invokeCommand<CurrentUserDto | null>("get_current_user");
}

export function logout(): Promise<boolean> {
  return invokeCommand<boolean>("logout");
}