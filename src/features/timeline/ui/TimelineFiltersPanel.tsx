import { eventTypeOptions } from "../model/timelineOptions";
import type { TimelineFiltersState } from "../model/timelineFilterDefaults";

type LinkOption = {
  id: string;
  label: string;
};

type TimelineFiltersPanelProps = {
  filters: TimelineFiltersState;
  objectOptions: LinkOption[];
  materialOptions: LinkOption[];
  onChange: (filters: TimelineFiltersState) => void;
  onApply: () => void;
  onReset: () => void;
};

export function TimelineFiltersPanel({
  filters,
  objectOptions,
  materialOptions,
  onChange,
  onApply,
  onReset,
}: TimelineFiltersPanelProps) {
  function updateFilter<K extends keyof TimelineFiltersState>(
    key: K,
    value: TimelineFiltersState[K],
  ) {
    onChange({
      ...filters,
      [key]: value,
    });
  }

  return (
    <section className="panel">
      <div className="panel__header">
        <h2>Фильтры хронологии</h2>
      </div>

      <div className="form-grid">
        <label>
          Поиск
          <input
            value={filters.query ?? ""}
            onChange={(event) => updateFilter("query", event.target.value)}
            onKeyDown={(event) => {
              if (event.key === "Enter") {
                onApply();
              }
            }}
            placeholder="EVT-001, название, описание..."
          />
        </label>

        <label>
          Тип события
          <select
            value={filters.eventType ?? ""}
            onChange={(event) => updateFilter("eventType", event.target.value)}
          >
            <option value="">Все типы</option>
            {eventTypeOptions.map((option) => (
              <option key={option.value} value={option.value}>
                {option.label}
              </option>
            ))}
          </select>
        </label>

        <label>
          Объект
          <select
            value={filters.objectId ?? ""}
            onChange={(event) => updateFilter("objectId", event.target.value)}
          >
            <option value="">Все объекты</option>
            {objectOptions.map((option) => (
              <option key={option.id} value={option.id}>
                {option.label}
              </option>
            ))}
          </select>
        </label>

        <label>
          Материал
          <select
            value={filters.materialId ?? ""}
            onChange={(event) => updateFilter("materialId", event.target.value)}
          >
            <option value="">Все материалы</option>
            {materialOptions.map((option) => (
              <option key={option.id} value={option.id}>
                {option.label}
              </option>
            ))}
          </select>
        </label>

        <label>
          Дата с
          <input
            type="date"
            value={filters.dateFrom ?? ""}
            onChange={(event) => updateFilter("dateFrom", event.target.value)}
          />
        </label>

        <label>
          Дата по
          <input
            type="date"
            value={filters.dateTo ?? ""}
            onChange={(event) => updateFilter("dateTo", event.target.value)}
          />
        </label>

        <label>
          В DOCX
          <select
            value={
              filters.includeInReport === undefined
                ? ""
                : filters.includeInReport
                  ? "yes"
                  : "no"
            }
            onChange={(event) => {
              const value = event.target.value;

              updateFilter(
                "includeInReport",
                value === "" ? undefined : value === "yes",
              );
            }}
          >
            <option value="">Все</option>
            <option value="yes">Включено</option>
            <option value="no">Исключено</option>
          </select>
        </label>
      </div>

      <div className="panel__actions">
        <button type="button" onClick={onApply}>
          Применить
        </button>

        <button type="button" onClick={onReset}>
          Сбросить
        </button>
      </div>
    </section>
  );
}
