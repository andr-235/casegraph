export type CaseWorkspaceSection =
  | "overview"
  | "materials"
  | "objects"
  | "relations"
  | "graph"
  | "timeline"
  | "report";

type SidebarItem = {
  section: CaseWorkspaceSection;
  label: string;
  icon: string;
};

const sidebarItems: SidebarItem[] = [
  { section: "overview", label: "Карточка дела", icon: "📁" },
  { section: "materials", label: "Материалы", icon: "📎" },
  { section: "objects", label: "Объекты", icon: "👤" },
  { section: "relations", label: "Связи", icon: "🔗" },
  { section: "graph", label: "Граф", icon: "🕸" },
  { section: "timeline", label: "Хронология", icon: "🕒" },
  { section: "report", label: "Справка", icon: "📄" },
];

type Props = {
  activeSection: CaseWorkspaceSection;
  onSectionChange: (section: CaseWorkspaceSection) => void;
  onBackToCases: () => void;
};

export function CaseSidebar({
  activeSection,
  onSectionChange,
  onBackToCases,
}: Props) {
  return (
    <aside
      style={{
        width: 220,
        borderRight: "1px solid #ddd",
        padding: 16,
        boxSizing: "border-box",
      }}
    >
      <button type="button" onClick={onBackToCases} style={{ marginBottom: 16 }}>
        ← К списку дел
      </button>

      <nav style={{ display: "flex", flexDirection: "column", gap: 8 }}>
        {sidebarItems.map((item) => {
          const active = item.section === activeSection;

          return (
            <button
              key={item.section}
              type="button"
              onClick={() => onSectionChange(item.section)}
              style={{
                textAlign: "left",
                padding: "8px 10px",
                border: active ? "1px solid #222" : "1px solid #ddd",
                background: active ? "#eee" : "white",
                cursor: "pointer",
              }}
            >
              {item.icon} {item.label}
            </button>
          );
        })}
      </nav>
    </aside>
  );
}