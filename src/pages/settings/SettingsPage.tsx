import { useEffect, useState } from "react";
import { getSettings, updateSettings, chooseSettingsDirectory } from "../../features/settings/api/settingsApi";
import type { AppSettingsDto, UpdateSettingsPayload, SettingsDirectoryTarget } from "../../features/settings/model/settingsTypes";
import { formatError } from "../../shared/lib/formatError";

type SettingsPageProps = {
  onBack: () => void;
};

export function SettingsPage({ onBack }: SettingsPageProps) {
  const [settings, setSettings] = useState<AppSettingsDto | null>(null);
  const [draft, setDraft] = useState<UpdateSettingsPayload | null>(null);
  const [isLoading, setIsLoading] = useState(true);
  const [isSaving, setIsSaving] = useState(false);
  const [errorMessage, setErrorMessage] = useState<string | null>(null);
  const [savedMessage, setSavedMessage] = useState<string | null>(null);
  const [pickingTarget, setPickingTarget] = useState<SettingsDirectoryTarget | null>(null);

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

      if (!response.path) {
        return;
      }

      if (target === "docxDefaultExportDir") {
        setDraft({
          ...draft,
          docx: {
            ...draft.docx,
            defaultExportDir: response.path,
          },
        });
        return;
      }

      if (target === "backupDefaultBackupDir") {
        setDraft({
          ...draft,
          backup: {
            ...draft.backup,
            defaultBackupDir: response.path,
          },
        });
      }
    } catch (err) {
      setErrorMessage(formatError(err));
    } finally {
      setPickingTarget(null);
    }
  }

  const isDirty =
    settings !== null &&
    draft !== null &&
    JSON.stringify(toUpdatePayload(settings)) !== JSON.stringify(draft);

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
        <div className="error-state" style={{ color: "crimson", marginBottom: 16 }}>
          <strong>Не удалось загрузить настройки.</strong>
          <p>{errorMessage}</p>
        </div>
        <button type="button" onClick={onBack}>
          Назад к делам
        </button>
      </main>
    );
  }

  if (!settings || !draft) {
    return (
      <main className="page" style={{ padding: 32 }}>
        <h1>Настройки</h1>
        <p>Настройки не найдены.</p>
        <button type="button" onClick={onBack}>
          Назад к делам
        </button>
      </main>
    );
  }

  return (
    <main className="page" style={{ padding: 32 }}>
      <header className="page-header" style={{ display: "flex", justifyContent: "space-between", marginBottom: 24 }}>
        <div>
          <h1 style={{ margin: 0 }}>Настройки</h1>
          <p style={{ margin: "4px 0 0", color: "#667085" }}>Локальные параметры приложения CaseGraph.</p>
        </div>

        <div style={{ display: "flex", gap: 8, alignItems: "center" }}>
          <button
            type="button"
            className="btn btn-primary"
            onClick={handleSave}
            disabled={!isDirty || isSaving}
            style={{ padding: "8px 16px", cursor: isDirty && !isSaving ? "pointer" : "default" }}
          >
            {isSaving ? "Сохранение..." : "Сохранить настройки"}
          </button>
          <button type="button" onClick={onBack} style={{ padding: "8px 16px", cursor: "pointer" }}>
            Назад к делам
          </button>
        </div>
      </header>

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

      <section className="settings-grid">
        <SettingsSection title="Хранилище данных">
          <SettingField label="Папка экспорта DOCX">
            <div className="settings-path-row" style={{ display: "flex", gap: 8, alignItems: "center" }}>
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
                disabled={pickingTarget !== null || isSaving}
                style={{ padding: "8px 12px", cursor: pickingTarget === null && !isSaving ? "pointer" : "default" }}
              >
                {pickingTarget === "docxDefaultExportDir" ? "Выбор..." : "Выбрать папку"}
              </button>
            </div>
          </SettingField>

          <SettingField label="Папка резервных копий">
            <div className="settings-path-row" style={{ display: "flex", gap: 8, alignItems: "center" }}>
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
                disabled={pickingTarget !== null || isSaving}
                style={{ padding: "8px 12px", cursor: pickingTarget === null && !isSaving ? "pointer" : "default" }}
              >
                {pickingTarget === "backupDefaultBackupDir" ? "Выбор..." : "Выбрать папку"}
              </button>
            </div>
          </SettingField>
        </SettingsSection>

        <SettingsSection title="Шаблоны и экспорт DOCX">
          <SettingField label="Шаблон отчета по умолчанию">
            <select
              value={draft.docx.defaultTemplate}
              onChange={(e) =>
                setDraft({
                  ...draft,
                  docx: {
                    ...draft.docx,
                    defaultTemplate: e.target.value,
                  },
                })
              }
              style={{ width: "100%", padding: 8, boxSizing: "border-box", borderRadius: 4, border: "1px solid #ccc" }}
            >
              <option value="analytical-report">Аналитическая справка</option>
              <option value="operational-summary">Оперативная сводка</option>
              <option value="extended-report">Расширенная справка</option>
            </select>
          </SettingField>

          <label style={{ display: "flex", alignItems: "center", gap: 8, cursor: "pointer", marginTop: 8 }}>
            <input
              type="checkbox"
              checked={draft.docx.includeMaterialsTable}
              onChange={(e) =>
                setDraft({
                  ...draft,
                  docx: {
                    ...draft.docx,
                    includeMaterialsTable: e.target.checked,
                  },
                })
              }
            />
            <span>Включать таблицу материалов в DOCX</span>
          </label>

          <label style={{ display: "flex", alignItems: "center", gap: 8, cursor: "pointer" }}>
            <input
              type="checkbox"
              checked={draft.docx.includeSha256Table}
              onChange={(e) =>
                setDraft({
                  ...draft,
                  docx: {
                    ...draft.docx,
                    includeSha256Table: e.target.checked,
                  },
                })
              }
            />
            <span>Включать таблицу контрольных сумм SHA-256</span>
          </label>
        </SettingsSection>

        <SettingsSection title="Резервное копирование">
          <label style={{ display: "flex", alignItems: "center", gap: 8, cursor: "pointer" }}>
            <input
              type="checkbox"
              checked={draft.backup.safetyBackupBeforeRestore}
              onChange={(e) =>
                setDraft({
                  ...draft,
                  backup: {
                    ...draft.backup,
                    safetyBackupBeforeRestore: e.target.checked,
                  },
                })
              }
            />
            <span>Создавать бэкап безопасности перед восстановлением</span>
          </label>

          <label style={{ display: "flex", alignItems: "center", gap: 8, cursor: "pointer" }}>
            <input
              type="checkbox"
              checked={draft.backup.verifyBackupAfterCreate}
              onChange={(e) =>
                setDraft({
                  ...draft,
                  backup: {
                    ...draft.backup,
                    verifyBackupAfterCreate: e.target.checked,
                  },
                })
              }
            />
            <span>Проверять целостность бэкапа после создания</span>
          </label>
        </SettingsSection>

        <SettingsSection title="Предупреждения и целостность">
          <label style={{ display: "flex", alignItems: "center", gap: 8, cursor: "pointer" }}>
            <input
              type="checkbox"
              checked={draft.integrity.warnBeforeDocxExport}
              onChange={(e) =>
                setDraft({
                  ...draft,
                  integrity: {
                    ...draft.integrity,
                    warnBeforeDocxExport: e.target.checked,
                  },
                })
              }
            />
            <span>Предупреждать перед экспортом DOCX при невалидной целостности</span>
          </label>

          <label style={{ display: "flex", alignItems: "center", gap: 8, cursor: "pointer" }}>
            <input
              type="checkbox"
              checked={draft.integrity.warnBeforeBackup}
              onChange={(e) =>
                setDraft({
                  ...draft,
                  integrity: {
                    ...draft.integrity,
                    warnBeforeBackup: e.target.checked,
                  },
                })
              }
            />
            <span>Предупреждать перед резервным копированием при ошибках</span>
          </label>
        </SettingsSection>

        <SettingsSection title="Политика доступа (Роли)">
          <label style={{ display: "flex", alignItems: "center", gap: 8, cursor: "pointer" }}>
            <input
              type="checkbox"
              checked={draft.access.viewerCanExportDocx}
              onChange={(e) =>
                setDraft({
                  ...draft,
                  access: {
                    ...draft.access,
                    viewerCanExportDocx: e.target.checked,
                  },
                })
              }
            />
            <span>Разрешить наблюдателю (viewer) экспорт в DOCX</span>
          </label>

          <label style={{ display: "flex", alignItems: "center", gap: 8, cursor: "pointer" }}>
            <input
              type="checkbox"
              checked={draft.access.analystCanCreateBackup}
              onChange={(e) =>
                setDraft({
                  ...draft,
                  access: {
                    ...draft.access,
                    analystCanCreateBackup: e.target.checked,
                  },
                })
              }
            />
            <span>Разрешить аналитику (analyst) создание резервных копий</span>
          </label>
        </SettingsSection>
      </section>
    </main>
  );
}

type SettingsSectionProps = {
  title: string;
  children: React.ReactNode;
};

function SettingsSection({ title, children }: SettingsSectionProps) {
  return (
    <section className="settings-section" style={{ display: "flex", flexDirection: "column", gap: 12 }}>
      <h2 style={{ margin: 0, borderBottom: "1px solid #eee", paddingBottom: 8, fontSize: "1.1rem" }}>{title}</h2>
      <div className="settings-section-body" style={{ display: "flex", flexDirection: "column", gap: 8 }}>
        {children}
      </div>
    </section>
  );
}

type SettingFieldProps = {
  label: string;
  children: React.ReactNode;
};

function SettingField({ label, children }: SettingFieldProps) {
  return (
    <div className="setting-field" style={{ display: "flex", flexDirection: "column", gap: 4 }}>
      <label style={{ fontSize: "0.9rem", color: "#475467" }}>{label}</label>
      {children}
    </div>
  );
}
