import { useEffect, useState } from "react";
import {
  chooseSettingsDirectory,
  getSettings,
  resetSettingsToDefaults,
  updateSettings,
} from "../../features/settings/api/settingsApi";
import type {
  AppSettingsDto,
  SettingsDirectoryTarget,
  UpdateSettingsPayload,
} from "../../features/settings/model/settingsTypes";
import type { EffectivePermissionsDto } from "../../features/auth/model/effectivePermissionsTypes";
import { ConfirmModal } from "../../shared/ui/ConfirmModal";
import { formatError } from "../../shared/lib/formatError";
import { can } from "../../shared/lib/permissions";
import { protectedOperations } from "../../shared/security/protectedOperations";

type SettingsPageProps = {
  permissions?: EffectivePermissionsDto | null;
  onReloadPermissions?: () => Promise<void>;
  onBack: () => void;
};

export function SettingsPage({ permissions, onReloadPermissions, onBack }: SettingsPageProps) {
  const [settings, setSettings] = useState<AppSettingsDto | null>(null);
  const [draft, setDraft] = useState<UpdateSettingsPayload | null>(null);
  const [isLoading, setIsLoading] = useState(true);
  const [isSaving, setIsSaving] = useState(false);
  const [isResetting, setIsResetting] = useState(false);
  const [isResetConfirmOpen, setIsResetConfirmOpen] = useState(false);
  const [errorMessage, setErrorMessage] = useState<string | null>(null);
  const [savedMessage, setSavedMessage] = useState<string | null>(null);
  const [pickingTarget, setPickingTarget] =
    useState<SettingsDirectoryTarget | null>(null);

  useEffect(() => {
    let isMounted = true;

    async function loadSettings() {
      try {
        setIsLoading(true);
        setErrorMessage(null);
        const data = await getSettings();
        if (isMounted) {
          setSettings(data);
          setDraft(toUpdatePayload(data));
        }
      } catch (error) {
        if (isMounted) {
          setErrorMessage(formatError(error));
        }
      } finally {
        if (isMounted) {
          setIsLoading(false);
        }
      }
    }

    void loadSettings();
    return () => {
      isMounted = false;
    };
  }, []);

  function toUpdatePayload(data: AppSettingsDto): UpdateSettingsPayload {
    return {
      docx: {
        defaultTemplate: data.docx.defaultTemplate,
        defaultExportDir: data.docx.defaultExportDir,
        includeMaterialsTable: data.docx.includeMaterialsTable,
        includeSha256Table: data.docx.includeSha256Table,
      },
      backup: {
        defaultBackupDir: data.backup.defaultBackupDir,
        safetyBackupBeforeRestore: data.backup.safetyBackupBeforeRestore,
        verifyBackupAfterCreate: data.backup.verifyBackupAfterCreate,
      },
      integrity: {
        warnBeforeDocxExport: data.integrity.warnBeforeDocxExport,
        warnBeforeBackup: data.integrity.warnBeforeBackup,
      },
      access: {
        viewerCanExportDocx: data.access.viewerCanExportDocx,
        analystCanCreateBackup: data.access.analystCanCreateBackup,
      },
    };
  }

  async function handleSave() {
    if (!draft) return;
    setIsSaving(true);
    setErrorMessage(null);
    setSavedMessage(null);
    try {
      const updated = await updateSettings(draft);
      setSettings(updated);
      setDraft(toUpdatePayload(updated));
      setSavedMessage("Настройки успешно сохранены.");

      if (onReloadPermissions) {
        await onReloadPermissions();
      }
    } catch (err) {
      setErrorMessage(formatError(err));
    } finally {
      setIsSaving(false);
    }
  }

  async function handlePickDirectory(target: SettingsDirectoryTarget) {
    if (!draft) return;
    setPickingTarget(target);
    setErrorMessage(null);
    setSavedMessage(null);
    try {
      const response = await chooseSettingsDirectory({ target });
      if (!response.path) return;
      if (target === "docxDefaultExportDir") {
        setDraft({ ...draft, docx: { ...draft.docx, defaultExportDir: response.path } });
        return;
      }
      if (target === "backupDefaultBackupDir") {
        setDraft({ ...draft, backup: { ...draft.backup, defaultBackupDir: response.path } });
      }
    } catch (err) {
      setErrorMessage(formatError(err));
    } finally {
      setPickingTarget(null);
    }
  }

  async function handleResetSettingsToDefaults() {
    setIsResetting(true);
    setErrorMessage(null);
    setSavedMessage(null);
    try {
      const nextSettings = await resetSettingsToDefaults();
      setSettings(nextSettings);
      setDraft(toUpdatePayload(nextSettings));
      setIsResetConfirmOpen(false);
      setSavedMessage("Настройки сброшены к значениям по умолчанию.");

      if (onReloadPermissions) {
        await onReloadPermissions();
      }
    } catch (err) {
      setErrorMessage(formatError(err));
    } finally {
      setIsResetting(false);
    }
  }

  const isDirty =
    settings !== null &&
    draft !== null &&
    JSON.stringify(toUpdatePayload(settings)) !== JSON.stringify(draft);

  const isAnyBusy = isSaving || isResetting || pickingTarget !== null || isLoading;

  if (isLoading) {
    return (
      <main className="page" style={{ padding: 32 }}>
        <h1>Настройки</h1>
        <p>Загрузка настроек…</p>
      </main>
    );
  }

  if (errorMessage && !settings) {
    return (
      <main className="page" style={{ padding: 32 }}>
        <h1>Настройки</h1>
        <div style={{ padding: 12, background: "#fde8e8", color: "#9b1c1c", borderRadius: 6, marginBottom: 16 }}>
          <strong>Не удалось загрузить настройки.</strong>
          <p>{errorMessage}</p>
        </div>
        <button type="button" onClick={onBack}>Назад к делам</button>
      </main>
    );
  }

  if (!settings || !draft) {
    return (
      <main className="page" style={{ padding: 32 }}>
        <h1>Настройки</h1>
        <p>Настройки не найдены.</p>
        <button type="button" onClick={onBack}>Назад к делам</button>
      </main>
    );
  }

  return (
    <main className="page" style={{ padding: 32 }}>
      {/* ─── Header ─── */}
      <header
        className="page-header"
        style={{ display: "flex", justifyContent: "space-between", marginBottom: 24 }}
      >
        <div>
          <h1 style={{ margin: 0 }}>Настройки</h1>
          <p style={{ margin: "4px 0 0", color: "#667085" }}>
            Локальные параметры приложения CaseGraph.
          </p>
        </div>

        <div style={{ display: "flex", gap: 8, alignItems: "center" }}>
          <button
            id="settings-save-btn"
            type="button"
            className="btn btn-primary"
            onClick={handleSave}
            disabled={!isDirty || isAnyBusy || !can(permissions, protectedOperations.settingsUpdate)}
            style={{ padding: "8px 16px" }}
          >
            {isSaving ? "Сохранение..." : "Сохранить настройки"}
          </button>

          <button
            id="settings-reset-btn"
            type="button"
            onClick={() => setIsResetConfirmOpen(true)}
            disabled={isAnyBusy}
            style={{
              padding: "8px 16px",
              background: "transparent",
              border: "1px solid #ef4444",
              color: "#ef4444",
              borderRadius: 6,
              cursor: isAnyBusy ? "default" : "pointer",
              fontWeight: 500,
            }}
          >
            Сбросить настройки
          </button>

          <button
            type="button"
            onClick={onBack}
            style={{ padding: "8px 16px", cursor: "pointer" }}
          >
            Назад к делам
          </button>
        </div>
      </header>

      {/* ─── Alerts ─── */}
      {errorMessage && (
        <div style={{ padding: 12, background: "#fde8e8", color: "#9b1c1c", borderRadius: 6, marginBottom: 16 }}>
          {errorMessage}
        </div>
      )}
      {savedMessage && (
        <div style={{ padding: 12, background: "#edf7ed", color: "#1e4620", borderRadius: 6, marginBottom: 16 }}>
          {savedMessage}
        </div>
      )}

      {/* ─── Settings form ─── */}
      <section className="settings-grid">

        {/* Storage paths */}
        <SettingsSection title="Хранилище данных">
          <SettingField label="Папка экспорта DOCX">
            <div className="settings-path-row">
              <input
                id="docx-export-dir"
                type="text"
                value={draft.docx.defaultExportDir}
                readOnly
                placeholder="Папка не выбрана"
                style={{ flex: 1, padding: 8, boxSizing: "border-box", borderRadius: 4, border: "1px solid #ccc", background: "#f9f9f9" }}
              />
              <button
                type="button"
                onClick={() => handlePickDirectory("docxDefaultExportDir")}
                disabled={isAnyBusy}
                style={{ padding: "8px 12px", cursor: isAnyBusy ? "default" : "pointer" }}
              >
                {pickingTarget === "docxDefaultExportDir" ? "Выбор..." : "Выбрать папку"}
              </button>
            </div>
          </SettingField>

          <SettingField label="Папка резервных копий">
            <div className="settings-path-row">
              <input
                id="backup-dir"
                type="text"
                value={draft.backup.defaultBackupDir}
                readOnly
                placeholder="Папка не выбрана"
                style={{ flex: 1, padding: 8, boxSizing: "border-box", borderRadius: 4, border: "1px solid #ccc", background: "#f9f9f9" }}
              />
              <button
                type="button"
                onClick={() => handlePickDirectory("backupDefaultBackupDir")}
                disabled={isAnyBusy}
                style={{ padding: "8px 12px", cursor: isAnyBusy ? "default" : "pointer" }}
              >
                {pickingTarget === "backupDefaultBackupDir" ? "Выбор..." : "Выбрать папку"}
              </button>
            </div>
          </SettingField>
        </SettingsSection>

        {/* DOCX template & options */}
        <SettingsSection title="Шаблоны и экспорт DOCX">
          <SettingField label="Шаблон отчёта по умолчанию">
            <select
              value={draft.docx.defaultTemplate}
              onChange={(e) =>
                setDraft({ ...draft, docx: { ...draft.docx, defaultTemplate: e.target.value } })
              }
              style={{ width: "100%", padding: 8, boxSizing: "border-box", borderRadius: 4, border: "1px solid #ccc" }}
            >
              <option value="analytical-report">Аналитическая справка</option>
              <option value="operational-summary">Оперативная сводка</option>
              <option value="extended-report">Расширенная справка</option>
            </select>
          </SettingField>

          <CheckboxField
            id="docx-materials"
            label="Включать таблицу материалов в DOCX"
            checked={draft.docx.includeMaterialsTable}
            onChange={(v) =>
              setDraft({ ...draft, docx: { ...draft.docx, includeMaterialsTable: v } })
            }
          />

          <CheckboxField
            id="docx-sha256"
            label="Включать таблицу контрольных сумм SHA-256"
            checked={draft.docx.includeSha256Table}
            onChange={(v) =>
              setDraft({ ...draft, docx: { ...draft.docx, includeSha256Table: v } })
            }
          />
        </SettingsSection>

        {/* Backup */}
        <SettingsSection title="Резервное копирование">
          <CheckboxField
            id="backup-safety"
            label="Создавать бэкап безопасности перед восстановлением"
            checked={draft.backup.safetyBackupBeforeRestore}
            onChange={(v) =>
              setDraft({ ...draft, backup: { ...draft.backup, safetyBackupBeforeRestore: v } })
            }
          />
          <CheckboxField
            id="backup-verify"
            label="Проверять целостность бэкапа после создания"
            checked={draft.backup.verifyBackupAfterCreate}
            onChange={(v) =>
              setDraft({ ...draft, backup: { ...draft.backup, verifyBackupAfterCreate: v } })
            }
          />
        </SettingsSection>

        {/* Integrity warnings */}
        <SettingsSection title="Предупреждения и целостность">
          <CheckboxField
            id="integrity-docx"
            label="Предупреждать перед экспортом DOCX при невалидной целостности"
            checked={draft.integrity.warnBeforeDocxExport}
            onChange={(v) =>
              setDraft({ ...draft, integrity: { ...draft.integrity, warnBeforeDocxExport: v } })
            }
          />
          <CheckboxField
            id="integrity-backup"
            label="Предупреждать перед резервным копированием при ошибках"
            checked={draft.integrity.warnBeforeBackup}
            onChange={(v) =>
              setDraft({ ...draft, integrity: { ...draft.integrity, warnBeforeBackup: v } })
            }
          />
        </SettingsSection>

        {/* Role policies */}
        <SettingsSection title="Политика доступа (Роли)">
          <CheckboxField
            id="access-viewer-docx"
            label="Разрешить наблюдателю (viewer) экспорт в DOCX"
            checked={draft.access.viewerCanExportDocx}
            onChange={(v) =>
              setDraft({ ...draft, access: { ...draft.access, viewerCanExportDocx: v } })
            }
          />
          <CheckboxField
            id="access-analyst-backup"
            label="Разрешить аналитику (analyst) создание резервных копий"
            checked={draft.access.analystCanCreateBackup}
            onChange={(v) =>
              setDraft({ ...draft, access: { ...draft.access, analystCanCreateBackup: v } })
            }
          />
        </SettingsSection>
      </section>

      {/* ─── Reset confirmation modal ─── */}
      {isResetConfirmOpen && (
        <ConfirmModal
          title="Сбросить настройки?"
          confirmText={isResetting ? "Сброс..." : "Сбросить"}
          cancelText="Отмена"
          tone="danger"
          disabled={isResetting}
          onCancel={() => setIsResetConfirmOpen(false)}
          onConfirm={() => void handleResetSettingsToDefaults()}
        >
          <p>
            Будут восстановлены значения по умолчанию для DOCX, backup,
            проверки целостности и политик доступа.
          </p>
          <p>
            Дела, материалы, пользователи, журнал действий и резервные
            копии не будут удалены.
          </p>
        </ConfirmModal>
      )}
    </main>
  );
}

/* ─── Helper sub-components ─── */

type SettingsSectionProps = { title: string; children: React.ReactNode };
function SettingsSection({ title, children }: SettingsSectionProps) {
  return (
    <section className="settings-section" style={{ display: "flex", flexDirection: "column", gap: 12 }}>
      <h2 style={{ margin: 0, borderBottom: "1px solid #eee", paddingBottom: 8, fontSize: "1.1rem" }}>
        {title}
      </h2>
      <div className="settings-section-body" style={{ display: "flex", flexDirection: "column", gap: 8 }}>
        {children}
      </div>
    </section>
  );
}

type SettingFieldProps = { label: string; children: React.ReactNode };
function SettingField({ label, children }: SettingFieldProps) {
  return (
    <div className="setting-field" style={{ display: "flex", flexDirection: "column", gap: 4 }}>
      <label style={{ fontSize: "0.9rem", color: "#475467" }}>{label}</label>
      {children}
    </div>
  );
}

type CheckboxFieldProps = {
  id: string;
  label: string;
  checked: boolean;
  onChange: (value: boolean) => void;
};
function CheckboxField({ id, label, checked, onChange }: CheckboxFieldProps) {
  return (
    <label
      htmlFor={id}
      style={{ display: "flex", alignItems: "center", gap: 8, cursor: "pointer" }}
    >
      <input
        id={id}
        type="checkbox"
        checked={checked}
        onChange={(e) => onChange(e.target.checked)}
      />
      <span>{label}</span>
    </label>
  );
}
