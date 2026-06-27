import { invokeCommand } from "../../../shared/api/invoke";
import type {
  AppSettingsDto,
  UpdateSettingsPayload,
} from "../model/settingsTypes";

export function getSettings(): Promise<AppSettingsDto> {
  return invokeCommand<AppSettingsDto>("get_settings");
}

export function updateSettings(
  payload: UpdateSettingsPayload,
): Promise<AppSettingsDto> {
  return invokeCommand<AppSettingsDto>("update_settings", { payload });
}
