import { useState } from "react";

import {
  chooseBackupFile,
  verifyBackup,
} from "../api/backupApi";
import type {
  BackupHistoryItemDto,
  VerifyBackupResponse,
} from "../model/backupTypes";

type VerifyBackupModalProps = {
  open: boolean;
  backup: BackupHistoryItemDto | null;
  onClose: () => void;
  onVerified: (result: VerifyBackupResponse) => void;
};

export function VerifyBackupModal({
  open,
  backup,
  onClose,
  onVerified,
}: VerifyBackupModalProps) {
  const [filePath, setFilePath] = useState("");
  const [result, setResult] = useState<VerifyBackupResponse | null>(null);
  const [isPickingFile, setIsPickingFile] = useState(false);
  const [isVerifying, setIsVerifying] = useState(false);
  const [errorMessage, setErrorMessage] = useState<string | null>(null);

  if (!open) {
    return null;
  }

  const isHistoryMode = Boolean(backup);

  async function handleChooseFile() {
    setIsPickingFile(true);
    setErrorMessage(null);

    try {
      const response = await chooseBackupFile();

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

  async function handleVerify() {
    setIsVerifying(true);
    setErrorMessage(null);
    setResult(null);

    try {
      const response = await verifyBackup({
        backupId: backup?.id ?? null,
        filePath: backup ? null : filePath,
      });

      setResult(response);
      onVerified(response);
    } catch (error) {
      setErrorMessage(
        error instanceof Error
          ? error.message
          : "Не удалось проверить backup.",
      );
    } finally {
      setIsVerifying(false);
    }
  }

  const canVerify = isHistoryMode || filePath.trim().length > 0;

  return (
    <div className="modal-backdrop" role="presentation">
      <div className="modal modal-wide" role="dialog" aria-modal="true">
        <div className="modal-header">
          <h2>Проверить backup</h2>
          <button type="button" onClick={onClose} disabled={isVerifying}>
            ×
          </button>
        </div>

        <div className="modal-body">
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
                  disabled={isPickingFile || isVerifying}
                >
                  {isPickingFile ? "Выбор…" : "Выбрать файл"}
                </button>
              </div>
            </div>
          )}

          {errorMessage && <div className="error-state">{errorMessage}</div>}

          {result && (
            <div className={result.isValid ? "success-state" : "warning-state"}>
              <h3>
                {result.isValid
                  ? "Backup корректен"
                  : "Backup повреждён или неполный"}
              </h3>

              <dl className="definition-list">
                <dt>Файл</dt>
                <dd>{result.fileName}</dd>

                <dt>SHA-256 архива</dt>
                <dd className="mono">{result.archiveSha256}</dd>

                <dt>Проверено файлов</dt>
                <dd>{result.summary.checkedEntries}</dd>

                <dt>Ошибок</dt>
                <dd>{result.summary.errorCount}</dd>
              </dl>

              {result.issues.length > 0 && (
                <div className="issue-list">
                  {result.issues.map((
                    issue: { code: string; message: string; archivePath: string | null },
                    index: number,
                  ) => (
                    <div
                      key={`${issue.code}-${index}`}
                      className="issue-row"
                    >
                      <strong>{issue.code}</strong>
                      <span>{issue.message}</span>
                      {issue.archivePath && <code>{issue.archivePath}</code>}
                    </div>
                  ))}
                </div>
              )}
            </div>
          )}
        </div>

        <div className="modal-footer">
          <button type="button" onClick={onClose} disabled={isVerifying}>
            Закрыть
          </button>
          <button
            type="button"
            onClick={handleVerify}
            disabled={!canVerify || isVerifying}
          >
            {isVerifying ? "Проверка…" : "Проверить"}
          </button>
        </div>
      </div>
    </div>
  );
}
