import { useEffect, useState } from "react";
import { useCaseSummary } from "./model/useCaseSummary";
import type { CaseWorkspaceSection } from "../../shared/types/workspaceTypes";

// ===================================================================
// Структура групп сайдбара
// ===================================================================

type SidebarItemDef = {
  section: CaseWorkspaceSection;
  label: string;
  icon: string;
  /** Ключ в CaseSummaryDto для счётчика (undefined = без счётчика) */
  countKey?: "objectCount" | "materialCount" | "relationCount" | "eventCount";
};

type SidebarGroupDef = {
  title: string;
  icon: string;
  items: SidebarItemDef[];
};

const sidebarGroups: SidebarGroupDef[] = [
  {
    title: "ОБЗОР",
    icon: "◈",
    items: [{ section: "overview", label: "Карточка дела", icon: "◈" }],
  },
  {
    title: "ДАННЫЕ",
    icon: "⌕",
    items: [
      { section: "objects", label: "Объекты", icon: "⌕", countKey: "objectCount" },
      { section: "materials", label: "Материалы", icon: "📎", countKey: "materialCount" },
      { section: "relations", label: "Связи", icon: "⇄", countKey: "relationCount" },
    ],
  },
  {
    title: "АНАЛИЗ",
    icon: "⇄",
    items: [
      { section: "graph", label: "Граф", icon: "🕸" },
      { section: "timeline", label: "Хронология", icon: "◷", countKey: "eventCount" },
    ],
  },
  {
    title: "РЕЗУЛЬТАТ",
    icon: "📄",
    items: [{ section: "report", label: "Справка", icon: "📄" }],
  },
];

// ===================================================================
// Пропсы
// ===================================================================

type Props = {
  activeSection: CaseWorkspaceSection;
  onSectionChange: (section: CaseWorkspaceSection) => void;
  onBackToCases: () => void;
  caseId: string;
};

// ===================================================================
// Компонент
// ===================================================================

export function CaseSidebar({
  activeSection,
  onSectionChange,
  onBackToCases,
  caseId,
}: Props) {
  const [expanded, setExpanded] = useState(true);
  const { summary } = useCaseSummary(caseId);

  // Закрывать сайдбар при смене дела
  useEffect(() => {
    setExpanded(true);
  }, [caseId]);

  const sidebarWidth = expanded
    ? "var(--sidebar-width)"
    : "var(--sidebar-collapsed-width)";

  const itemStyle = (section: CaseWorkspaceSection): React.CSSProperties => {
    const active = section === activeSection;
    return {
      display: "flex",
      alignItems: "center",
      gap: "var(--space-3)",
      width: "100%",
      padding: expanded ? "8px 12px" : "8px 0",
      border: "none",
      borderLeft: active ? "3px solid var(--accent)" : "3px solid transparent",
      background: active ? "var(--bg-selected)" : "none",
      color: active ? "var(--accent)" : "var(--text-secondary)",
      cursor: "pointer",
      fontSize: 13,
      textAlign: "left",
      boxSizing: "border-box",
      justifyContent: expanded ? "flex-start" : "center",
      borderRadius: 0,
      transition: "background 100ms, color 100ms",
    };
  };

  return (
    <aside
      style={{
        width: sidebarWidth,
        background: "var(--bg-sidebar)",
        borderRight: "1px solid var(--border-subtle)",
        display: "flex",
        flexDirection: "column",
        transition: "width 200ms ease",
        overflow: "hidden",
        flexShrink: 0,
        boxSizing: "border-box",
      }}
    >
      {/* Кнопка сворачивания */}
      <div
        style={{
          display: "flex",
          justifyContent: expanded ? "flex-end" : "center",
          padding: "var(--space-2)",
          borderBottom: "1px solid var(--border-subtle)",
        }}
      >
        <button
          type="button"
          onClick={() => setExpanded((prev) => !prev)}
          title={expanded ? "Свернуть" : "Развернуть"}
          style={{
            background: "none",
            border: "1px solid var(--border-subtle)",
            borderRadius: "var(--radius-sm)",
            color: "var(--text-muted)",
            cursor: "pointer",
            padding: "4px 6px",
            fontSize: 12,
            lineHeight: 1,
          }}
        >
          {expanded ? "◀" : "▶"}
        </button>
      </div>

      {/* Группы */}
      <nav style={{ flex: 1, overflowY: "auto", padding: "var(--space-2) 0" }}>
        {sidebarGroups.map((group) => (
          <div key={group.title} style={{ marginBottom: "var(--space-2)" }}>
            {/* Заголовок группы */}
            {expanded && (
              <div
                style={{
                  padding: "6px 12px",
                  fontSize: 10,
                  fontWeight: 600,
                  color: "var(--text-muted)",
                  letterSpacing: "0.5px",
                  textTransform: "uppercase",
                }}
              >
                {group.icon} {group.title}
              </div>
            )}

            {/* Элементы группы */}
            {group.items.map((item) => {
              const count =
                item.countKey && summary
                  ? (summary[item.countKey] as number)
                  : undefined;

              return (
                <button
                  key={item.section}
                  type="button"
                  onClick={() => onSectionChange(item.section)}
                  title={expanded ? undefined : item.label}
                  style={itemStyle(item.section)}
                >
                  <span style={{ fontSize: 16, flexShrink: 0 }}>
                    {item.icon}
                  </span>

                  {expanded && (
                    <>
                      <span style={{ flex: 1, minWidth: 0, overflow: "hidden", textOverflow: "ellipsis", whiteSpace: "nowrap" }}>
                        {item.label}
                      </span>

                      {count !== undefined && (
                        <span
                          style={{
                            fontSize: 11,
                            color: "var(--text-muted)",
                            background: "var(--bg-surface)",
                            borderRadius: "var(--radius-sm)",
                            padding: "0 6px",
                            lineHeight: "18px",
                            minWidth: 20,
                            textAlign: "center",
                            fontFamily: "var(--font-mono)",
                          }}
                        >
                          {count}
                        </span>
                      )}
                    </>
                  )}
                </button>
              );
            })}
          </div>
        ))}
      </nav>

      {/* Разделитель + кнопка назад */}
      <div
        style={{
          borderTop: "1px solid var(--border-subtle)",
          padding: "var(--space-2)",
        }}
      >
        <button
          type="button"
          onClick={onBackToCases}
          title="Все дела"
          style={{
            display: "flex",
            alignItems: "center",
            justifyContent: expanded ? "flex-start" : "center",
            gap: "var(--space-2)",
            width: "100%",
            padding: "8px 12px",
            border: "none",
            background: "none",
            color: "var(--text-muted)",
            cursor: "pointer",
            fontSize: 13,
          }}
        >
          <span style={{ fontSize: 14 }}>←</span>
          {expanded && <span>Все дела</span>}
        </button>
      </div>
    </aside>
  );
}
