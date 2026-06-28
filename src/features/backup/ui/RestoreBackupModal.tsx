import { useState } from "react";

import {
  chooseRestoreBackupFile,
  restoreBackupPreflight,
} from "../api/backupApi";
import type {
  BackupHistoryItemDto,
  RestoreBackupPreflightResponse,
} from "../model/backupTypes";

type RestoreBackupModalProps = {
  open: boolean;
  backup: BackupHistoryItemDto | null;
  onClose: () => void;
  onPreflightComplete: (result: RestoreBackupPreflightResponse) => void;
};

export function RestoreBackupModal({
  open,
  backup,
  onClose,
  onPreflightComplete,
}: RestoreBackupModalProps) {
  const [filePath, setFilePath] = useState("");
  const [result, setResult] = useState<RestoreBackupPreflightResponse | null>(null);
  const [isPickingFile, setIsPickingFile] = useState(false);
  const [isChecking, setIsChecking] = useState(false);
  const [errorMessage, setErrorMessage] = useState<string | null>(null);

  if (!open) {
    return null;
  }

  const isHistoryMode = Boolean(backup);
  const canRunPreflight = isHistoryMode || filePath.trim().length > 0;

  async function handleChooseFile() {
    setIsPickingFile(true);
    setErrorMessage(null);

    try {
      const response = await chooseRestoreBackupFile();

      if (response.filePath) {
        setFilePath(response.filePath);
      }
    } catch (error) {
      setErrorMessage(
        error instanceof Error
          ? error.message
          : "Не удалось выбрать backup-файл.",
      );
    } finally {
      setIsPickingFile(false);
    }
  }

  async function handleRunPreflight() {
    setIsChecking(true);
    setErrorMessage(null);
    setResult(null);

    try {
      const response = await restoreBackupPreflight({
        backupId: backup?.id ?? null,
        filePath: backup ? null : filePath,
      });

      setResult(response);
      onPreflightComplete(response);
    } catch (error) {
      setErrorMessage(
        error instanceof Error
          ? error.message
          : "Не удалось выполнить preflight restore.",
      );
    } finally {
      setIsChecking(false);
    }
  }

  return (
    <div className="modal-backdrop" role="presentation">
      <div className="modal modal-wide" role="dialog" aria-modal="true">
        <div className="modal-header">
          <h2>Проверка восстановления</h2>
          <button type="button" onClick={onClose} disabled={isChecking}>
            ×
          </button>
        </div>

        <div className="modal-body">
          <div className="warning-state">
            <strong>Восстановление пока не выполняется.</strong>
            <p>
              На этом шаге система только проверяет backup, совместимость версий
              и возможность безопасного восстановления. Перед реальным восстановлением
              будет создан safety backup.
            </p>
          </div>

          {backup ? (
            <div className="info-panel">
              <div>Backup: {backup.backupCode}</div>
              <div>Файл: {backup.fileName}</div>
              <div>Статус: {backup.status}</div>
            </div>
          ) : (
            <div className="form-field">
              <label>Backup ZIP</label>
              <div className="inline-control">
                <input value={filePath} readOnly placeholder="Файл не выбран" />
                <button
                  type="button"
                  onClick={handleChooseFile}
                  disabled={isPickingFile || isChecking}
                >
                  {isPickingFile ? "Выбор…" : "Выбрать файл"}
                </button>
              </div>
            </div>
          )}

          {errorMessage && <div className="error-state">{errorMessage}</div>}

          {result && (
            <div className={result.canRestore ? "success-state" : "error-state"}>
              <h3>
                {result.canRestore
                  ? "Backup готов к следующему шагу восстановления"
                  : "Backup нельзя восстановить"}
              </h3>

              <dl className="definition-list">
                <dt>Файл</dt>
                <dd>{result.fileName}</dd>

                <dt>Backup code</dt>
                <dd>{result.backupCode ?? "—"}</dd>

                <dt>Тип backup</dt>
                <dd>{result.metadata.backupType}</dd>

                <dt>Версия приложения</dt>
                <dd>
                  {result.metadata.appVersion}
                  {!result.compatibility.appVersionOk && " · отличается от текущей"}
                </dd>

                <dt>Версия схемы</dt>
                <dd>
                  {result.metadata.schemaVersion}
                  {!result.compatibility.schemaVersionOk && " · несовместима"}
                </dd>

                <dt>Файлов в manifest</dt>
                <dd>{result.metadata.fileCount}</dd>

                <dt>SHA-256 архива</dt>
                <dd className="mono">{result.archiveSha256}</dd>

                <dt>Safety backup</dt>
                <dd>{result.requiresSafetyBackup ? "Обязателен перед restore" : "Не требуется"}</dd>
              </dl>

              {result.errors.length > 0 && (
                <div className="issue-list">
                  <h4>Ошибки</h4>
                  {result.errors.map((issue: RestoreBackupPreflightResponse["errors"][number]) => (
                    <div key={issue.code} className="issue-row issue-error">
                      <strong>{issue.code}</strong>
                      <span>{issue.message}</span>
                    </div>
                  ))}
                </div>
              )}

              {result.warnings.length > 0 && (
                <div className="issue-list">
                  <h4>Предупреждения</h4>
                  {result.warnings.map((issue: RestoreBackupPreflightResponse["warnings"][number]) => (
                    <div key={issue.code} className="issue-row issue-warning">
                      <strong>{issue.code}</strong>
                      <span>{issue.message}</span>
                    </div>
                  ))}
                </div>
              )}
            </div>
          )}
        </div>

        <div className="modal-footer">
          <button type="button" onClick={onClose} disabled={isChecking}>
            Закрыть
          </button>

          <button
            type="button"
            onClick={handleRunPreflight}
            disabled={!canRunPreflight || isChecking}
          >
            {isChecking ? "Проверка…" : "Проверить восстановление"}
          </button>

          <button type="button" disabled>
            Продолжить restore
          </button>
        </div>
      </div>
    </div>
  );
}
