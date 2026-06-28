import { invokeCommand } from "../../../shared/api/invoke";
import type { EffectivePermissionsDto } from "../model/effectivePermissionsTypes";

export async function getEffectivePermissions(): Promise<EffectivePermissionsDto> {
  return invokeCommand<EffectivePermissionsDto>("get_effective_permissions");
}
