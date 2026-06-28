import type { ProtectedOperationKey } from "../../../shared/security/protectedOperations";

export type EffectivePolicyFlagsDto = {
  viewerCanExportDocx: boolean;
  analystCanCreateBackup: boolean;
};

export type EffectivePermissionsDto = {
  role: "administrator" | "analyst" | "viewer";
  mustChangePassword: boolean;
  operations: Partial<Record<ProtectedOperationKey, boolean>>;
  policyFlags: EffectivePolicyFlagsDto;
};
