import { useEffect, useState } from "react";
import type { CurrentUserDto } from "../../features/auth/model/authTypes";
import type { EffectivePermissionsDto } from "../../features/auth/model/effectivePermissionsTypes";
import { getCases } from "../../features/cases/api/casesApi";
import type { CaseDto } from "../../features/cases/model/caseTypes";
import { CreateCaseModal } from "./CreateCaseModal";
import { getCaseStatusLabel } from "../../features/cases/model/caseStatus";
import { can } from "../../shared/lib/permissions";
import { protectedOperations } from "../../shared/security/protectedOperations";
import { AppShell } from "../../shared/ui/AppShell";
import { TopBar } from "../../shared/ui/TopBar";

type Props = {
  user: CurrentUserDto;
  permissions: EffectivePermissionsDto | null;
  onLogout: () => void;
  onOpenCase: (caseItem: CaseDto) => void;
  onOpenAuditLog?: () => void;
  onOpenUsers?: () => void;
  onOpenSettings?: () => void;
  onOpenBackup?: () => void;
};

export function CasesPage({ user, permissions, onLogout, onOpenCase, onOpenAuditLog, onOpenUsers, onOpenSettings, onOpenBackup }: Props) {
  const [cases, setCases] = useState<CaseDto[]>([]);
  const [loading, setLoading] = useState(true);
  const [error, setError] = useState<string | null>(null);
  const [createModalOpen, setCreateModalOpen] = useState(false);

  async function loadCases() {
    try {
      setLoading(true);
      setError(null);

      const response = await getCases();
      setCases(response);
    } catch (err) {
      console.error(err);
      setError("Не удалось загрузить список дел.");
    } finally {
      setLoading(false);
    }
  }

  useEffect(() => {
    loadCases();
  }, []);

  function handleCaseCreated(caseItem: CaseDto) {
    setCases((current) => [caseItem, ...current]);
    setCreateModalOpen(false);
  }

  return (
    <AppShell>
      <TopBar
        displayName={user.displayName}
        onOpenSettings={onOpenSettings}
        onOpenBackup={onOpenBackup}
        onOpenAuditLog={onOpenAuditLog}
        onOpenUsers={onOpenUsers}
        onLogout={onLogout}
      />

      <main
        style={{
          padding: "var(--space-6)",
          overflow: "auto",
          flex: 1,
        }}
      >
        <div
          style={{
            display: "flex",
            justifyContent: "space-between",
            alignItems: "center",
            marginBottom: "var(--space-5)",
          }}
        >
          <h2 style={{ margin: 0, color: "var(--text-primary)", fontSize: 18 }}>
            Список дел
          </h2>

          {can(permissions, protectedOperations.caseCreate) ? (
            <button type="button" onClick={() => setCreateModalOpen(true)}>
              Создать дело
            </button>
          ) : null}
        </div>

        {loading && <p style={{ color: "var(--text-muted)" }}>Загрузка дел...</p>}

        {error && <p style={{ color: "var(--danger)" }}>{error}</p>}

        {!loading && !error && cases.length === 0 && (
          <section>
            <h2>Дел пока нет</h2>
            <p>Создайте первое дело, чтобы начать работу.</p>
          </section>
        )}

        {!loading && !error && cases.length > 0 && (
          <table
            style={{
              width: "100%",
              borderCollapse: "collapse",
              color: "var(--text-primary)",
            }}
          >
            <thead>
              <tr
                style={{
                  borderBottom: "1px solid var(--border-subtle)",
                  color: "var(--text-secondary)",
                  fontSize: 13,
                  textAlign: "left",
                }}
              >
                <th style={{ padding: "var(--space-2) var(--space-3)" }}>Код</th>
                <th style={{ padding: "var(--space-2) var(--space-3)" }}>Название</th>
                <th style={{ padding: "var(--space-2) var(--space-3)" }}>Объект анализа</th>
                <th style={{ padding: "var(--space-2) var(--space-3)" }}>Статус</th>
                <th style={{ padding: "var(--space-2) var(--space-3)" }}>Создано</th>
                <th style={{ padding: "var(--space-2) var(--space-3)" }}>Действия</th>
              </tr>
            </thead>

            <tbody>
              {cases.map((caseItem) => (
                <tr
                  key={caseItem.id}
                  style={{
                    borderBottom: "1px solid var(--border-subtle)",
                    fontSize: 13,
                  }}
                >
                  <td style={{ padding: "var(--space-3)" }}>{caseItem.caseCode}</td>
                  <td style={{ padding: "var(--space-3)" }}>{caseItem.title}</td>
                  <td style={{ padding: "var(--space-3)" }}>{caseItem.subject}</td>
                  <td style={{ padding: "var(--space-3)" }}>
                    {getCaseStatusLabel(caseItem.status)}
                  </td>
                  <td style={{ padding: "var(--space-3)" }}>{caseItem.createdAt}</td>
                  <td style={{ padding: "var(--space-3)" }}>
                    <button type="button" onClick={() => onOpenCase(caseItem)}>
                      Открыть
                    </button>
                  </td>
                </tr>
              ))}
            </tbody>
          </table>
        )}

        {createModalOpen && (
          <CreateCaseModal
            onCreated={handleCaseCreated}
            onClose={() => setCreateModalOpen(false)}
          />
        )}
      </main>
    </AppShell>
  );
}
