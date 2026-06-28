import { useCallback, useEffect, useState } from "react";

import type { EffectivePermissionsDto } from "../../features/auth/model/effectivePermissionsTypes";
import { getBackupHistory } from "../../features/backup/api/backupApi";
import type {
  BackupHistoryItemDto,
  CreateBackupResponse,
  RestoreBackupPreflightResponse,
  VerifyBackupResponse,
} from "../../features/backup/model/backupTypes";
import { CreateBackupModal } from "../../features/backup/ui/CreateBackupModal";
import { RestoreBackupModal } from "../../features/backup/ui/RestoreBackupModal";
import { VerifyBackupModal } from "../../features/backup/ui/VerifyBackupModal";
import { can } from "../../shared/lib/permissions";
import { protectedOperations } from "../../shared/security/protectedOperations";

function formatFileSize(size?: number | null): string {
  if (!size) return "—";

  if (size < 1024) return `${size} Б`;
  if (size < 1024 * 1024) return `${Math.round(size / 1024)} КБ`;

  return `${(size / 1024 / 1024).toFixed(1)} МБ`;
}

function statusLabel(status: BackupHistoryItemDto["status"]): string {
  switch (status) {
    case "created":
      return "Создан";
    case "verified":
      return "Проверен";
    case "failed":
      return "Ошибка";
    case "restored":
      return "Восстановлен";
    default:
      return status;
  }
}

function typeLabel(type: BackupHistoryItemDto["backupType"]): string {
  switch (type) {
    case "full":
      return "Полный";
    case "case":
      return "По делу";
    case "safety":
      return "Safety";
    default:
      return type;
  }
}

type BackupPageProps = {
  permissions?: EffectivePermissionsDto | null;
  onBack: () => void;
};

export function BackupPage({ permissions, onBack }: BackupPageProps) {
  const canReadBackup = can(permissions, protectedOperations.backupRead);
  const canCreateBackup = can(permissions, protectedOperations.backupCreate);
  const canVerifyBackup = can(permissions, protectedOperations.backupVerify);
  const canRestoreBackup = can(permissions, protectedOperations.backupRestore);

  const [items, setItems] = useState<BackupHistoryItemDto[]>([]);
  const [isLoading, setIsLoading] = useState(true);
  const [errorMessage, setErrorMessage] = useState<string | null>(null);
  const [successMessage, setSuccessMessage] = useState<string | null>(null);
  const [isCreateModalOpen, setCreateModalOpen] = useState(false);
  const [isVerifyModalOpen, setVerifyModalOpen] = useState(false);
  const [verifyTarget, setVerifyTarget] = useState<BackupHistoryItemDto | null>(null);
  const [isRestoreModalOpen, setRestoreModalOpen] = useState(false);
  const [restoreTarget, setRestoreTarget] = useState<BackupHistoryItemDto | null>(null);

  const actionsDisabledReason =
    !canCreateBackup
      ? "Создание резервной копии недоступно для текущей роли или политики доступа."
      : null;

  const loadHistory = useCallback(async () => {
    if (!canReadBackup) {
      setIsLoading(false);
      setErrorMessage("Недостаточно прав для просмотра резервного копирования.");
      return;
    }

    setIsLoading(true);
    setErrorMessage(null);

    try {
      const response = await getBackupHistory();
      setItems(response);
    } catch (error) {
      setErrorMessage(
        error instanceof Error
          ? error.message
          : "Не удалось загрузить историю резервного копирования.",
      );
    } finally {
      setIsLoading(false);
    }
  }, [canReadBackup]);

  useEffect(() => {
    loadHistory();
  }, [loadHistory]);

  function handleBackupCreated(result: CreateBackupResponse) {
    setSuccessMessage(`Backup ${result.backupCode} создан: ${result.fileName}`);
    void loadHistory();
  }

  function handleOpenVerifyBackup(item: BackupHistoryItemDto) {
    setVerifyTarget(item);
    setVerifyModalOpen(true);
  }

  function handleOpenVerifyExternalFile() {
    setVerifyTarget(null);
    setVerifyModalOpen(true);
  }

  function handleVerified(result: VerifyBackupResponse) {
    setSuccessMessage(
      result.isValid
        ? `Backup ${result.backupCode ?? result.fileName} успешно проверен.`
        : `Backup ${result.backupCode ?? result.fileName} не прошёл проверку.`,
    );

    void loadHistory();
  }

  function handleOpenRestoreFromHistory(item: BackupHistoryItemDto) {
    setRestoreTarget(item);
    setRestoreModalOpen(true);
  }

  function handleOpenRestoreFromFile() {
    setRestoreTarget(null);
    setRestoreModalOpen(true);
  }

  function handleRestorePreflightComplete(result: RestoreBackupPreflightResponse) {
    setSuccessMessage(
      result.canRestore
        ? `Backup ${result.backupCode ?? result.fileName} прошёл preflight restore.`
        : `Backup ${result.backupCode ?? result.fileName} нельзя восстановить.`,
    );
  }

  if (!canReadBackup) {
    return (
      <section className="page">
        <h1>Резервное копирование</h1>
        <div className="empty-state">
          Недостаточно прав для просмотра раздела.
        </div>
      </section>
    );
  }

  return (
    <section className="page">
      <div className="page-header">
        <div>
          <h1>Резервное копирование</h1>
          <p className="page-description">
            История локальных резервных копий CaseGraph.
          </p>
        </div>

        <div className="page-actions">
          <button
            type="button"
            disabled={!canCreateBackup}
            title={actionsDisabledReason ?? undefined}
            onClick={() => setCreateModalOpen(true)}
          >
            Создать backup
          </button>

          <button
            type="button"
            disabled={!canVerifyBackup}
            onClick={handleOpenVerifyExternalFile}
          >
            Проверить backup
          </button>

          <button
            type="button"
            disabled={!canRestoreBackup}
            onClick={handleOpenRestoreFromFile}
          >
            Восстановить
          </button>

          <button type="button" onClick={onBack}>
            Назад
          </button>
        </div>
      </div>

      {successMessage && (
        <div className="success-state">{successMessage}</div>
      )}

      {isLoading && <div className="loading-state">Загрузка истории backup…</div>}

      {errorMessage && !isLoading && (
        <div className="error-state">{errorMessage}</div>
      )}

      {!isLoading && !errorMessage && items.length === 0 && (
        <div className="empty-state">
          Резервные копии ещё не создавались.
        </div>
      )}

      {!isLoading && !errorMessage && items.length > 0 && (
        <table className="data-table">
          <thead>
            <tr>
              <th>Код</th>
              <th>Тип</th>
              <th>Статус</th>
              <th>Файл</th>
              <th>Размер</th>
              <th>Дело</th>
              <th>Создано</th>
              <th>Действия</th>
            </tr>
          </thead>
          <tbody>
            {items.map((item) => (
              <tr key={item.id}>
                <td>{item.backupCode}</td>
                <td>{typeLabel(item.backupType)}</td>
                <td>{statusLabel(item.status)}</td>
                <td>{item.fileName}</td>
                <td>{formatFileSize(item.fileSize)}</td>
                <td>{item.caseCode ?? "—"}</td>
                <td>{item.createdAt}</td>
                <td>
                  <button
                    type="button"
                    disabled={!canVerifyBackup}
                    onClick={() => handleOpenVerifyBackup(item)}
                  >
                    Проверить
                  </button>
                  <button
                    type="button"
                    disabled={!canRestoreBackup}
                    onClick={() => handleOpenRestoreFromHistory(item)}
                  >
                    Восстановить
                  </button>
                </td>
              </tr>
            ))}
          </tbody>
        </table>
      )}
      <CreateBackupModal
        open={isCreateModalOpen}
        onClose={() => setCreateModalOpen(false)}
        onCreated={handleBackupCreated}
      />

      <VerifyBackupModal
        open={isVerifyModalOpen}
        backup={verifyTarget}
        onClose={() => setVerifyModalOpen(false)}
        onVerified={handleVerified}
      />

      <RestoreBackupModal
        open={isRestoreModalOpen}
        backup={restoreTarget}
        onClose={() => setRestoreModalOpen(false)}
        onPreflightComplete={handleRestorePreflightComplete}
      />
    </section>
  );
}
