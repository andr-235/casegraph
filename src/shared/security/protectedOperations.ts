export const protectedOperations = {
  caseCreate: "case.create",
  caseRead: "case.read",
  caseUpdate: "case.update",

  materialImport: "material.import",
  materialRead: "material.read",
  materialUpdate: "material.update",

  objectCreate: "object.create",
  objectRead: "object.read",
  objectUpdate: "object.update",

  relationCreate: "relation.create",
  relationRead: "relation.read",
  relationUpdate: "relation.update",

  timelineCreate: "timeline.create",
  timelineRead: "timeline.read",
  timelineUpdate: "timeline.update",

  reportGenerate: "report.generate",
  reportRead: "report.read",
  reportUpdate: "report.update",

  docxExport: "docx.export",

  auditRead: "audit.read",
  userManage: "user.manage",

  settingsRead: "settings.read",
  settingsUpdate: "settings.update",

  backupRead: "backup.read",
  backupCreate: "backup.create",
  backupVerify: "backup.verify",
  backupRestore: "backup.restore",

  integrityRead: "integrity.read",
  integrityRun: "integrity.run",
} as const;

export type ProtectedOperationKey =
  (typeof protectedOperations)[keyof typeof protectedOperations];
