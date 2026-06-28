import { useEffect, useState } from "react";

import type { EffectivePermissionsDto } from "../../features/auth/model/effectivePermissionsTypes";
import { getBackupHistory } from "../../features/backup/api/backupApi";
import type { BackupHistoryItemDto } from "../../features/backup/model/backupTypes";
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

  const actionsDisabledReason =
    !canCreateBackup
      ? "Создание резервной копии недоступно для текущей роли или политики доступа."
      : null;

  useEffect(() => {
    let cancelled = false;

    async function load() {
      if (!canReadBackup) {
        setIsLoading(false);
        setErrorMessage("Недостаточно прав для просмотра резервного копирования.");
        return;
      }

      setIsLoading(true);
      setErrorMessage(null);

      try {
        const response = await getBackupHistory();

        if (!cancelled) {
          setItems(response);
        }
      } catch (error) {
        if (!cancelled) {
          setErrorMessage(
            error instanceof Error
              ? error.message
              : "Не удалось загрузить историю резервного копирования.",
          );
        }
      } finally {
        if (!cancelled) {
          setIsLoading(false);
        }
      }
    }

    load();

    return () => {
      cancelled = true;
    };
  }, [canReadBackup]);

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
          <button type="button" disabled={!canCreateBackup} title={actionsDisabledReason ?? undefined}>
            Создать backup
          </button>

          <button type="button" disabled={!canVerifyBackup}>
            Проверить backup
          </button>

          <button type="button" disabled={!canRestoreBackup}>
            Восстановить
          </button>

          <button type="button" onClick={onBack}>
            Назад
          </button>
        </div>
      </div>

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
              </tr>
            ))}
          </tbody>
        </table>
      )}
    </section>
  );
}
