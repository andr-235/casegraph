import { invokeCommand } from "../../shared/api/invoke";

import type { RestoreRecoveryStatus } from "../../features/backup/model/restoreRecoveryTypes";

export type InitializeAppResponse = {
  appVersion: string;
  hasAdmin: boolean;
  databaseReady: boolean;
  offlineMode: true;
  restoreRecoveryStatus: RestoreRecoveryStatus;
};

export function initializeApp(): Promise<InitializeAppResponse> {
  return invokeCommand<InitializeAppResponse>("initialize_app");
}