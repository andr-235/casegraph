import { useCallback, useEffect, useState } from "react";
import { getObjectById, updateObject } from "../api/objectsApi";
import { getObjectTypeLabel } from "../model/objectOptions";
import type { ObjectDetailsDto, ObjectListItemDto } from "../model/objectTypes";
import { InspectorHeader } from "../../../shared/ui/inspector/InspectorHeader";
import { InspectorSection } from "../../../shared/ui/inspector/InspectorSection";
import { InspectorStat } from "../../../shared/ui/inspector/InspectorStat";

type Props = {
  caseId: string;
  objectId: string;
  revision: number;
  onClose: () => void;
  onOpenFullCard?: () => void;
  onUpdated: (item: ObjectListItemDto) => void;
};

function detailsToList(d: ObjectDetailsDto): ObjectListItemDto {
  return {
    id: d.id,
    caseId: d.caseId,
    objectCode: d.objectCode,
    objectType: d.objectType,
    title: d.title,
    value: d.value,
    description: d.description,
    isKey: d.isKey,
    includeInReport: d.includeInReport,
    linkedMaterialCount: d.linkedMaterialCount,
    relationCount: d.relationCount,
    createdAt: d.createdAt,
    updatedAt: d.updatedAt,
  };
}

export function ObjectInspectorContent({
  caseId,
  objectId,
  revision,
  onClose,
  onOpenFullCard,
  onUpdated,
}: Props) {
  const [data, setData] = useState<ObjectDetailsDto | null>(null);
  const [loading, setLoading] = useState(true);
  const [error, setError] = useState<string | null>(null);
  const [toggling, setToggling] = useState<string | null>(null);

  const load = useCallback(async () => {
    setLoading(true);
    setError(null);

    try {
      const res = await getObjectById(caseId, objectId);
      setData(res.objectItem);
      log.debug(`Inspector loaded: ${res.objectItem.objectCode}`);
    } catch (err) {
      setError(err instanceof Error ? err.message : "Ошибка загрузки объекта");
    } finally {
      setLoading(false);
    }
  }, [caseId, objectId]);

  useEffect(() => {
    load();
  }, [load, revision]);

  const handleToggle = useCallback(
    async (field: "isKey" | "includeInReport") => {
      if (!data) return;

      setToggling(field);

      try {
        const newValue = !data[field];
        const res = await updateObject({
          caseId,
          objectId,
          title: data.title,
          value: data.value ?? undefined,
          description: data.description,
          isKey: field === "isKey" ? newValue : data.isKey,
          includeInReport: field === "includeInReport" ? newValue : data.includeInReport,
          confidenceNote: data.confidenceNote,
        });

        setData(res.objectItem);
        onUpdated(detailsToList(res.objectItem));

        log.info(
          `Toggle ${field}: ${data[field]} → ${newValue} (${data.objectCode})`
        );
      } catch (err) {
        setError(
          err instanceof Error ? err.message : `Ошибка переключения ${field}`
        );
      } finally {
        setToggling(null);
      }
    },
    [data, caseId, objectId, onUpdated]
  );

  if (loading) {
    return (
      <div style={{ color: "var(--text-muted)", fontSize: 13 }}>
        Загрузка объекта...
      </div>
    );
  }

  if (error) {
    return (
      <div style={{ color: "var(--danger)", fontSize: 13 }}>{error}</div>
    );
  }

  if (!data) {
    return (
      <div style={{ color: "var(--text-muted)", fontSize: 13 }}>
        Объект не найден.
      </div>
    );
  }

  return (
    <div style={{ display: "flex", flexDirection: "column", gap: "var(--space-4)" }}>
      <InspectorHeader
        code={data.objectCode}
        title={data.title}
        onClose={onClose}
        onOpenFullCard={onOpenFullCard}
      />

      {/* Тип + ★ */}
      <div style={{ display: "flex", alignItems: "center", gap: "var(--space-2)" }}>
        <span
          style={{
            fontSize: 12,
            color: "var(--text-secondary)",
            background: "var(--bg-elevated)",
            padding: "2px 8px",
            borderRadius: "var(--radius-sm)",
          }}
        >
          {getObjectTypeLabel(data.objectType)}
        </span>

        {data.isKey && (
          <span
            style={{
              fontSize: 14,
              color: "var(--warning)",
              lineHeight: 1,
            }}
            title="Ключевой объект"
          >
            ★
          </span>
        )}
      </div>

      {/* Значение (если есть) */}
      {data.value && (
        <div style={{ fontSize: 13, color: "var(--text-secondary)" }}>
          {data.value}
        </div>
      )}

      {/* Описание (до 3 строк) */}
      {data.description && (
        <div
          style={{
            fontSize: 13,
            color: "var(--text-secondary)",
            lineHeight: 1.5,
            display: "-webkit-box",
            WebkitLineClamp: 3,
            WebkitBoxOrient: "vertical",
            overflow: "hidden",
          }}
          title={data.description}
        >
          {data.description}
        </div>
      )}

      {/* Статистика */}
      <div
        style={{
          display: "grid",
          gridTemplateColumns: "1fr 1fr",
          gap: "var(--space-3)",
        }}
      >
        <InspectorStat label="Связи" value={data.relationCount} />
        <InspectorStat label="Материалы" value={data.linkedMaterialCount} />
      </div>

      {/* Связанные материалы */}
      <InspectorSection title="Связанные материалы">
        {data.linkedMaterials.length === 0 ? (
          <span style={{ fontSize: 12, color: "var(--text-muted)" }}>
            Нет связанных материалов
          </span>
        ) : (
          <div
            style={{
              maxHeight: 200,
              overflowY: "auto",
              display: "flex",
              flexDirection: "column",
              gap: "var(--space-1)",
            }}
          >
            {data.linkedMaterials.map((m) => (
              <div
                key={m.id}
                style={{
                  fontSize: 12,
                  color: "var(--text-secondary)",
                  padding: "4px 6px",
                  borderRadius: "var(--radius-sm)",
                  background: "var(--bg-elevated)",
                }}
              >
                <span style={{ fontFamily: "var(--font-mono)", color: "var(--text-muted)" }}>
                  {m.materialCode}
                </span>
                {" "}
                {m.title}
              </div>
            ))}
          </div>
        )}
      </InspectorSection>

      {/* Быстрые действия */}
      <InspectorSection title="Действия">
        <div style={{ display: "flex", flexDirection: "column", gap: "var(--space-2)" }}>
          <button
            type="button"
            onClick={() => handleToggle("isKey")}
            disabled={toggling !== null}
            style={{
              display: "flex",
              alignItems: "center",
              gap: "var(--space-2)",
              padding: "6px 10px",
              border: "1px solid var(--border-subtle)",
              borderRadius: "var(--radius-sm)",
              background: "none",
              color: data.isKey ? "var(--warning)" : "var(--text-muted)",
              cursor: "pointer",
              fontSize: 12,
            }}
          >
            <span style={{ fontSize: 14 }}>{data.isKey ? "★" : "☆"}</span>
            {data.isKey ? "Убрать ключевой" : "Сделать ключевым"}
          </button>

          <button
            type="button"
            onClick={() => handleToggle("includeInReport")}
            disabled={toggling !== null}
            style={{
              display: "flex",
              alignItems: "center",
              gap: "var(--space-2)",
              padding: "6px 10px",
              border: "1px solid var(--border-subtle)",
              borderRadius: "var(--radius-sm)",
              background: "none",
              color: data.includeInReport ? "var(--success)" : "var(--text-muted)",
              cursor: "pointer",
              fontSize: 12,
            }}
          >
            <span style={{ fontSize: 14 }}>
              {data.includeInReport ? "●" : "○"}
            </span>
            {data.includeInReport ? "Убрать из справки" : "Включить в справку"}
          </button>
        </div>
      </InspectorSection>
    </div>
  );
}

const log = {
  debug: (msg: string) => console.debug(`[ObjectInspectorContent] ${msg}`),
  info: (msg: string) => console.info(`[ObjectInspectorContent] ${msg}`),
};
