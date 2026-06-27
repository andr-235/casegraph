import { useEffect, useState } from "react";
import { getSettings } from "../../features/settings/api/settingsApi";
import type { AppSettingsDto } from "../../features/settings/model/settingsTypes";
import { formatError } from "../../shared/lib/formatError";

type SettingsPageProps = {
  onBack: () => void;
};

export function SettingsPage({ onBack }: SettingsPageProps) {
  const [settings, setSettings] = useState<AppSettingsDto | null>(null);
  const [isLoading, setIsLoading] = useState(true);
  const [errorMessage, setErrorMessage] = useState<string | null>(null);

  useEffect(() => {
    let isMounted = true;

    async function loadSettings() {
      try {
        setIsLoading(true);
        setErrorMessage(null);

        const data = await getSettings();

        if (isMounted) {
          setSettings(data);
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

  if (isLoading) {
    return (
      <main className="page">
        <h1>Настройки</h1>
        <p>Загрузка настроек…</p>
      </main>
    );
  }

  if (errorMessage) {
    return (
      <main className="page">
        <h1>Настройки</h1>
        <div className="error-state">
          <strong>Не удалось загрузить настройки.</strong>
          <p>{errorMessage}</p>
        </div>
      </main>
    );
  }

  if (!settings) {
    return (
      <main className="page">
        <h1>Настройки</h1>
        <p>Настройки не найдены.</p>
      </main>
    );
  }

  return (
    <main className="page">
      <header className="page-header">
        <div>
          <h1>Настройки</h1>
          <p>Локальные параметры приложения CaseGraph.</p>
        </div>

        <div style={{ display: "flex", gap: 8 }}>
          <button type="button" className="btn btn-primary" disabled>
            Сохранить
          </button>
          <button type="button" onClick={onBack}>
            Назад к делам
          </button>
        </div>
      </header>

      <section className="settings-grid">
        <SettingsSection title="Хранилище данных">
          <ReadonlySetting
            label="Путь хранения данных"
            value={settings.storagePath ?? "Используется путь по умолчанию"}
          />
        </SettingsSection>

        <SettingsSection title="DOCX export">
          <ReadonlySetting
            label="Шаблон по умолчанию"
            value={settings.docxDefaultTemplate}
          />
          <ReadonlySetting
            label="Экспорт DOCX для наблюдателя"
            value={settings.viewerCanExportDocx ? "Разрешён" : "Запрещён"}
          />
        </SettingsSection>

        <SettingsSection title="Backup">
          <ReadonlySetting
            label="Папка backup по умолчанию"
            value={settings.backupDefaultPath ?? "Не задана"}
          />
          <ReadonlySetting
            label="Backup для аналитика"
            value={settings.analystCanCreateBackup ? "Разрешён" : "Запрещён"}
          />
        </SettingsSection>

        <SettingsSection title="Целостность и журнал">
          <ReadonlySetting
            label="Проверка целостности при запуске"
            value={settings.integrityCheckOnStartup ? "Включена" : "Выключена"}
          />
          <ReadonlySetting
            label="Строгий режим audit"
            value={settings.auditStrictMode ? "Включён" : "Выключен"}
          />
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
    <section className="settings-section">
      <h2>{title}</h2>
      <div className="settings-section-body">{children}</div>
    </section>
  );
}

type ReadonlySettingProps = {
  label: string;
  value: string;
};

function ReadonlySetting({ label, value }: ReadonlySettingProps) {
  return (
    <div className="readonly-setting">
      <span>{label}</span>
      <strong>{value}</strong>
    </div>
  );
}
