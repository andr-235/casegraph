export type AppErrorDto = {
  code: string;
  message: string;
  details?: string;
  technicalDetails?: Record<string, unknown>;
};

export type CommandResult<T> =
  | {
      ok: true;
      data: T;
    }
  | {
      ok: false;
      error: AppErrorDto;
    };

export class AppCommandError extends Error {
  code: string;
  details?: string;
  technicalDetails?: Record<string, unknown>;

  constructor(error: AppErrorDto) {
    super(error.message);
    this.name = "AppCommandError";
    this.code = error.code;
    this.details = error.details;
    this.technicalDetails = error.technicalDetails;
  }
}