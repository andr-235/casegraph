import { invokeCommand } from "../../../shared/api/invoke";

export type InitializeAppResponse = {
  appVersion: string;
  hasAdmin: boolean;
  databaseReady: boolean;
  offlineMode: true;
};

export function initializeApp(): Promise<InitializeAppResponse> {
  return invokeCommand<InitializeAppResponse>("initialize_app");
}