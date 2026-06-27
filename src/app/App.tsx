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
import { AuditLogPage } from "../pages/audit-log/AuditLogPage";
import { UsersPage } from "../features/users/ui/UsersPage";
import { ChangePasswordPage } from "../pages/change-password/ChangePasswordPage";

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
  const [showAuditLog, setShowAuditLog] = useState(false);
  const [showUsers, setShowUsers] = useState(false);
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

  useEffect(() => {
    function handlePasswordChangeRequired() {
      setCurrentUser((prev) => (prev ? { ...prev, mustChangePassword: true } : null));
    }
    window.addEventListener("password-change-required", handlePasswordChangeRequired);
    return () => {
      window.removeEventListener("password-change-required", handlePasswordChangeRequired);
    };
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

          if (user.mustChangePassword) {
            setBootstrapState("authenticated");
            return;
          }

          setBootstrapState("authenticated");
        }}
      />
    );
  }

  if (!currentUser) {
    return <main style={{ padding: 32 }}>Сессия не найдена.</main>;
  }

  if (currentUser.mustChangePassword) {
    return (
      <ChangePasswordPage
        onPasswordChanged={async () => {
          try {
            const { getCurrentUser } = await import(
              "../features/auth/api/authApi"
            );
            const user = await getCurrentUser();

            if (user) {
              setCurrentUser(user);
            }
          } finally {
            setBootstrapState("authenticated");
          }
        }}
      />
    );
  }

  if (selectedCase) {
    return (
      <CaseWorkspacePage
        user={currentUser}
        caseItem={selectedCase}
        onCaseUpdated={setSelectedCase}
        onBackToCases={() => setSelectedCase(null)}
        onLogout={handleLogout}
      />
    );
  }

  if (showAuditLog) {
    return (
      <AuditLogPage
        user={currentUser}
        onBack={() => setShowAuditLog(false)}
      />
    );
  }

  if (showUsers && currentUser.role === "administrator") {
    return (
      <UsersPage
        user={currentUser}
        onBack={() => setShowUsers(false)}
      />
    );
  }

  return (
    <CasesPage
      user={currentUser}
      onLogout={handleLogout}
      onOpenCase={setSelectedCase}
      onOpenAuditLog={() => setShowAuditLog(true)}
      onOpenUsers={() => setShowUsers(true)}
    />
  );
}