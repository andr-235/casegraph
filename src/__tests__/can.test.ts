import { describe, it, expect } from "vitest";
import { can } from "../shared/lib/permissions";
import { protectedOperations } from "../shared/security/protectedOperations";
import type { ProtectedOperationKey } from "../shared/security/protectedOperations";
import type { EffectivePermissionsDto } from "../features/auth/model/effectivePermissionsTypes";

const adminPermissions: EffectivePermissionsDto = {
  role: "administrator",
  mustChangePassword: false,
  operations: {
    "case.create": true,
    "case.read": true,
    "case.update": true,
    "object.create": true,
    "object.read": true,
    "object.update": true,
    "material.import": true,
    "material.read": true,
    "material.update": true,
    "relation.create": true,
    "relation.read": true,
    "relation.update": true,
    "timeline.create": true,
    "timeline.read": true,
    "timeline.update": true,
    "report.generate": true,
    "report.read": true,
    "report.update": true,
    "docx.export": true,
    "audit.read": true,
    "user.manage": true,
    "backup.create": true,
    "backup.read": true,
    "backup.restore": true,
    "settings.read": true,
    "settings.update": true,
  },
  policyFlags: { viewerCanExportDocx: false, analystCanCreateBackup: false },
};

const viewerPermissions: EffectivePermissionsDto = {
  role: "viewer",
  mustChangePassword: false,
  operations: {
    "case.read": true,
    "object.read": true,
    "material.read": true,
    "relation.read": true,
    "timeline.read": true,
    "report.read": true,
    "audit.read": true,
  },
  policyFlags: { viewerCanExportDocx: false, analystCanCreateBackup: false },
};

describe("can()", () => {
  it("returns true when operation is granted for admin", () => {
    expect(can(adminPermissions, protectedOperations.caseCreate)).toBe(true);
    expect(can(adminPermissions, protectedOperations.userManage)).toBe(true);
    expect(can(adminPermissions, protectedOperations.backupRestore)).toBe(true);
  });

  it("returns true when operation is granted for viewer", () => {
    expect(can(viewerPermissions, protectedOperations.caseRead)).toBe(true);
    expect(can(viewerPermissions, protectedOperations.objectRead)).toBe(true);
  });

  it("returns false when operation is not granted", () => {
    expect(can(viewerPermissions, protectedOperations.caseCreate)).toBe(false);
    expect(can(viewerPermissions, protectedOperations.userManage)).toBe(false);
    expect(can(viewerPermissions, protectedOperations.backupCreate)).toBe(false);
  });

  it("returns false when permissions is null", () => {
    expect(can(null, protectedOperations.caseRead)).toBe(false);
  });

  it("returns false when permissions is undefined", () => {
    expect(can(undefined, protectedOperations.caseRead)).toBe(false);
  });

  it("returns false when operations record is empty", () => {
    const empty: EffectivePermissionsDto = {
      role: "viewer",
      mustChangePassword: false,
      operations: {},
      policyFlags: { viewerCanExportDocx: false, analystCanCreateBackup: false },
    };
    expect(can(empty, protectedOperations.caseRead)).toBe(false);
  });

  it("handles unknown operation key gracefully", () => {
    const key = "unknown.op" as ProtectedOperationKey;
    expect(can(adminPermissions, key)).toBe(false);
  });
});
