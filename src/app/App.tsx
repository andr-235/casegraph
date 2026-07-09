import { useEffect, useState } from "react";
import {
  getCurrentUser,
  logout,
} from "../features/auth/api/authApi";
import { getEffectivePermissions } from "../features/auth/api/permissionsApi";
import type { CurrentUserDto } from "../features/auth/model/authTypes";
import type { EffectivePermissionsDto } from "../features/auth/model/effectivePermissionsTypes";
import type { CaseDto } from "../features/cases/model/caseTypes";
import { initializeApp } from "../app/api/appApi";
import { can } from "../shared/lib/permissions";
import { protectedOperations } from "../shared/security/protectedOperations";
import { FirstAdminSetupPage } from "../pages/first-admin/FirstAdminSetupPage";
import { LoginPage } from "../pages/login/LoginPage";
import { CasesPage } from "../pages/cases/CasesPage";
import { CaseWorkspacePage } from "../pages/case-workspace/CaseWorkspacePage";
import { AuditLogPage } from "../pages/audit-log/AuditLogPage";
import { UsersPage } from "../features/users/ui/UsersPage";
import { ChangePasswordPage } from "../pages/change-password/ChangePasswordPage";
import { SettingsPage } from "../pages/settings/SettingsPage";
import { BackupPage } from "../pages/backup/BackupPage";

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
  const [permissions, setPermissions] = useState<EffectivePermissionsDto | null>(null);
  const [selectedCase, setSelectedCase] = useState<CaseDto | null>(null);
  const [showAuditLog, setShowAuditLog] = useState(false);
  const [showUsers, setShowUsers] = useState(false);
  const [showSettings, setShowSettings] = useState(false);
  const [showBackup, setShowBackup] = useState(false);
  const [startupError, setStartupError] = useState<string | null>(null);

  async function reloadPermissions() {
    try {
      const nextPermissions = await getEffectivePermissions();
      setPermissions(nextPermissions);
    } catch {
      setPermissions(null);
    }
  }

  async function boot() {
    try {
      setBootstrapState("loading");
      setStartupError(null);

      const appState = await initializeApp();

      if (!appState.hasAdmin) {
        setCurrentUser(null);
        setSelectedCase(null);
        setPermissions(null);
        setBootstrapState("firstAdminRequired");
        return;
      }

      const user = await getCurrentUser();

      if (!user) {
        setCurrentUser(null);
        setSelectedCase(null);
        setPermissions(null);
        setBootstrapState("loginRequired");
        return;
      }

      setCurrentUser(user);
      await reloadPermissions();
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
      setPermissions(null);
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
        onLoggedIn={async (user) => {
          setCurrentUser(user);
          setSelectedCase(null);
          await reloadPermissions();
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
            await reloadPermissions();
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
        permissions={permissions}
        caseItem={selectedCase}
        onCaseUpdated={setSelectedCase}
        onBackToCases={() => setSelectedCase(null)}
        onLogout={handleLogout}
        onOpenSettings={() => setShowSettings(true)}
        onOpenBackup={() => setShowBackup(true)}
        onOpenAuditLog={() => setShowAuditLog(true)}
        onOpenUsers={() => setShowUsers(true)}
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

  if (showUsers && can(permissions, protectedOperations.userManage)) {
    return (
      <UsersPage
        user={currentUser}
        onBack={() => setShowUsers(false)}
      />
    );
  }

  if (showBackup && can(permissions, protectedOperations.backupRead)) {
    return (
      <BackupPage
        permissions={permissions}
        onBack={() => setShowBackup(false)}
      />
    );
  }

  if (showSettings && can(permissions, protectedOperations.settingsRead)) {
    return (
      <SettingsPage
        permissions={permissions}
        onReloadPermissions={reloadPermissions}
        onBack={() => setShowSettings(false)}
      />
    );
  }

  return (
    <CasesPage
      user={currentUser}
      permissions={permissions}
      onLogout={handleLogout}
      onOpenCase={setSelectedCase}
      onOpenAuditLog={() => setShowAuditLog(true)}
      onOpenUsers={() => setShowUsers(true)}
      onOpenSettings={() => setShowSettings(true)}
      onOpenBackup={() => setShowBackup(true)}
    />
  );
}
