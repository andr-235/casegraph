import { useCallback, useEffect, useMemo, useRef, useState } from "react";
import { getObjects } from "../../features/objects/api/objectsApi";
import { getObjectTypeLabel } from "../../features/objects/model/objectOptions";
import type { ObjectListItemDto, ObjectType } from "../../features/objects/model/objectTypes";
import { ObjectEditModal } from "../../features/objects/ui/ObjectEditModal";
import { CreateObjectModal } from "../../features/objects/ui/CreateObjectModal";
import type { CaseDto } from "../../features/cases/model/caseTypes";

// ===================================================================
// Типы фильтров
// ===================================================================

type FilterValue = "all" | "yes" | "no";

interface Filters {
  search: string;
  objectType: ObjectType | "all";
  isKey: FilterValue;
  includeInReport: FilterValue;
}

const allFilterOptions: ObjectType[] = [
  "person", "account", "phone", "address", "vehicle",
  "organization", "document", "image", "publication",
  "event", "source", "other",
];

// ===================================================================
// Пропсы
// ===================================================================

type Props = {
  caseItem: CaseDto;
  /** Вызывается при клике на строку — открыть Inspector */
  onInspectorOpen: (objectId: string) => void;
  /** Инвалидация Inspector после создания объекта */
  onInspectorInvalidate: () => void;
  /** Регистрирует внутренний обработчик обновления для вызова из Inspector */
  onRegisterUpdateHandler: (handler: (item: ObjectListItemDto) => void) => void;
};

// ===================================================================
// Компонент
// ===================================================================

export function ObjectsPage({
  caseItem,
  onInspectorOpen,
  onInspectorInvalidate,
  onRegisterUpdateHandler,
}: Props) {
  const [items, setItems] = useState<ObjectListItemDto[]>([]);
  const [isLoading, setIsLoading] = useState(true);
  const [error, setError] = useState("");
  const [isCreateOpen, setIsCreateOpen] = useState(false);
  const [editingObjectId, setEditingObjectId] = useState<string | null>(null);
  const [selectedId, setSelectedId] = useState<string | null>(null);
  const [filters, setFilters] = useState<Filters>({
    search: "",
    objectType: "all",
    isKey: "all",
    includeInReport: "all",
  });

  // Ref для управления фокусом при фильтрации
  const searchRef = useRef<HTMLInputElement>(null);

  // ================================================================
  // Загрузка
  // ================================================================

  const loadObjects = useCallback(async () => {
    setIsLoading(true);
    setError("");

    try {
      const response = await getObjects(caseItem.id);
      setItems(response.items);
      log.debug(`Loaded ${response.items.length} objects`);
    } catch (unknownError) {
      setError(
        unknownError instanceof Error
          ? unknownError.message
          : "Не удалось загрузить объекты.",
      );
    } finally {
      setIsLoading(false);
    }
  }, [caseItem.id]);

  useEffect(() => {
    loadObjects();
  }, [loadObjects]);

  // ================================================================
  // Фильтрация (client-side, useMemo)
  // ================================================================

  const filteredItems = useMemo(() => {
    let result = items;

    // Поиск по названию и значению
    if (filters.search.trim()) {
      const q = filters.search.trim().toLowerCase();
      result = result.filter(
        (item) =>
          item.title.toLowerCase().includes(q) ||
          (item.value ?? "").toLowerCase().includes(q),
      );
    }

    // Тип
    if (filters.objectType !== "all") {
      result = result.filter((item) => item.objectType === filters.objectType);
    }

    // Ключевой
    if (filters.isKey !== "all") {
      result = result.filter((item) =>
        filters.isKey === "yes" ? item.isKey : !item.isKey,
      );
    }

    // В справке
    if (filters.includeInReport !== "all") {
      result = result.filter((item) =>
        filters.includeInReport === "yes" ? item.includeInReport : !item.includeInReport,
      );
    }

    log.debug(
      `Filter: ${items.length} → ${result.length} (search="${filters.search}" type=${filters.objectType} key=${filters.isKey} report=${filters.includeInReport})`,
    );

    return result;
  }, [items, filters]);

  // ================================================================
  // Обработчики
  // ================================================================

  const handleCreated = useCallback(
    (objectItem: ObjectListItemDto) => {
      setIsCreateOpen(false);
      setItems((prev) => [objectItem, ...prev]);
      onInspectorInvalidate();
    },
    [onInspectorInvalidate],
  );

  const handleRowClick = useCallback(
    (item: ObjectListItemDto) => {
      log.info(`Row click → Inspector: ${item.objectCode}`);
      setSelectedId(item.id);
      onInspectorOpen(item.id);
    },
    [onInspectorOpen],
  );

  const handleRowDoubleClick = useCallback(
    (item: ObjectListItemDto) => {
      log.info(`Double click → EditModal: ${item.objectCode}`);
      setEditingObjectId(item.id);
    },
    [],
  );

  const handleRowKeyDown = useCallback(
    (e: React.KeyboardEvent, item: ObjectListItemDto) => {
      if (e.key === "Enter") {
        e.preventDefault();
        handleRowDoubleClick(item);
      }
    },
    [handleRowDoubleClick],
  );

  // Регистрируем обработчик обновления из Inspector
  const handleInspectorUpdate = useCallback(
    (updated: ObjectListItemDto) => {
      setItems((prev) =>
        prev.map((item) => (item.id === updated.id ? updated : item)),
      );
    },
    [],
  );

  // Передаём обработчик наверх при монтировании
  useEffect(() => {
    onRegisterUpdateHandler(handleInspectorUpdate);
  }, [onRegisterUpdateHandler, handleInspectorUpdate]);

  const handleEditUpdated = useCallback(
    (updated: ObjectListItemDto) => {
      setItems((prev) =>
        prev.map((item) => (item.id === updated.id ? updated : item)),
      );
      setEditingObjectId(null);
    },
    [],
  );

  const handleEditDeleted = useCallback(
    (deletedId: string) => {
      setItems((prev) => prev.filter((item) => item.id !== deletedId));
      setEditingObjectId(null);
      setSelectedId((prev) => (prev === deletedId ? null : prev));
    },
    [],
  );

  const resetFilters = useCallback(() => {
    setFilters({
      search: "",
      objectType: "all",
      isKey: "all",
      includeInReport: "all",
    });
    searchRef.current?.focus();
  }, []);

  // ================================================================
  // Рендер
  // ================================================================

  return (
    <section style={{ display: "flex", flexDirection: "column", gap: "var(--space-4)" }}>
      {/* Заголовок + кнопка создания */}
      <div style={{ display: "flex", alignItems: "center", justifyContent: "space-between" }}>
        <h2 style={{ margin: 0, fontSize: 18, fontWeight: 600, color: "var(--text-primary)" }}>
          Объекты
        </h2>

        <button
          type="button"
          onClick={() => setIsCreateOpen(true)}
          style={{
            padding: "8px 16px",
            borderRadius: "var(--radius-sm)",
            border: "none",
            background: "var(--accent)",
            color: "#fff",
            cursor: "pointer",
            fontSize: 13,
            fontWeight: 600,
          }}
        >
          + Объект
        </button>
      </div>

      {/* Фильтр-бар */}
      <div
        style={{
          display: "flex",
          alignItems: "center",
          gap: "var(--space-3)",
          flexWrap: "wrap",
        }}
      >
        <input
          ref={searchRef}
          type="text"
          placeholder="Поиск по названию или значению…"
          value={filters.search}
          onChange={(e) => setFilters((prev) => ({ ...prev, search: e.target.value }))}
          style={{
            flex: "1 1 200px",
            padding: "6px 10px",
            borderRadius: "var(--radius-sm)",
            border: "1px solid var(--border-default)",
            background: "var(--bg-surface)",
            color: "var(--text-primary)",
            fontSize: 13,
            outline: "none",
          }}
        />

        <select
          value={filters.objectType}
          onChange={(e) =>
            setFilters((prev) => ({
              ...prev,
              objectType: e.target.value as ObjectType | "all",
            }))
          }
          style={{
            padding: "6px 10px",
            borderRadius: "var(--radius-sm)",
            border: "1px solid var(--border-default)",
            background: "var(--bg-surface)",
            color: "var(--text-primary)",
            fontSize: 13,
            outline: "none",
          }}
        >
          <option value="all">Все типы</option>
          {allFilterOptions.map((t) => (
            <option key={t} value={t}>
              {getObjectTypeLabel(t)}
            </option>
          ))}
        </select>

        <select
          value={filters.isKey}
          onChange={(e) =>
            setFilters((prev) => ({
              ...prev,
              isKey: e.target.value as FilterValue,
            }))
          }
          style={{
            padding: "6px 10px",
            borderRadius: "var(--radius-sm)",
            border: "1px solid var(--border-default)",
            background: "var(--bg-surface)",
            color: "var(--text-primary)",
            fontSize: 13,
            outline: "none",
          }}
        >
          <option value="all">Все</option>
          <option value="yes">Ключевые</option>
          <option value="no">Обычные</option>
        </select>

        <select
          value={filters.includeInReport}
          onChange={(e) =>
            setFilters((prev) => ({
              ...prev,
              includeInReport: e.target.value as FilterValue,
            }))
          }
          style={{
            padding: "6px 10px",
            borderRadius: "var(--radius-sm)",
            border: "1px solid var(--border-default)",
            background: "var(--bg-surface)",
            color: "var(--text-primary)",
            fontSize: 13,
            outline: "none",
          }}
        >
          <option value="all">В справке: все</option>
          <option value="yes">В справке</option>
          <option value="no">Не в справке</option>
        </select>

        <button
          type="button"
          onClick={resetFilters}
          style={{
            padding: "6px 12px",
            borderRadius: "var(--radius-sm)",
            border: "1px solid var(--border-subtle)",
            background: "none",
            color: "var(--text-secondary)",
            cursor: "pointer",
            fontSize: 12,
          }}
        >
          Сбросить
        </button>
      </div>

      {/* Ошибка */}
      {error && (
        <div style={{ color: "var(--danger)", fontSize: 13 }}>{error}</div>
      )}

      {/* Контент */}
      {isLoading ? (
        <div style={{ color: "var(--text-muted)", fontSize: 13, padding: "var(--space-4)" }}>
          Загрузка объектов...
        </div>
      ) : filteredItems.length === 0 ? (
        <div
          style={{
            color: "var(--text-muted)",
            fontSize: 13,
            padding: "var(--space-6)",
            textAlign: "center",
          }}
        >
          {items.length === 0
            ? "Объекты ещё не добавлены."
            : "Ничего не найдено по выбранным фильтрам."}
        </div>
      ) : (
        <div style={{ overflowX: "auto" }}>
          <table
            style={{
              width: "100%",
              borderCollapse: "collapse",
              fontSize: 13,
            }}
          >
            <thead>
              <tr
                style={{
                  borderBottom: "1px solid var(--border-subtle)",
                  color: "var(--text-muted)",
                  fontSize: 11,
                  fontWeight: 600,
                  textTransform: "uppercase",
                  letterSpacing: "0.5px",
                }}
              >
                <th style={{ padding: "8px 10px", textAlign: "left", whiteSpace: "nowrap" }}>Код</th>
                <th style={{ padding: "8px 10px", textAlign: "left", whiteSpace: "nowrap" }}>Тип</th>
                <th style={{ padding: "8px 10px", textAlign: "left" }}>Название</th>
                <th style={{ padding: "8px 10px", textAlign: "right", whiteSpace: "nowrap" }}>Мат.</th>
                <th style={{ padding: "8px 10px", textAlign: "right", whiteSpace: "nowrap" }}>Связи</th>
                <th style={{ padding: "8px 10px", textAlign: "center", whiteSpace: "nowrap" }}>Ключ.</th>
                <th style={{ padding: "8px 10px", textAlign: "center", whiteSpace: "nowrap" }}>Справка</th>
              </tr>
            </thead>
            <tbody>
              {filteredItems.map((item) => {
                const isSelected = item.id === selectedId;

                return (
                  <tr
                    key={item.id}
                    onClick={() => handleRowClick(item)}
                    onDoubleClick={() => handleRowDoubleClick(item)}
                    onKeyDown={(e) => handleRowKeyDown(e, item)}
                    tabIndex={0}
                    style={{
                      cursor: "pointer",
                      borderBottom: "1px solid var(--border-subtle)",
                      transition: "background 80ms",
                      background: isSelected ? "var(--bg-selected)" : undefined,
                    }}
                    onMouseEnter={(e) => {
                      if (!isSelected) {
                        e.currentTarget.style.background = "var(--bg-hover)";
                      }
                    }}
                    onMouseLeave={(e) => {
                      if (!isSelected) {
                        e.currentTarget.style.background = "none";
                      }
                    }}
                  >
                    <td
                      style={{
                        padding: "8px 10px",
                        fontFamily: "var(--font-mono)",
                        fontSize: 12,
                        color: "var(--text-muted)",
                        whiteSpace: "nowrap",
                      }}
                    >
                      {item.objectCode}
                    </td>
                    <td style={{ padding: "8px 10px", color: "var(--text-secondary)", whiteSpace: "nowrap" }}>
                      {getObjectTypeLabel(item.objectType)}
                    </td>
                    <td style={{ padding: "8px 10px", color: "var(--text-primary)" }}>
                      {item.title}
                    </td>
                    <td style={{ padding: "8px 10px", textAlign: "right", color: "var(--text-secondary)", fontFamily: "var(--font-mono)", fontSize: 12 }}>
                      {item.linkedMaterialCount}
                    </td>
                    <td style={{ padding: "8px 10px", textAlign: "right", color: "var(--text-secondary)", fontFamily: "var(--font-mono)", fontSize: 12 }}>
                      {item.relationCount}
                    </td>
                    <td style={{ padding: "8px 10px", textAlign: "center", color: item.isKey ? "var(--warning)" : "var(--text-muted)", fontSize: 14 }}>
                      {item.isKey ? "★" : "☆"}
                    </td>
                    <td style={{ padding: "8px 10px", textAlign: "center", color: item.includeInReport ? "var(--success)" : "var(--text-muted)", fontSize: 14 }}>
                      {item.includeInReport ? "●" : "○"}
                    </td>
                  </tr>
                );
              })}
            </tbody>
          </table>
        </div>
      )}

      {/* Модалки */}
      {isCreateOpen && (
        <CreateObjectModal
          caseId={caseItem.id}
          onClose={() => setIsCreateOpen(false)}
          onCreated={handleCreated}
        />
      )}

      {editingObjectId && (
        <ObjectEditModal
          caseId={caseItem.id}
          objectId={editingObjectId}
          onClose={() => setEditingObjectId(null)}
          onUpdated={handleEditUpdated}
          onDeleted={handleEditDeleted}
        />
      )}
    </section>
  );
}

const log = {
  debug: (msg: string) => console.debug(`[ObjectsPage] ${msg}`),
  info: (msg: string) => console.info(`[ObjectsPage] ${msg}`),
};
