import { AppCommandError } from "../api/commandResult";

export function formatError(error: unknown): string {
  if (error instanceof AppCommandError) {
    return error.message;
  }

  if (error instanceof Error) {
    return error.message;
  }

  return String(error);
}
