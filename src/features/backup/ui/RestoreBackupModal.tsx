import { useState } from "react";

import {
  chooseRestoreBackupFile,
  createRestoreSafetyBackup,
  restoreBackup,
  restoreBackupPreflight,
} from "../api/backupApi";
import type {
  BackupHistoryItemDto,
  CreateRestoreSafetyBackupResponse,
  RestoreBackupPreflightResponse,
  RestoreBackupResponse,
} from "../model/backupTypes";

type RestoreBackupModalProps = {
  open: boolean;
  backup: BackupHistoryItemDto | null;
  onClose: () => void;
  onPreflightComplete: (result: RestoreBackupPreflightResponse) => void;
  onRestoreComplete?: () => void;
};

export function RestoreBackupModal({
  open,
  backup,
  onClose,
  onPreflightComplete,
  onRestoreComplete,
}: RestoreBackupModalProps) {
  const [filePath, setFilePath] = useState("");
  const [result, setResult] = useState<RestoreBackupPreflightResponse | null>(null);
  const [isPickingFile, setIsPickingFile] = useState(false);
  const [isChecking, setIsChecking] = useState(false);
  const [errorMessage, setErrorMessage] = useState<string | null>(null);
  const [safetyBackup, setSafetyBackup] =
    useState<CreateRestoreSafetyBackupResponse | null>(null);
  const [isCreatingSafetyBackup, setIsCreatingSafetyBackup] = useState(false);
  const [safetyErrorMessage, setSafetyErrorMessage] = useState<string | null>(null);

  const [confirmationPhrase, setConfirmationPhrase] = useState("");
  const [isRestoring, setIsRestoring] = useState(false);
  const [restoreErrorMessage, setRestoreErrorMessage] = useState<string | null>(null);
  const [restoreResult, setRestoreResult] = useState<RestoreBackupResponse | null>(null);

  if (!open) {
    return null;
  }

  const isHistoryMode = Boolean(backup);
  const canRunPreflight = isHistoryMode || filePath.trim().length > 0;

  async function handleChooseFile() {
    setIsPickingFile(true);
    setErrorMessage(null);
    setResult(null);
    setSafetyBackup(null);
    setSafetyErrorMessage(null);

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

  async function handleCreateSafetyBackup() {
    if (!result || !result.canRestore) {
      return;
    }

    setIsCreatingSafetyBackup(true);
    setSafetyErrorMessage(null);
    setSafetyBackup(null);

    try {
      const response = await createRestoreSafetyBackup({
        restoreBackupId: backup?.id ?? result.backupId ?? null,
        restoreFilePath: backup ? null : filePath,
        restoreArchiveSha256: result.archiveSha256,
      });

      setSafetyBackup(response);
    } catch (error) {
      setSafetyErrorMessage(
        error instanceof Error
          ? error.message
          : "Не удалось создать safety backup.",
      );
    } finally {
      setIsCreatingSafetyBackup(false);
    }
  }

  async function handleRestore() {
    if (!result?.canRestore || !safetyBackup) {
      return;
    }

    setIsRestoring(true);
    setRestoreErrorMessage(null);
    setRestoreResult(null);

    try {
      const response = await restoreBackup({
        restoreBackupId: backup?.id ?? result.backupId ?? null,
        restoreFilePath: backup ? null : filePath,
        restoreArchiveSha256: result.archiveSha256,
        safetyBackupId: safetyBackup.safetyBackupId,
        safetyArchiveSha256: safetyBackup.safetyArchiveSha256,
        confirmationPhrase,
      });

      setRestoreResult(response);
      onRestoreComplete?.();
    } catch (error) {
      setRestoreErrorMessage(
        error instanceof Error
          ? error.message
          : "Не удалось выполнить восстановление.",
      );
    } finally {
      setIsRestoring(false);
    }
  }

  async function handleRunPreflight() {
    setIsChecking(true);
    setErrorMessage(null);
    setResult(null);
    setSafetyBackup(null);
    setSafetyErrorMessage(null);

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

              {result.canRestore && !safetyBackup && (
                <div className="warning-state" style={{ marginTop: "1rem" }}>
                  <h3>Требуется safety backup</h3>
                  <p>
                    Перед восстановлением нужно создать резервную копию текущего состояния
                    приложения. Без неё restore не будет доступен.
                  </p>

                  {safetyErrorMessage && (
                    <div className="error-state">{safetyErrorMessage}</div>
                  )}

                  <button
                    type="button"
                    onClick={handleCreateSafetyBackup}
                    disabled={isCreatingSafetyBackup}
                  >
                    {isCreatingSafetyBackup
                      ? "Создание safety backup…"
                      : "Создать safety backup"}
                  </button>
                </div>
              )}

              {safetyBackup && !restoreResult && (
                <div className="success-state" style={{ marginTop: "1rem" }}>
                  <h3>Safety backup создан</h3>

                  <dl className="definition-list">
                    <dt>Backup code</dt>
                    <dd>{safetyBackup.safetyBackupCode}</dd>

                    <dt>Файл</dt>
                    <dd>{safetyBackup.safetyFileName}</dd>

                    <dt>Размер</dt>
                    <dd>{safetyBackup.safetyFileSize} байт</dd>

                    <dt>SHA-256</dt>
                    <dd className="mono">{safetyBackup.safetyArchiveSha256}</dd>

                    <dt>Restore target</dt>
                    <dd>
                      {safetyBackup.restoreTarget.backupCode ??
                        safetyBackup.restoreTarget.fileName}
                    </dd>
                  </dl>
                </div>
              )}

              {safetyBackup && !restoreResult && (
                <div className="danger-state" style={{ marginTop: "1rem" }}>
                  <h3>Финальное подтверждение восстановления</h3>

                  <p>
                    Восстановление заменит текущую базу данных и локальные файлы приложения
                    содержимым выбранного backup. Перед операцией создан safety backup.
                  </p>

                  <p>
                    Для продолжения введите: <strong>ВОССТАНОВИТЬ</strong>
                  </p>

                  <input
                    value={confirmationPhrase}
                    onChange={(event) => setConfirmationPhrase(event.target.value)}
                    placeholder="ВОССТАНОВИТЬ"
                    disabled={isRestoring}
                  />

                  {restoreErrorMessage && (
                    <div className="error-state">{restoreErrorMessage}</div>
                  )}

                  <button
                    type="button"
                    className="danger-button"
                    onClick={handleRestore}
                    disabled={confirmationPhrase !== "ВОССТАНОВИТЬ" || isRestoring}
                  >
                    {isRestoring ? "Восстановление…" : "Выполнить restore"}
                  </button>
                </div>
              )}

              {restoreResult && (
                <div className="success-state" style={{ marginTop: "1rem" }}>
                  <h3>Восстановление выполнено</h3>

                  <p>{restoreResult.message}</p>

                  <dl className="definition-list">
                    <dt>Restore operation</dt>
                    <dd>{restoreResult.restoreOperationId}</dd>

                    <dt>Восстановленный backup</dt>
                    <dd>{restoreResult.restoredBackupCode ?? "Внешний backup-файл"}</dd>

                    <dt>Safety backup</dt>
                    <dd>{restoreResult.safetyBackupCode}</dd>

                    <dt>Требуется перезапуск</dt>
                    <dd>{restoreResult.requiresRestart ? "Да" : "Нет"}</dd>
                  </dl>

                  <div className="warning-state">
                    Закройте CaseGraph и запустите приложение заново.
                  </div>
                </div>
              )}
            </div>
          )}
        </div>

        <div className="modal-footer">
          <button type="button" onClick={onClose} disabled={isChecking || isCreatingSafetyBackup || isRestoring}>
            Закрыть
          </button>

          <button
            type="button"
            onClick={handleRunPreflight}
            disabled={!canRunPreflight || isChecking || isCreatingSafetyBackup || isRestoring || Boolean(restoreResult)}
          >
            {isChecking ? "Проверка…" : "Проверить восстановление"}
          </button>

          <button
            type="button"
            onClick={handleCreateSafetyBackup}
            disabled={!result?.canRestore || Boolean(safetyBackup) || isCreatingSafetyBackup || isRestoring || Boolean(restoreResult)}
          >
            {isCreatingSafetyBackup ? "Создание…" : "Создать safety backup"}
          </button>

          <button
            type="button"
            className="danger-button"
            onClick={handleRestore}
            disabled={!safetyBackup || confirmationPhrase !== "ВОССТАНОВИТЬ" || isRestoring || Boolean(restoreResult)}
          >
            {isRestoring ? "Восстановление…" : "Выполнить restore"}
          </button>
        </div>
      </div>
    </div>
  );
}
