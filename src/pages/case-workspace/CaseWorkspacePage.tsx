import { useCallback, useEffect, useState } from "react";
import type { CurrentUserDto } from "../../features/auth/model/authTypes";
import type { EffectivePermissionsDto } from "../../features/auth/model/effectivePermissionsTypes";
import type { CaseDto } from "../../features/cases/model/caseTypes";
import { CaseOverviewPage } from "./CaseOverviewPage";
import type { CaseWorkspaceSection } from "../../shared/types/workspaceTypes";
import { CaseSidebar } from "./CaseSidebar";
import { AppShell } from "../../shared/ui/AppShell";
import { TopBar } from "../../shared/ui/TopBar";
import { CaseInspector } from "./inspector/CaseInspector";
import { useCaseInspector } from "./inspector/useCaseInspector";
import { MaterialsPage } from "../materials/MaterialsPage";
import { ObjectsPage } from "./ObjectsPage";
import { GraphPage } from "./GraphPage";
import { RelationsPage } from "./RelationsPage";
import { TimelinePage } from "./TimelinePage";
import { can } from "../../shared/lib/permissions";
import { protectedOperations } from "../../shared/security/protectedOperations";

type Props = {
  user: CurrentUserDto;
  permissions: EffectivePermissionsDto | null;
  caseItem: CaseDto;
  onCaseUpdated: (caseItem: CaseDto) => void;
  onBackToCases: () => void;
  onLogout: () => void;
  onRefresh?: () => void;
  onOpenSettings?: () => void;
  onOpenBackup?: () => void;
  onOpenAuditLog?: () => void;
  onOpenUsers?: () => void;
};

export function CaseWorkspacePage({
  user,
  permissions,
  caseItem,
  onCaseUpdated,
  onBackToCases,
  onRefresh,
  onOpenSettings,
  onOpenBackup,
  onOpenAuditLog,
  onOpenUsers,
  onLogout,
}: Props) {
  const [activeSection, setActiveSection] =
    useState<CaseWorkspaceSection>("overview");

  const inspector = useCaseInspector();

  // Закрывать Inspector при смене раздела
  const handleSectionChange = useCallback(
    (section: CaseWorkspaceSection) => {
      inspector.close();
      setActiveSection(section);
    },
    [inspector]
  );

  // Сброс инспектора при смене дела
  useEffect(() => {
    inspector.close();
    setActiveSection("overview");
    // eslint-disable-next-line react-hooks/exhaustive-deps
  }, [caseItem.id]);

  const handleOpenFullCard = useCallback(
    (target: { type: string; id: string }) => {
      log.info(
        `Open full card: type=${target.type} id=${target.id}`
      );
      // В будущем здесь будет открытие соответствующей модалки
    },
    []
  );

  return (
    <AppShell>
      <TopBar
        caseCode={caseItem.caseCode}
        title={caseItem.title}
        displayName={user.displayName}
        onRefresh={onRefresh}
        onOpenSettings={onOpenSettings}
        onOpenBackup={onOpenBackup}
        onOpenAuditLog={onOpenAuditLog}
        onOpenUsers={onOpenUsers}
        onLogout={onLogout}
      />

      <div style={{ flex: 1, display: "flex", overflow: "hidden" }}>
        <CaseSidebar
          activeSection={activeSection}
          onSectionChange={handleSectionChange}
          onBackToCases={onBackToCases}
          caseId={caseItem.id}
        />

        <section
          style={{
            flex: 1,
            overflow: "auto",
            padding: "var(--space-6)",
          }}
        >
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
              canEdit={
                can(permissions, protectedOperations.relationUpdate) ||
                can(permissions, protectedOperations.relationCreate)
              }
            />
          )}

          {activeSection === "graph" && (
            <GraphPage caseId={caseItem.id} />
          )}

          {activeSection === "timeline" && (
            <TimelinePage
              caseId={caseItem.id}
              readonly={
                !can(permissions, protectedOperations.timelineCreate) &&
                !can(permissions, protectedOperations.timelineUpdate)
              }
            />
          )}

          {activeSection === "report" && (
            <section>
              <p style={{ color: "var(--text-muted)" }}>
                Справка — будет реализована позже.
              </p>
            </section>
          )}
        </section>

        <CaseInspector
          target={inspector.target}
          revision={inspector.revision}
          caseId={caseItem.id}
          onClose={inspector.close}
          onOpenFullCard={handleOpenFullCard}
          onInvalidate={inspector.invalidate}
        />
      </div>
    </AppShell>
  );
}

const log = {
  debug: (msg: string) => console.debug(`[CaseWorkspacePage] ${msg}`),
  info: (msg: string) => console.info(`[CaseWorkspacePage] ${msg}`),
};
