import { invoke } from "@tauri-apps/api/core";
import type { CommandResult } from "./commandResult";
import { AppCommandError } from "./commandResult";

export async function invokeCommand<T>(
  command: string,
  payload?: Record<string, unknown>
): Promise<T> {
  const result = await invoke<CommandResult<T>>(command, payload ?? {});

  if (!result.ok) {
    throw new AppCommandError(result.error);
  }

  return result.data;
}