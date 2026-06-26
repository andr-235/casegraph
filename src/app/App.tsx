import { useEffect, useState } from "react";
import {
  getCurrentUser,
  logout,
} from "../features/auth/api/authApi";
import type { CurrentUserDto } from "../features/auth/model/authTypes";
import type { CaseDto } from "../features/cases/model/caseTypes";
import { initializeApp } from "../app/api/appApi";
import { FirstAdminSetupPage } from "../pages/first-admin/FirstAdminSetupPage";
import { LoginPage } from "../pages/login/LoginPage";
import { CasesPage } from "../pages/cases/CasesPage";
import { CaseWorkspacePage } from "../pages/case-workspace/CaseWorkspacePage";

type BootstrapState =
  | "loading"
  | "firstAdminRequired"
  | "loginRequired"
  | "authenticated"
  | "error";

export function App() {
  const [bootstrapState, setBootstrapState] =
    useState<BootstrapState>("loading");
  const [currentUser, setCurrentUser] = useState<CurrentUserDto | null>(null);
  const [selectedCase, setSelectedCase] = useState<CaseDto | null>(null);
  const [startupError, setStartupError] = useState<string | null>(null);

  async function boot() {
    try {
      setBootstrapState("loading");
      setStartupError(null);

      const appState = await initializeApp();

      if (!appState.hasAdmin) {
        setCurrentUser(null);
        setSelectedCase(null);
        setBootstrapState("firstAdminRequired");
        return;
      }

      const user = await getCurrentUser();

      if (!user) {
        setCurrentUser(null);
        setSelectedCase(null);
        setBootstrapState("loginRequired");
        return;
      }

      setCurrentUser(user);
      setBootstrapState("authenticated");
    } catch (err) {
      console.error(err);
      setStartupError("Не удалось запустить приложение.");
      setBootstrapState("error");
    }
  }

  useEffect(() => {
    boot();
  }, []);

  async function handleLogout() {
    try {
      await logout();
    } finally {
      setCurrentUser(null);
      setSelectedCase(null);
      setBootstrapState("loginRequired");
    }
  }

  if (bootstrapState === "loading") {
    return <main style={{ padding: 32 }}>Загрузка CaseGraph...</main>;
  }

  if (bootstrapState === "error") {
    return (
      <main style={{ padding: 32 }}>
        <h1>Ошибка запуска</h1>
        <p>{startupError}</p>
        <button type="button" onClick={boot}>
          Повторить
        </button>
      </main>
    );
  }

  if (bootstrapState === "firstAdminRequired") {
    return <FirstAdminSetupPage onCreated={boot} />;
  }

  if (bootstrapState === "loginRequired") {
    return (
      <LoginPage
        onLoggedIn={(user) => {
          setCurrentUser(user);
          setSelectedCase(null);
          setBootstrapState("authenticated");
        }}
      />
    );
  }

  if (!currentUser) {
    return <main style={{ padding: 32 }}>Сессия не найдена.</main>;
  }

  if (selectedCase) {
    return (
      <CaseWorkspacePage
        user={currentUser}
        caseItem={selectedCase}
        onBackToCases={() => setSelectedCase(null)}
        onLogout={handleLogout}
      />
    );
  }

    return (
    <CasesPage
      user={currentUser}
      onLogout={handleLogout}
      onOpenCase={setSelectedCase}
    />
  );
}