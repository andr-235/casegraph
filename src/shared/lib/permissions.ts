import type { EffectivePermissionsDto } from "../../features/auth/model/effectivePermissionsTypes";
import type { ProtectedOperationKey } from "../security/protectedOperations";

export function can(
  permissions: EffectivePermissionsDto | null | undefined,
  operation: ProtectedOperationKey,
): boolean {
  return Boolean(permissions?.operations?.[operation]);
}
