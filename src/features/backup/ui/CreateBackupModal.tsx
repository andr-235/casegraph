import { useState } from "react";

import {
  chooseBackupFolder,
  createBackup,
} from "../api/backupApi";
import type { CreateBackupResponse } from "../model/backupTypes";

type CreateBackupModalProps = {
  open: boolean;
  onClose: () => void;
  onCreated: (result: CreateBackupResponse) => void;
};

export function CreateBackupModal({
  open,
  onClose,
  onCreated,
}: CreateBackupModalProps) {
  const [outputFolderPath, setOutputFolderPath] = useState("");
  const [includeExports, setIncludeExports] = useState(true);
  const [includeAuditLog, setIncludeAuditLog] = useState(true);
  const [includeTemplates, setIncludeTemplates] = useState(true);
  const [isPickingFolder, setIsPickingFolder] = useState(false);
  const [isCreating, setIsCreating] = useState(false);
  const [errorMessage, setErrorMessage] = useState<string | null>(null);

  if (!open) {
    return null;
  }

  async function handleChooseFolder() {
    setIsPickingFolder(true);
    setErrorMessage(null);

    try {
      const result = await chooseBackupFolder();

      if (result.folderPath) {
        setOutputFolderPath(result.folderPath);
      }
    } catch (error) {
      setErrorMessage(
        error instanceof Error
          ? error.message
          : "Не удалось выбрать папку backup.",
      );
    } finally {
      setIsPickingFolder(false);
    }
  }

  async function handleCreate() {
    if (!outputFolderPath.trim()) {
      setErrorMessage("Выбери папку для сохранения backup.");
      return;
    }

    setIsCreating(true);
    setErrorMessage(null);

    try {
      const result = await createBackup({
        backupType: "full",
        outputFolderPath,
        includeExports,
        includeAuditLog,
        includeTemplates,
      });

      onCreated(result);
      onClose();
    } catch (error) {
      setErrorMessage(
        error instanceof Error
          ? error.message
          : "Не удалось создать backup.",
      );
    } finally {
      setIsCreating(false);
    }
  }

  return (
    <div className="modal-backdrop" role="presentation">
      <div className="modal" role="dialog" aria-modal="true">
        <div className="modal-header">
          <h2>Создать резервную копию</h2>
          <button type="button" onClick={onClose} disabled={isCreating}>
            ×
          </button>
        </div>

        <div className="modal-body">
          <div className="form-field">
            <label>Тип backup</label>
            <input value="Полный backup" disabled />
            <p className="field-hint">
              В этом срезе создаётся полный локальный архив приложения.
            </p>
          </div>

          <div className="form-field">
            <label>Папка сохранения</label>
            <div className="inline-control">
              <input value={outputFolderPath} readOnly placeholder="Папка не выбрана" />
              <button
                type="button"
                onClick={handleChooseFolder}
                disabled={isPickingFolder || isCreating}
              >
                {isPickingFolder ? "Выбор…" : "Выбрать"}
              </button>
            </div>
          </div>

          <label className="checkbox-row">
            <input
              type="checkbox"
              checked={includeExports}
              onChange={(event) => setIncludeExports(event.target.checked)}
              disabled={isCreating}
            />
            Включить DOCX exports
          </label>

          <label className="checkbox-row">
            <input
              type="checkbox"
              checked={includeAuditLog}
              onChange={(event) => setIncludeAuditLog(event.target.checked)}
              disabled={isCreating}
            />
            Включить audit log
          </label>

          <label className="checkbox-row">
            <input
              type="checkbox"
              checked={includeTemplates}
              onChange={(event) => setIncludeTemplates(event.target.checked)}
              disabled={isCreating}
            />
            Включить DOCX templates
          </label>

          {errorMessage && <div className="error-state">{errorMessage}</div>}
        </div>

        <div className="modal-footer">
          <button type="button" onClick={onClose} disabled={isCreating}>
            Отмена
          </button>
          <button
            type="button"
            onClick={handleCreate}
            disabled={isCreating || !outputFolderPath.trim()}
          >
            {isCreating ? "Создание…" : "Создать backup"}
          </button>
        </div>
      </div>
    </div>
  );
}
