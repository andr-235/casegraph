export type RestoreRecoveryAction =
  | "none"
  | "cleanupCompletedRestore"
  | "rollbackInterruptedRestore"
  | "manualReviewRequired";

export type RestoreRecoveryStatus = {
  recoveryRequired: boolean;
  operationId: string | null;
  phase: string | null;
  restoreBackupCode: string | null;
  safetyBackupCode: string | null;
  startedAt: string | null;
  updatedAt: string | null;
  lastErrorCode: string | null;
  recommendedAction: RestoreRecoveryAction;
};

export type ResolveRestoreRecoveryPayload = {
  action: "cleanup_completed_restore" | "rollback_interrupted_restore";
  confirmationPhrase: string;
};

export type ResolveRestoreRecoveryResponse = {
  resolved: boolean;
  action: string;
  requiresRestart: boolean;
  message: string;
};
