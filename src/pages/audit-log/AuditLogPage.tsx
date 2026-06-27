import { useEffect, useState } from "react";

import { getAuditLogs } from "../../features/audit/api/auditApi";
import type {
  AuditLogDto,
  GetAuditLogsPayload,
} from "../../features/audit/model/auditTypes";
import {
  AuditResultBadge,
  AuditSeverityBadge,
} from "../../features/audit/ui/AuditBadges";
import { formatError } from "../../shared/lib/formatError";

const DEFAULT_PAGE_SIZE = 50;

type AuditFilters = {
  action: string;
  result: string;
  severity: string;
  dateFrom: string;
  dateTo: string;
};

const emptyFilters: AuditFilters = {
  action: "",
  result: "",
  severity: "",
  dateFrom: "",
  dateTo: "",
};

type Props = {
  onBack: () => void;
};

export function AuditLogPage({ onBack }: Props) {
  const [items, setItems] = useState<AuditLogDto[]>([]);
  const [filters, setFilters] = useState<AuditFilters>(emptyFilters);
  const [total, setTotal] = useState(0);
  const [page, setPage] = useState(1);

  const [loading, setLoading] = useState(false);
  const [errorMessage, setErrorMessage] = useState("");

  async function loadAuditLogs(nextPage = page) {
    setLoading(true);
    setErrorMessage("");

    const payload: GetAuditLogsPayload = {
      action: filters.action || undefined,
      result: filters.result || undefined,
      severity: filters.severity || undefined,
      dateFrom: filters.dateFrom || undefined,
      dateTo: filters.dateTo || undefined,
      page: nextPage,
      pageSize: DEFAULT_PAGE_SIZE,
    };

    try {
      const response = await getAuditLogs(payload);

      setItems(response.items);
      setTotal(response.total);
      setPage(response.page);
    } catch (err) {
      setErrorMessage(formatError(err));
    } finally {
      setLoading(false);
    }
  }

  useEffect(() => {
    void loadAuditLogs(1);
    // eslint-disable-next-line react-hooks/exhaustive-deps
  }, []);

  function updateFilter<K extends keyof AuditFilters>(
    key: K,
    value: AuditFilters[K],
  ) {
    setFilters((current) => ({ ...current, [key]: value }));
  }

  function applyFilters() {
    void loadAuditLogs(1);
  }

  function resetFilters() {
    setFilters(emptyFilters);
    setPage(1);
    setTimeout(() => {
      void loadAuditLogs(1);
    }, 0);
  }

  const hasNextPage = page * DEFAULT_PAGE_SIZE < total;
  const hasPreviousPage = page > 1;

  return (
    <main style={{ padding: 32 }}>
      <section className="page audit-log-page">
        <header className="page-header">
          <div>
            <h1>Журнал действий</h1>
            <p>
              Просмотр значимых действий пользователей и системных операций.
            </p>
          </div>

          <div style={{ display: "flex", gap: 8 }}>
            <button type="button" onClick={() => void loadAuditLogs(page)}>
              Обновить
            </button>
            <button type="button" onClick={onBack}>
              Назад к делам
            </button>
          </div>
        </header>

        <div className="audit-log-toolbar">
          <label>
            Действие
            <input
              value={filters.action}
              onChange={(event) => updateFilter("action", event.target.value)}
              placeholder="EVENT_CREATED"
            />
          </label>

          <label>
            Результат
            <select
              value={filters.result}
              onChange={(event) => updateFilter("result", event.target.value)}
            >
              <option value="">Все</option>
              <option value="success">success</option>
              <option value="error">error</option>
              <option value="denied">denied</option>
            </select>
          </label>

          <label>
            Важность
            <select
              value={filters.severity}
              onChange={(event) => updateFilter("severity", event.target.value)}
            >
              <option value="">Все</option>
              <option value="info">info</option>
              <option value="warning">warning</option>
              <option value="error">error</option>
              <option value="critical">critical</option>
            </select>
          </label>

          <label>
            С даты
            <input
              type="date"
              value={filters.dateFrom}
              onChange={(event) => updateFilter("dateFrom", event.target.value)}
            />
          </label>

          <label>
            По дату
            <input
              type="date"
              value={filters.dateTo}
              onChange={(event) => updateFilter("dateTo", event.target.value)}
            />
          </label>

          <button type="button" onClick={applyFilters}>
            Применить
          </button>

          <button type="button" onClick={resetFilters}>
            Сбросить
          </button>
        </div>

        {errorMessage ? <div className="error-box">{errorMessage}</div> : null}

        {loading ? <div className="loading-state">Загрузка журнала...</div> : null}

        {!loading && items.length === 0 ? (
          <div className="empty-state">Записей журнала пока нет.</div>
        ) : null}

        {!loading && items.length > 0 ? (
          <>
            <table className="data-table audit-log-table">
              <thead>
                <tr>
                  <th>Время</th>
                  <th>Пользователь</th>
                  <th>Роль</th>
                  <th>Действие</th>
                  <th>Сущность</th>
                  <th>Дело</th>
                  <th>Результат</th>
                  <th>Важность</th>
                </tr>
              </thead>

              <tbody>
                {items.map((item) => (
                  <tr key={item.id}>
                    <td>{item.createdAt}</td>
                    <td>{item.username}</td>
                    <td>{item.userRole}</td>
                    <td>
                      <code>{item.action}</code>
                    </td>
                    <td>
                      <code>{item.entityType}</code>
                      {item.entityId ? (
                        <div className="muted-text">{item.entityId}</div>
                      ) : null}
                    </td>
                    <td>{item.caseId ?? "\u2014"}</td>
                    <td>
                      <AuditResultBadge value={item.result} />
                    </td>
                    <td>
                      <AuditSeverityBadge value={item.severity} />
                    </td>
                  </tr>
                ))}
              </tbody>
            </table>

            <footer className="table-footer">
              <span>
                Показано {items.length} из {total}
              </span>

              <div className="pagination">
                <button
                  type="button"
                  disabled={!hasPreviousPage}
                  onClick={() => void loadAuditLogs(page - 1)}
                >
                  Назад
                </button>

                <span>Страница {page}</span>

                <button
                  type="button"
                  disabled={!hasNextPage}
                  onClick={() => void loadAuditLogs(page + 1)}
                >
                  Вперёд
                </button>
              </div>
            </footer>
          </>
        ) : null}
      </section>
    </main>
  );
}
