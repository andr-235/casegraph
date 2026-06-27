import type {
  AuditActionOptionDto,
  AuditUserOptionDto,
} from "../model/auditTypes";
import type { AuditLogFilters } from "../model/auditLogFilters";

type AuditLogToolbarProps = {
  filters: AuditLogFilters;
  actionOptions: AuditActionOptionDto[];
  userOptions: AuditUserOptionDto[];
  isAdministrator: boolean;
  loading: boolean;
  dictionaryErrorMessage: string;
  onFiltersChange: (filters: AuditLogFilters) => void;
  onApply: () => void;
  onReset: () => void;
  onRefreshDictionaries: () => void;
};

export function AuditLogToolbar({
  filters,
  actionOptions,
  userOptions,
  isAdministrator,
  loading,
  dictionaryErrorMessage,
  onFiltersChange,
  onApply,
  onReset,
  onRefreshDictionaries,
}: AuditLogToolbarProps) {
  function updateFilter<Key extends keyof AuditLogFilters>(
    key: Key,
    value: AuditLogFilters[Key],
  ) {
    onFiltersChange({
      ...filters,
      [key]: value,
    });
  }

  return (
    <section className="audit-toolbar-section">
      <div className="audit-toolbar">
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

        {isAdministrator ? (
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

        <div className="audit-toolbar-actions">
          <button type="button" onClick={onApply} disabled={loading}>
            Применить
          </button>

          <button type="button" onClick={onReset} disabled={loading}>
            Сбросить
          </button>

          <button type="button" onClick={onRefreshDictionaries}>
            Обновить словари
          </button>
        </div>
      </div>

      {dictionaryErrorMessage ? (
        <div className="warning-box">{dictionaryErrorMessage}</div>
      ) : null}
    </section>
  );
}
