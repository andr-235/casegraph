import {
  getRelationConfidenceLabel,
  getRelationTypeLabel,
  relationConfidenceOptions,
  relationTypeOptions,
} from "../../relations/model/relationConstants";
import {
  getObjectTypeLabel,
  objectTypeOptions,
} from "../../objects/model/objectOptions";
import type { GraphFilters } from "../lib/filterGraphData";

type GraphFiltersPanelProps = {
  filters: GraphFilters;
  onChange: (filters: GraphFilters) => void;
  onReset: () => void;
};

export function GraphFiltersPanel({
  filters,
  onChange,
  onReset,
}: GraphFiltersPanelProps) {
  return (
    <section className="graph-filters-panel">
      <div className="graph-filters-header">
        <h2>Фильтры графа</h2>

        <button type="button" onClick={onReset}>
          Сбросить
        </button>
      </div>

      <div className="graph-filters-grid">
        <label>
          <span>Тип объекта</span>
          <select
            value={filters.objectType}
            onChange={(event) =>
              onChange({ ...filters, objectType: event.target.value })
            }
          >
            <option value="all">Все типы</option>
            {objectTypeOptions.map((option) => (
              <option key={option.value} value={option.value}>
                {getObjectTypeLabel(option.value)}
              </option>
            ))}
          </select>
        </label>

        <label>
          <span>Тип связи</span>
          <select
            value={filters.relationType}
            onChange={(event) =>
              onChange({ ...filters, relationType: event.target.value })
            }
          >
            <option value="all">Все типы</option>
            {relationTypeOptions.map((option) => (
              <option key={option.value} value={option.value}>
                {getRelationTypeLabel(option.value)}
              </option>
            ))}
          </select>
        </label>

        <label>
          <span>Достоверность</span>
          <select
            value={filters.confidenceLevel}
            onChange={(event) =>
              onChange({ ...filters, confidenceLevel: event.target.value })
            }
          >
            <option value="all">Любая</option>
            {relationConfidenceOptions.map((option) => (
              <option key={option.value} value={option.value}>
                {getRelationConfidenceLabel(option.value)}
              </option>
            ))}
          </select>
        </label>

        <label className="graph-filter-checkbox">
          <input
            type="checkbox"
            checked={filters.onlyKeyObjects}
            onChange={(event) =>
              onChange({ ...filters, onlyKeyObjects: event.target.checked })
            }
          />
          <span>Только ключевые объекты</span>
        </label>
      </div>
    </section>
  );
}
