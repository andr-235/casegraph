import { useEffect, useMemo, useState } from "react";

import { getMaterials } from "../../features/materials/api/materialsApi";
import { getObjects } from "../../features/objects/api/objectsApi";
import { TimelineFiltersPanel } from "../../features/timeline/ui/TimelineFiltersPanel";
import { CreateEventModal } from "../../features/timeline/ui/CreateEventModal";
import { EventCardModal } from "../../features/timeline/ui/EventCardModal";
import { getTimeline } from "../../features/timeline/api/timelineApi";
import {
  getDatePrecisionLabel,
  getEventTypeLabel,
} from "../../features/timeline/model/timelineOptions";
import type { TimelineEventDto } from "../../features/timeline/model/timelineTypes";
import {
  buildGetTimelinePayload,
  createEmptyTimelineFilters,
  type TimelineFiltersState,
} from "../../features/timeline/model/timelineFilterDefaults";

type TimelinePageProps = {
  caseId: string;
  readonly?: boolean;
};

type SelectOption = {
  id: string;
  label: string;
};

export function TimelinePage({ caseId, readonly = false }: TimelinePageProps) {
  const [items, setItems] = useState<TimelineEventDto[]>([]);
  const [objectOptions, setObjectOptions] = useState<SelectOption[]>([]);
  const [materialOptions, setMaterialOptions] = useState<SelectOption[]>([]);
  const [filters, setFilters] = useState<TimelineFiltersState>(() =>
    createEmptyTimelineFilters(),
  );

  const [isLoading, setIsLoading] = useState(true);
  const [isCreateOpen, setIsCreateOpen] = useState(false);
  const [selectedEventId, setSelectedEventId] = useState<string | null>(null);
  const [errorMessage, setErrorMessage] = useState("");

  async function loadTimelineWithFilters(nextFilters: TimelineFiltersState) {
    setErrorMessage("");
    setIsLoading(true);

    try {
      const result = await getTimeline(
        buildGetTimelinePayload(caseId, nextFilters),
      );
      setItems(result.items);
    } catch (unknownError) {
      setErrorMessage(
        unknownError instanceof Error
          ? unknownError.message
          : "Не удалось загрузить хронологию",
      );
    } finally {
      setIsLoading(false);
    }
  }

  function loadTimeline() {
    void loadTimelineWithFilters(filters);
  }

  function applyFilters() {
    void loadTimeline();
  }

  function resetFilters() {
    const emptyFilters = createEmptyTimelineFilters();
    setFilters(emptyFilters);
    void loadTimelineWithFilters(emptyFilters);
  }

  function closeEventModal() {
    setSelectedEventId(null);
  }

  function refreshTimelineAndCloseEventModal() {
    setSelectedEventId(null);
    void loadTimeline();
  }

  async function loadSelectOptions() {
    try {
      const [objectsResult, materialsResult] = await Promise.all([
        getObjects(caseId),
        getMaterials(caseId),
      ]);

      setObjectOptions(
        objectsResult.items.map((object) => ({
          id: object.id,
          label: `${object.objectCode} · ${object.title}`,
        })),
      );

      setMaterialOptions(
        materialsResult.map((material) => ({
          id: material.id,
          label: `${material.materialCode} · ${material.title}`,
        })),
      );
    } catch {
      // Silently fail - options will just be empty
    }
  }

  useEffect(() => {
    void loadTimeline();
    void loadSelectOptions();
  }, [caseId]);

  const stats = useMemo(() => {
    return {
      total: items.length,
      included: items.filter((item) => item.includeInReport).length,
      withObjects: items.filter((item) => item.linkedObjectCount > 0).length,
      withMaterials: items.filter((item) => item.linkedMaterialCount > 0).length,
    };
  }, [items]);

  return (
    <section className="workspace-section">
      <div className="section-header">
        <div>
          <h1>Хронология</h1>
          <p>
            Событий: {stats.total} · В справку: {stats.included} · С объектами:{" "}
            {stats.withObjects} · С материалами: {stats.withMaterials}
          </p>
        </div>

        {!readonly && (
          <button type="button" onClick={() => setIsCreateOpen(true)}>
            Создать событие
          </button>
        )}
      </div>

      {errorMessage && <div className="error-box">{errorMessage}</div>}

      <TimelineFiltersPanel
        filters={filters}
        objectOptions={objectOptions}
        materialOptions={materialOptions}
        onChange={setFilters}
        onApply={applyFilters}
        onReset={resetFilters}
      />

      {isLoading ? (
        <p>Загрузка хронологии...</p>
      ) : items.length === 0 ? (
        <div className="empty-state">
          <h2>Событий пока нет</h2>
          <p>Создай первое событие, чтобы начать формировать хронологию дела.</p>
        </div>
      ) : (
        <table className="data-table">
          <thead>
            <tr>
              <th>Код</th>
              <th>Дата</th>
              <th>Тип</th>
              <th>Название</th>
              <th>Объекты</th>
              <th>Материалы</th>
              <th>DOCX</th>
              <th>Действия</th>
            </tr>
          </thead>
          <tbody>
            {items.map((item) => (
              <tr key={item.id}>
                <td>{item.eventCode}</td>
                <td>
                  {item.eventDate}
                  {item.eventTime ? ` ${item.eventTime}` : ""}
                  <div className="muted">
                    {getDatePrecisionLabel(item.datePrecision)}
                  </div>
                </td>
                <td>{getEventTypeLabel(item.eventType)}</td>
                <td>
                  <strong>{item.title}</strong>
                  {item.description && (
                    <div className="muted">{item.description}</div>
                  )}
                </td>
                <td>{item.linkedObjectCount}</td>
                <td>{item.linkedMaterialCount}</td>
                <td>{item.includeInReport ? "Да" : "Нет"}</td>
                <td>
                  <button
                    type="button"
                    onClick={() => setSelectedEventId(item.id)}
                  >
                    Открыть
                  </button>
                </td>
              </tr>
            ))}
          </tbody>
        </table>
      )}

      {isCreateOpen && (
        <CreateEventModal
          caseId={caseId}
          objectOptions={objectOptions}
          materialOptions={materialOptions}
          onClose={() => setIsCreateOpen(false)}
          onCreated={() => {
            setIsCreateOpen(false);
            void loadTimeline();
          }}
        />
      )}

      {selectedEventId && (
        <EventCardModal
          caseId={caseId}
          eventId={selectedEventId}
          objectOptions={objectOptions}
          materialOptions={materialOptions}
          readonly={readonly}
          onClose={closeEventModal}
          onSaved={refreshTimelineAndCloseEventModal}
          onDeleted={refreshTimelineAndCloseEventModal}
        />
      )}
    </section>
  );
}
