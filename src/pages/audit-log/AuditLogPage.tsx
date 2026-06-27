import { useEffect, useState } from "react";

import type { CurrentUserDto } from "../../features/auth/model/authTypes";
import {
  exportAuditLog,
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
  ExportAuditLogResponse,
} from "../../features/audit/model/auditTypes";
import {
  buildAuditLogExportPayload,
  buildAuditLogsPayload,
  emptyAuditLogFilters,
  type AuditLogFilters,
} from "../../features/audit/model/auditLogFilters";
import { AuditLogDetailsPanel } from "../../features/audit/ui/AuditLogDetailsPanel";
import { AuditLogPagination } from "../../features/audit/ui/AuditLogPagination";
import { AuditLogTable } from "../../features/audit/ui/AuditLogTable";
import { AuditLogToolbar } from "../../features/audit/ui/AuditLogToolbar";
import { formatError } from "../../shared/lib/formatError";

const PAGE_SIZE = 25;

type Props = {
  user: CurrentUserDto;
  onBack: () => void;
};

export function AuditLogPage({ user, onBack }: Props) {
  const isAdministrator = user.role === "administrator";

  const [items, setItems] = useState<AuditLogDto[]>([]);
  const [total, setTotal] = useState(0);
  const [page, setPage] = useState(1);

  const [filters, setFilters] = useState<AuditLogFilters>(emptyAuditLogFilters);

  const [loading, setLoading] = useState(false);
  const [errorMessage, setErrorMessage] = useState("");

  const [actionOptions, setActionOptions] = useState<AuditActionOptionDto[]>([]);
  const [userOptions, setUserOptions] = useState<AuditUserOptionDto[]>([]);
  const [dictionaryErrorMessage, setDictionaryErrorMessage] = useState("");

  const [selectedAuditLogId, setSelectedAuditLogId] = useState<string | null>(null);
  const [selectedAuditLog, setSelectedAuditLog] =
    useState<AuditLogDetailsDto | null>(null);
  const [detailsLoading, setDetailsLoading] = useState(false);
  const [detailsErrorMessage, setDetailsErrorMessage] = useState("");

  const [exporting, setExporting] = useState(false);
  const [exportErrorMessage, setExportErrorMessage] = useState("");
  const [exportResponse, setExportResponse] = useState<ExportAuditLogResponse | null>(null);

  function closeDetailsPanel() {
    setSelectedAuditLogId(null);
    setSelectedAuditLog(null);
    setDetailsErrorMessage("");
    setDetailsLoading(false);
  }

  async function loadAuditLogs(
    nextPage = page,
    nextFilters: AuditLogFilters = filters,
  ) {
    setLoading(true);
    setErrorMessage("");

    const payload = buildAuditLogsPayload({
      filters: nextFilters,
      page: nextPage,
      pageSize: PAGE_SIZE,
      isAdministrator,
    });

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

    if (!isAdministrator) {
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
    void loadAuditLogs(1, filters);
    void loadDictionaries();
    // eslint-disable-next-line react-hooks/exhaustive-deps
  }, []);

  function applyFilters() {
    setPage(1);
    void loadAuditLogs(1, filters);
  }

  function resetFilters() {
    setFilters(emptyAuditLogFilters);
    setPage(1);
    void loadAuditLogs(1, emptyAuditLogFilters);
  }

  function changePage(nextPage: number) {
    void loadAuditLogs(nextPage, filters);
  }

  async function exportLog() {
    setExporting(true);
    setExportErrorMessage("");
    setExportResponse(null);

    const payload = buildAuditLogExportPayload({
      filters,
      isAdministrator,
    });

    try {
      const response = await exportAuditLog(payload);

      setExportResponse(response);
    } catch (err) {
      setExportErrorMessage(formatError(err));
    } finally {
      setExporting(false);
    }
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

  return (
    <main className="page audit-log-page" style={{ padding: 32 }}>
      <header className="page-header">
        <div>
          <h1>Журнал действий</h1>
          <p>Просмотр значимых событий, ошибок и изменений.</p>
        </div>

        <div style={{ display: "flex", gap: 8 }}>
          {isAdministrator ? (
            <button
              type="button"
              disabled={exporting}
              onClick={() => void exportLog()}
            >
              {exporting ? "Экспорт..." : "Экспорт CSV"}
            </button>
          ) : null}
          <button
            type="button"
            onClick={() => {
              void loadAuditLogs(page, filters);
              void loadDictionaries();
            }}
          >
            Обновить
          </button>
          <button type="button" onClick={onBack}>
            Назад к делам
          </button>
        </div>
      </header>

      {errorMessage ? <div className="error-box">{errorMessage}</div> : null}

      {exportErrorMessage ? (
        <div className="error-box">{exportErrorMessage}</div>
      ) : null}

      {exportResponse ? (
        <div className="success-box">
          Экспорт завершён: {exportResponse.exportedCount} записей,
          файл: {exportResponse.filePath}
        </div>
      ) : null}

      <AuditLogToolbar
        filters={filters}
        actionOptions={actionOptions}
        userOptions={userOptions}
        isAdministrator={isAdministrator}
        loading={loading}
        dictionaryErrorMessage={dictionaryErrorMessage}
        onFiltersChange={setFilters}
        onApply={applyFilters}
        onReset={resetFilters}
        onRefreshDictionaries={() => void loadDictionaries()}
      />

      <div className="audit-log-layout">
        <div className="audit-log-main">
          <AuditLogTable
            items={items}
            selectedAuditLogId={selectedAuditLogId}
            loading={loading}
            onSelect={(auditLogId) => void selectAuditLog(auditLogId)}
          />

          <AuditLogPagination
            page={page}
            pageSize={PAGE_SIZE}
            total={total}
            loading={loading}
            onPageChange={changePage}
          />
        </div>

        <AuditLogDetailsPanel
          item={selectedAuditLog}
          loading={detailsLoading}
          errorMessage={detailsErrorMessage}
          onClose={closeDetailsPanel}
        />
      </div>
    </main>
  );
}
