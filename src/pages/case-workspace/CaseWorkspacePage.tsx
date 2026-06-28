import { useState } from "react";
import type { CurrentUserDto } from "../../features/auth/model/authTypes";
import type { EffectivePermissionsDto } from "../../features/auth/model/effectivePermissionsTypes";
import type { CaseDto } from "../../features/cases/model/caseTypes";
import { CaseOverviewPage } from "./CaseOverviewPage";
import {
  CaseSidebar,
  type CaseWorkspaceSection,
} from "./CaseSidebar";
import { MaterialsPage } from "../materials/MaterialsPage";
import { ObjectsPage } from "./ObjectsPage";
import { GraphPage } from "./GraphPage";
import { RelationsPage } from "./RelationsPage";
import { TimelinePage } from "./TimelinePage";
import { SettingsPage } from "../settings/SettingsPage";
import { can } from "../../shared/lib/permissions";
import { protectedOperations } from "../../shared/security/protectedOperations";

type Props = {
  user: CurrentUserDto;
  permissions: EffectivePermissionsDto | null;
  caseItem: CaseDto;
  onCaseUpdated: (caseItem: CaseDto) => void;
  onBackToCases: () => void;
  onLogout: () => void;
};

const sectionTitles: Record<CaseWorkspaceSection, string> = {
  overview: "Карточка дела",
  materials: "Материалы",
  objects: "Объекты",
  relations: "Связи",
  graph: "Граф связей",
  timeline: "Хронология",
  report: "Справка",
  settings: "Настройки",
};

export function CaseWorkspacePage({
  user,
  permissions,
  caseItem,
  onCaseUpdated,
  onBackToCases,
  onLogout,
}: Props) {
  const [activeSection, setActiveSection] =
    useState<CaseWorkspaceSection>("overview");

  return (
    <main style={{ minHeight: "100vh", display: "flex", flexDirection: "column" }}>
      <header
        style={{
          height: 56,
          borderBottom: "1px solid #ddd",
          padding: "0 24px",
          display: "flex",
          alignItems: "center",
          justifyContent: "space-between",
          boxSizing: "border-box",
        }}
      >
        <div>
          <strong>CaseGraph</strong>
          <span style={{ marginLeft: 16 }}>
            {caseItem.caseCode} · {caseItem.title}
          </span>
        </div>

        <div style={{ display: "flex", alignItems: "center", gap: 12 }}>
          <span>
            {user.displayName} · {user.role}
          </span>

          <button type="button" onClick={onLogout}>
            Выйти
          </button>
        </div>
      </header>

      <div style={{ flex: 1, display: "flex" }}>
        <CaseSidebar
          activeSection={activeSection}
          onSectionChange={setActiveSection}
          onBackToCases={onBackToCases}
          showSettings={can(permissions, protectedOperations.settingsRead)}
        />

        <section style={{ flex: 1, padding: 32 }}>
          {activeSection === "overview" && (
            <CaseOverviewPage
              caseItem={caseItem}
              onCaseUpdated={onCaseUpdated}
            />
          )}

          {activeSection === "materials" && (
            <MaterialsPage caseItem={caseItem} />
          )}

          {activeSection === "objects" && (
            <ObjectsPage caseItem={caseItem} />
          )}

          {activeSection === "relations" && (
            <RelationsPage
              caseId={caseItem.id}
              canEdit={can(permissions, protectedOperations.relationUpdate) || can(permissions, protectedOperations.relationCreate)}
            />
          )}

          {activeSection === "graph" && (
            <GraphPage caseId={caseItem.id} />
          )}

          {activeSection === "timeline" && (
            <TimelinePage
              caseId={caseItem.id}
              readonly={!can(permissions, protectedOperations.timelineCreate) && !can(permissions, protectedOperations.timelineUpdate)}
            />
          )}

          {activeSection === "settings" && can(permissions, protectedOperations.settingsRead) && (
            <SettingsPage
              permissions={permissions}
              onReloadPermissions={undefined}
              onBack={() => setActiveSection("overview")}
            />
          )}

          {activeSection !== "overview" && activeSection !== "materials" && activeSection !== "objects" && activeSection !== "relations" && activeSection !== "graph" && activeSection !== "timeline" && activeSection !== "settings" && (
            <PlaceholderSection
              title={sectionTitles[activeSection]}
              caseItem={caseItem}
            />
          )}
        </section>
      </div>
    </main>
  );
}

type PlaceholderSectionProps = {
  title: string;
  caseItem: CaseDto;
};

function PlaceholderSection({ title, caseItem }: PlaceholderSectionProps) {
  return (
    <section>
      <h2>{title}</h2>

      <p>
        Дело: <strong>{caseItem.caseCode}</strong> · {caseItem.title}
      </p>

      <div
        style={{
          marginTop: 24,
          padding: 24,
          border: "1px dashed #aaa",
          background: "#fafafa",
        }}
      >
        <h3>Раздел пока не реализован</h3>
        <p>
          Это заглушка рабочего раздела. Реальную функциональность подключим
          отдельным vertical slice.
        </p>
      </div>
    </section>
  );
}