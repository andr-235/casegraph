import type { AppSettingsDto } from "../../features/settings/model/settingsTypes";
import type { CurrentUserDto } from "../../features/auth/model/authTypes";

export type ProtectedOperation =
  | "docx.export"
  | "backup.create"
  | "backup.restore"
  | "settings.read"
  | "settings.update"
  | "user.manage"
  | "audit.read";

export function canPerformOperation(
  user: CurrentUserDto,
  settings: AppSettingsDto | null,
  operation: ProtectedOperation,
): boolean {
  switch (operation) {
    case "docx.export":
      return canExportDocx(user, settings);

    case "backup.create":
      return canCreateBackup(user, settings);

    case "backup.restore":
    case "settings.read":
    case "settings.update":
    case "user.manage":
      return user.role === "administrator";

    case "audit.read":
      return user.role === "administrator" || user.role === "analyst";

    default:
      return false;
  }
}

function canExportDocx(
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

function canCreateBackup(
  user: CurrentUserDto,
  settings: AppSettingsDto | null,
): boolean {
  if (user.role === "administrator") return true;

  if (user.role === "analyst") {
    return settings?.access.analystCanCreateBackup === true;
  }

  return false;
}
