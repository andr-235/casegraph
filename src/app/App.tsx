import { useEffect, useState } from "react";
import { initializeApp, type InitializeAppResponse } from "../shared/api/appApi";

type AppStatus = "booting" | "ready" | "error";

export function App() {
  const [status, setStatus] = useState<AppStatus>("booting");
  const [init, setInit] = useState<InitializeAppResponse | null>(null);
  const [error, setError] = useState<string | null>(null);

  useEffect(() => {
    async function boot() {
      try {
        const response = await initializeApp();
        setInit(response);
        setStatus("ready");
      } catch (err) {
        console.error(err);
        setError("Не удалось инициализировать приложение.");
        setStatus("error");
      }
    }

    boot();
  }, []);

  if (status === "booting") {
    return <div>Загрузка CaseGraph...</div>;
  }

  if (status === "error") {
    return (
      <div>
        <h1>Ошибка запуска</h1>
        <p>{error}</p>
      </div>
    );
  }

  if (!init?.hasAdmin) {
    return <div>Показать FirstAdminSetupPage</div>;
  }

  return <div>Показать LoginPage</div>;
}