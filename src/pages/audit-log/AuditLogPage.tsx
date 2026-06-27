import { useEffect, useState } from "react";

import type { CurrentUserDto } from "../../features/auth/model/authTypes";
import {
  getAuditActions,
  getAuditLogById,
  getAuditLogs,
  getAuditUsers,
} from "../../features/audit/api/auditApi";
import type {
  AuditActionOptionDto,
  AuditLogDetailsDto,
  AuditLogDto,
  AuditUserOptionDto,
  GetAuditLogsPayload,
} from "../../features/audit/model/auditTypes";
import {
  AuditResultBadge,
  AuditSeverityBadge,
} from "../../features/audit/ui/AuditBadges";
import { AuditLogDetailsPanel } from "../../features/audit/ui/AuditLogDetailsPanel";
import { formatError } from "../../shared/lib/formatError";

const DEFAULT_PAGE_SIZE = 50;

type AuditFilters = {
  action: string;
  result: string;
  severity: string;
  userId: string;
  dateFrom: string;
  dateTo: string;
};

const emptyFilters: AuditFilters = {
  action: "",
  result: "",
  severity: "",
  userId: "",
  dateFrom: "",
  dateTo: "",
};

type Props = {
  user: CurrentUserDto;
  onBack: () => void;
};

export function AuditLogPage({ user, onBack }: Props) {
  const isAdmin = user.role === "administrator";

  const [items, setItems] = useState<AuditLogDto[]>([]);
  const [filters, setFilters] = useState<AuditFilters>(emptyFilters);
  const [total, setTotal] = useState(0);
  const [page, setPage] = useState(1);

  const [loading, setLoading] = useState(false);
  const [errorMessage, setErrorMessage] = useState("");

  const [selectedAuditLogId, setSelectedAuditLogId] = useState<string | null>(null);
  const [selectedAuditLog, setSelectedAuditLog] =
    useState<AuditLogDetailsDto | null>(null);
  const [detailsLoading, setDetailsLoading] = useState(false);
  const [detailsErrorMessage, setDetailsErrorMessage] = useState("");

  const [actionOptions, setActionOptions] = useState<AuditActionOptionDto[]>([]);
  const [userOptions, setUserOptions] = useState<AuditUserOptionDto[]>([]);
  const [dictionaryErrorMessage, setDictionaryErrorMessage] = useState("");

  async function loadAuditLogs(
    nextPage = page,
    overrideFilters: AuditFilters = filters,
  ) {
    setLoading(true);
    setErrorMessage("");

    const payload: GetAuditLogsPayload = {
      action: overrideFilters.action || undefined,
      result: overrideFilters.result || undefined,
      severity: overrideFilters.severity || undefined,
      userId: isAdmin && overrideFilters.userId ? overrideFilters.userId : undefined,
      dateFrom: overrideFilters.dateFrom || undefined,
      dateTo: overrideFilters.dateTo || undefined,
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

    closeDetailsPanel();
  }

  async function loadDictionaries() {
    setDictionaryErrorMessage("");

    try {
      const actionsResponse = await getAuditActions();

      setActionOptions(actionsResponse.items);
    } catch (err) {
      setDictionaryErrorMessage(formatError(err));
      return;
    }

    if (!isAdmin) {
      setUserOptions([]);
      return;
    }

    try {
      const usersResponse = await getAuditUsers();

      setUserOptions(usersResponse.items);
    } catch (err) {
      setDictionaryErrorMessage(formatError(err));
    }
  }

  useEffect(() => {
    void loadAuditLogs(1);
    void loadDictionaries();
    // eslint-disable-next-line react-hooks/exhaustive-deps
  }, []);

  function updateFilter<K extends keyof AuditFilters>(
    key: K,
    value: AuditFilters[K],
  ) {
    setFilters((current) => ({ ...current, [key]: value }));
  }

  function applyFilters() {
    setPage(1);
    void loadAuditLogs(1, filters);
  }

  function resetFilters() {
    setFilters(emptyFilters);
    setPage(1);
    void loadAuditLogs(1, emptyFilters);
  }

  async function selectAuditLog(auditLogId: string) {
    setSelectedAuditLogId(auditLogId);
    setSelectedAuditLog(null);
    setDetailsLoading(true);
    setDetailsErrorMessage("");

    try {
      const response = await getAuditLogById({ auditLogId });

      setSelectedAuditLog(response.item);
    } catch (err) {
      setDetailsErrorMessage(formatError(err));
    } finally {
      setDetailsLoading(false);
    }
  }

  function closeDetailsPanel() {
    setSelectedAuditLogId(null);
    setSelectedAuditLog(null);
    setDetailsErrorMessage("");
    setDetailsLoading(false);
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

        <div className="audit-log-layout">
          <div className="audit-log-main">
            <div className="audit-log-toolbar">
              <label className="field">
                <span>Действие</span>
                <select
                  value={filters.action}
                  onChange={(event) => updateFilter("action", event.target.value)}
                >
                  <option value="">Все действия</option>
                  {actionOptions.map((option) => (
                    <option key={option.action} value={option.action}>
                      {option.action} ({option.count})
                    </option>
                  ))}
                </select>
              </label>

              <label className="field">
                <span>Результат</span>
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

              <label className="field">
                <span>Важность</span>
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

              {isAdmin ? (
                <label className="field">
                  <span>Пользователь</span>
                  <select
                    value={filters.userId}
                    onChange={(event) => updateFilter("userId", event.target.value)}
                  >
                    <option value="">Все пользователи</option>
                    {userOptions.map((option) => (
                      <option key={option.userId} value={option.userId}>
                        {option.username} · {option.userRole} ({option.count})
                      </option>
                    ))}
                  </select>
                </label>
              ) : null}

              <label className="field">
                <span>С даты</span>
                <input
                  type="date"
                  value={filters.dateFrom}
                  onChange={(event) => updateFilter("dateFrom", event.target.value)}
                />
              </label>

              <label className="field">
                <span>По дату</span>
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

            {dictionaryErrorMessage ? (
              <div className="warning-box">{dictionaryErrorMessage}</div>
            ) : null}

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
                      <tr
                        key={item.id}
                        className={selectedAuditLogId === item.id ? "selected-row" : undefined}
                        onClick={() => void selectAuditLog(item.id)}
                      >
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
          </div>

          <AuditLogDetailsPanel
            item={selectedAuditLog}
            loading={detailsLoading}
            errorMessage={detailsErrorMessage}
            onClose={closeDetailsPanel}
          />
        </div>
      </section>
    </main>
  );
}
