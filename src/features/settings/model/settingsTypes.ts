export type AppSettingsDto = {
  storagePath: string | null;
  docxDefaultTemplate: string;
  backupDefaultPath: string | null;
  integrityCheckOnStartup: boolean;
  viewerCanExportDocx: boolean;
  analystCanCreateBackup: boolean;
  auditStrictMode: boolean;
};
