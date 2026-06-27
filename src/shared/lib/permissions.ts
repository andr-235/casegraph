import type { AppSettingsDto } from "../../features/settings/model/settingsTypes";
import type { CurrentUserDto } from "../../features/auth/model/authTypes";

export function canExportDocx(
  user: CurrentUserDto,
  settings: AppSettingsDto | null,
): boolean {
  if (user.role === "administrator") return true;
  if (user.role === "analyst") return true;

  if (user.role === "viewer") {
    return settings?.access.viewerCanExportDocx === true;
  }

  return false;
}

export function canCreateBackup(
  user: CurrentUserDto,
  settings: AppSettingsDto | null,
): boolean {
  if (user.role === "administrator") return true;

  if (user.role === "analyst") {
    return settings?.access.analystCanCreateBackup === true;
  }

  return false;
}

export function canRestoreBackup(user: CurrentUserDto): boolean {
  return user.role === "administrator";
}
