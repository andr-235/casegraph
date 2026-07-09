import { InspectorPanel } from "../../../shared/ui/inspector/InspectorPanel";
import { ObjectInspectorContent } from "../../../features/objects/ui/ObjectInspectorContent";
import type { CaseInspectorTarget } from "../../../shared/types/workspaceTypes";
import type { ObjectListItemDto } from "../../../features/objects/model/objectTypes";

type Props = {
  target: CaseInspectorTarget | null;
  revision: number;
  caseId: string;
  onClose: () => void;
  onOpenFullCard?: (target: CaseInspectorTarget) => void;
  onInvalidate: () => void;
  /** Коллбэк обновления объекта из Inspector (для ObjectsPage) */
  onObjectUpdated?: (item: ObjectListItemDto) => void;
};

/**
 * CaseInspector — центральный маршрутизатор контента правой панели.
 * Рендерит соответствующий *InspectorContent в зависимости от target.type.
 */
export function CaseInspector({
  target,
  revision,
  caseId,
  onClose,
  onOpenFullCard,
  onInvalidate,
  onObjectUpdated,
}: Props) {
  return (
    <InspectorPanel visible={target !== null}>
      {target && (
        <InspectorContentSwitch
          target={target}
          revision={revision}
          caseId={caseId}
          onClose={onClose}
          onOpenFullCard={onOpenFullCard}
          onInvalidate={onInvalidate}
          onObjectUpdated={onObjectUpdated}
        />
      )}
    </InspectorPanel>
  );
}

function InspectorContentSwitch({
  target,
  revision,
  caseId,
  onClose,
  onOpenFullCard,
  onInvalidate,
  onObjectUpdated,
}: {
  target: CaseInspectorTarget;
  revision: number;
  caseId: string;
  onClose: () => void;
  onOpenFullCard?: (target: CaseInspectorTarget) => void;
  onInvalidate: () => void;
  onObjectUpdated?: (item: ObjectListItemDto) => void;
}) {
  const handleOpenFullCard = onOpenFullCard
    ? () => onOpenFullCard(target)
    : undefined;

  switch (target.type) {
    case "object":
      return (
        <ObjectInspectorContent
          caseId={caseId}
          objectId={target.id}
          revision={revision}
          onClose={onClose}
          onOpenFullCard={handleOpenFullCard}
          onUpdated={onObjectUpdated ?? (() => {})}
        />
      );

    case "material":
      return (
        <PlaceholderContent
          type="Material"
          caseId={caseId}
          entityId={target.id}
          revision={revision}
          onClose={onClose}
          onOpenFullCard={handleOpenFullCard}
          onInvalidate={onInvalidate}
        />
      );

    case "relation":
      return (
        <PlaceholderContent
          type="Relation"
          caseId={caseId}
          entityId={target.id}
          revision={revision}
          onClose={onClose}
          onOpenFullCard={handleOpenFullCard}
          onInvalidate={onInvalidate}
        />
      );

    case "event":
      return (
        <PlaceholderContent
          type="Event"
          caseId={caseId}
          entityId={target.id}
          revision={revision}
          onClose={onClose}
          onOpenFullCard={handleOpenFullCard}
          onInvalidate={onInvalidate}
        />
      );

    default:
      return (
        <div style={{ color: "var(--text-muted)", fontSize: 13 }}>
          Неизвестный тип сущности
        </div>
      );
  }
}

/**
 * PlaceholderContent — временный контент для Inspector, пока не реализованы
 * конкретные *InspectorContent компоненты.
 */
function PlaceholderContent({
  type,
  entityId,
  revision,
  onClose,
}: {
  type: string;
  caseId: string;
  entityId: string;
  revision: number;
  onClose: () => void;
  onOpenFullCard?: () => void;
  onInvalidate: () => void;
}) {
  return (
    <div
      style={{
        display: "flex",
        flexDirection: "column",
        gap: "var(--space-4)",
      }}
    >
      {/* Заглушка заголовка */}
      <div
        style={{
          display: "flex",
          alignItems: "center",
          justifyContent: "space-between",
          borderBottom: "1px solid var(--border-subtle)",
          paddingBottom: "var(--space-3)",
        }}
      >
        <span
          style={{
            fontFamily: "var(--font-mono)",
            fontSize: 12,
            color: "var(--text-muted)",
          }}
        >
          {type} #{entityId.slice(0, 8)}
        </span>

        <button
          type="button"
          onClick={onClose}
          style={{
            background: "none",
            border: "none",
            color: "var(--text-muted)",
            cursor: "pointer",
            fontSize: 16,
            lineHeight: 1,
            padding: 2,
          }}
        >
          ×
        </button>
      </div>

      <div
        style={{
          color: "var(--text-muted)",
          fontSize: 13,
          fontStyle: "italic",
        }}
      >
        Инспектор {type.toLowerCase()} — будет реализован позже.
        <br />
        revision: {revision}
      </div>
    </div>
  );
}
