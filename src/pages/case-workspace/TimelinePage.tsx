import { useEffect, useMemo, useState } from "react";

import { getMaterials } from "../../features/materials/api/materialsApi";
import { getObjects } from "../../features/objects/api/objectsApi";
import { TimelineFiltersPanel } from "../../features/timeline/ui/TimelineFiltersPanel";
import { CreateEventModal } from "../../features/timeline/ui/CreateEventModal";
import { EventCardModal } from "../../features/timeline/ui/EventCardModal";
import { getTimeline, toggleEventReportInclude } from "../../features/timeline/api/timelineApi";
import {
  getDatePrecisionLabel,
  getEventTypeLabel,
} from "../../features/timeline/model/timelineOptions";
import type { TimelineEventDto } from "../../features/timeline/model/timelineTypes";
import { EventReportIncludeButton } from "../../features/timeline/ui/EventReportIncludeButton";
import {
  replaceTimelineEvent,
  setTimelineEventIncludeInReport,
} from "../../features/timeline/lib/replaceTimelineEvent";
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
  const [togglingEventId, setTogglingEventId] = useState<string | null>(null);
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

  async function handleToggleIncludeInReport(
    eventId: string,
    currentValue: boolean,
  ) {
    if (readonly || togglingEventId) {
      return;
    }

    const nextValue = !currentValue;
    const previousItems = items;

    setTogglingEventId(eventId);
    setErrorMessage("");

    setItems((current) =>
      setTimelineEventIncludeInReport(current, eventId, nextValue),
    );

    try {
      const result = await toggleEventReportInclude({
        caseId,
        eventId,
        includeInReport: nextValue,
      });

      setItems((current) => replaceTimelineEvent(current, result.eventItem));
    } catch {
      setItems(previousItems);
      setErrorMessage("Не удалось изменить включение события в DOCX");
    } finally {
      setTogglingEventId(null);
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
    <section>
      <div
        style={{
          display: "flex",
          justifyContent: "space-between",
          alignItems: "center",
          marginBottom: "var(--space-4)",
        }}
      >
        <div>
          <h2 style={{ margin: 0, color: "var(--text-primary)", fontSize: 18 }}>
            Хронология
          </h2>
          <p style={{ margin: "var(--space-1) 0 0", color: "var(--text-muted)", fontSize: 12 }}>
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

      {errorMessage && (
        <div
          style={{
            padding: "var(--space-2) var(--space-3)",
            border: "1px solid var(--danger)",
            borderRadius: "var(--radius-sm)",
            background: "color-mix(in srgb, var(--danger) 10%, transparent)",
            color: "var(--danger)",
            marginBottom: "var(--space-4)",
            fontSize: 13,
          }}
        >
          {errorMessage}
        </div>
      )}

      <TimelineFiltersPanel
        filters={filters}
        objectOptions={objectOptions}
        materialOptions={materialOptions}
        onChange={setFilters}
        onApply={applyFilters}
        onReset={resetFilters}
      />

      {isLoading ? (
        <p style={{ color: "var(--text-muted)" }}>Загрузка хронологии...</p>
      ) : items.length === 0 ? (
        <div
          style={{
            marginTop: "var(--space-5)",
            padding: "var(--space-6)",
            border: "1px dashed var(--border-subtle)",
            borderRadius: "var(--radius-md)",
            color: "var(--text-muted)",
            fontSize: 13,
          }}
        >
          <h3 style={{ margin: "0 0 var(--space-2)", color: "var(--text-primary)" }}>
            Событий пока нет
          </h3>
          <p style={{ margin: 0 }}>Создайте первое событие, чтобы начать формировать хронологию дела.</p>
        </div>
      ) : (
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
              <th style={{ padding: "var(--space-2) var(--space-3)" }}>Дата</th>
              <th style={{ padding: "var(--space-2) var(--space-3)" }}>Тип</th>
              <th style={{ padding: "var(--space-2) var(--space-3)" }}>Название</th>
              <th style={{ padding: "var(--space-2) var(--space-3)" }}>Объекты</th>
              <th style={{ padding: "var(--space-2) var(--space-3)" }}>Материалы</th>
              <th style={{ padding: "var(--space-2) var(--space-3)" }}>DOCX</th>
              <th style={{ padding: "var(--space-2) var(--space-3)" }}>Действия</th>
            </tr>
          </thead>
          <tbody>
            {items.map((item) => (
              <tr
                key={item.id}
                style={{
                  borderBottom: "1px solid var(--border-subtle)",
                  fontSize: 13,
                }}
              >
                <td style={{ padding: "var(--space-3)" }}>{item.eventCode}</td>
                <td style={{ padding: "var(--space-3)" }}>
                  {item.eventDate}
                  {item.eventTime ? ` ${item.eventTime}` : ""}
                  <div style={{ color: "var(--text-muted)", fontSize: 11 }}>
                    {getDatePrecisionLabel(item.datePrecision)}
                  </div>
                </td>
                <td style={{ padding: "var(--space-3)" }}>
                  {getEventTypeLabel(item.eventType)}
                </td>
                <td style={{ padding: "var(--space-3)" }}>
                  <strong>{item.title}</strong>
                  {item.description && (
                    <div style={{ color: "var(--text-muted)", fontSize: 11 }}>
                      {item.description}
                    </div>
                  )}
                </td>
                <td style={{ padding: "var(--space-3)" }}>
                  {item.linkedObjectCount}
                </td>
                <td style={{ padding: "var(--space-3)" }}>
                  {item.linkedMaterialCount}
                </td>
                <td style={{ padding: "var(--space-3)" }}>
                  <EventReportIncludeButton
                    includeInReport={item.includeInReport}
                    disabled={readonly || Boolean(togglingEventId)}
                    loading={togglingEventId === item.id}
                    onToggle={() =>
                      void handleToggleIncludeInReport(
                        item.id,
                        item.includeInReport,
                      )
                    }
                  />
                </td>
                <td style={{ padding: "var(--space-3)" }}>
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
