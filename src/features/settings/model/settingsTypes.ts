export type AppSettingsDto = {
  docx: {
    defaultTemplate: string;
    defaultExportDir: string;
    includeMaterialsTable: boolean;
    includeSha256Table: boolean;
  };
  backup: {
    defaultBackupDir: string;
    safetyBackupBeforeRestore: boolean;
    verifyBackupAfterCreate: boolean;
  };
  integrity: {
    warnBeforeDocxExport: boolean;
    warnBeforeBackup: boolean;
  };
  access: {
    viewerCanExportDocx: boolean;
    analystCanCreateBackup: boolean;
  };
};

export type UpdateSettingsPayload = {
  docx: {
    defaultTemplate: string;
    defaultExportDir: string;
    includeMaterialsTable: boolean;
    includeSha256Table: boolean;
  };
  backup: {
    defaultBackupDir: string;
    safetyBackupBeforeRestore: boolean;
    verifyBackupAfterCreate: boolean;
  };
  integrity: {
    warnBeforeDocxExport: boolean;
    warnBeforeBackup: boolean;
  };
  access: {
    viewerCanExportDocx: boolean;
    analystCanCreateBackup: boolean;
  };
};
