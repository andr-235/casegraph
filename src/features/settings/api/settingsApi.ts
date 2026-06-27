import { invokeCommand } from "../../../shared/api/invoke";
import type { AppSettingsDto } from "../model/settingsTypes";

export function getSettings(): Promise<AppSettingsDto> {
  return invokeCommand<AppSettingsDto>("get_settings");
}
