import { useEffect, useState } from "react";
import {
  initializeApp,
  type InitializeAppResponse,
} from "./api/appApi";
import {
  getCurrentUser,
  logout,
} from "../features/auth/api/authApi";
import type { CurrentUserDto } from "../features/auth/model/authTypes";
import { FirstAdminSetupPage } from "../pages/first-admin/FirstAdminSetupPage";
import { LoginPage } from "../pages/login/LoginPage";
import { CasesPage } from "../pages/cases/CasesPage";

type AppStatus = "booting" | "ready" | "error";

export function App() {
  const [status, setStatus] = useState<AppStatus>("booting");
  const [init, setInit] = useState<InitializeAppResponse | null>(null);
  const [currentUser, setCurrentUser] = useState<CurrentUserDto | null>(null);
  const [error, setError] = useState<string | null>(null);

  async function boot() {
    try {
      setStatus("booting");

      const initResponse = await initializeApp();
      setInit(initResponse);

      if (initResponse.hasAdmin) {
        const user = await getCurrentUser();
        setCurrentUser(user);
      } else {
        setCurrentUser(null);
      }

      setStatus("ready");
    } catch (err) {
      console.error(err);
      setError("Не удалось инициализировать приложение.");
      setStatus("error");
    }
  }

  async function handleLogout() {
    await logout();
    setCurrentUser(null);
  }

  useEffect(() => {
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
    return <FirstAdminSetupPage onCreated={boot} />;
  }

  if (!currentUser) {
    return <LoginPage onLoggedIn={setCurrentUser} />;
  }

  return <CasesPage user={currentUser} onLogout={handleLogout} />;
}