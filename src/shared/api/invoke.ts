import { invoke } from "@tauri-apps/api/core";
import type { CommandResult } from "./commandResult";
import { AppCommandError, PASSWORD_CHANGE_REQUIRED_ERROR } from "./commandResult";

export async function invokeCommand<T>(
  command: string,
  payload?: Record<string, unknown>
): Promise<T> {
  const result = await invoke<CommandResult<T>>(command, payload ?? {});

  if (!result.ok) {
    if (result.error.code === PASSWORD_CHANGE_REQUIRED_ERROR) {
      window.dispatchEvent(new CustomEvent("password-change-required"));
    }
    throw new AppCommandError(result.error);
  }

  return result.data;
}