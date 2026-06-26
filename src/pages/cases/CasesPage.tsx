import { useEffect, useState } from "react";
import type { CurrentUserDto } from "../../features/auth/model/authTypes";
import { getCases } from "../../features/cases/api/casesApi";
import type { CaseDto } from "../../features/cases/model/caseTypes";
import { CreateCaseModal } from "./CreateCaseModal";
import { getCaseStatusLabel } from "../../features/cases/model/caseStatus";

type Props = {
  user: CurrentUserDto;
  onLogout: () => void;
  onOpenCase: (caseItem: CaseDto) => void;
};

export function CasesPage({ user, onLogout, onOpenCase }: Props) {
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
    <main style={{ padding: 32 }}>
      <header style={{ display: "flex", justifyContent: "space-between" }}>
        <div>
          <h1>Список дел</h1>
          <p>
            Пользователь: <strong>{user.displayName}</strong> · Роль:{" "}
            <strong>{user.role}</strong>
          </p>
        </div>

        <div style={{ display: "flex", gap: 8 }}>
          <button type="button" onClick={() => setCreateModalOpen(true)}>
            Создать дело
          </button>

          <button type="button" onClick={onLogout}>
            Выйти
          </button>
        </div>
      </header>

      <hr />

      {loading && <p>Загрузка дел...</p>}

      {error && <p style={{ color: "crimson" }}>{error}</p>}

      {!loading && !error && cases.length === 0 && (
        <section>
          <h2>Дел пока нет</h2>
          <p>Создайте первое дело, чтобы начать работу.</p>
        </section>
      )}

      {!loading && !error && cases.length > 0 && (
        <table border={1} cellPadding={8} style={{ borderCollapse: "collapse" }}>
          <thead>
            <tr>
              <th>Код</th>
              <th>Название</th>
              <th>Объект анализа</th>
              <th>Статус</th>
              <th>Создано</th>
              <th>Действия</th>
            </tr>
          </thead>

          <tbody>
            {cases.map((caseItem) => (
              <tr key={caseItem.id}>
                <td>{caseItem.caseCode}</td>
                <td>{caseItem.title}</td>
                <td>{caseItem.subject}</td>
                <td>{getCaseStatusLabel(caseItem.status)}</td>
                <td>{caseItem.createdAt}</td>
                <td>
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
  );
}