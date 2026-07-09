/**
 * Типы для workspace-редизайна (тёмная тема, инспектор, сайдбар).
 */

import type { CaseDto } from "../../features/cases/model/caseTypes";

// ===================================================================
// CaseWorkspaceSection — разделы рабочего пространства (без settings)
// ===================================================================
export type CaseWorkspaceSection =
  | "overview"
  | "materials"
  | "objects"
  | "relations"
  | "graph"
  | "timeline"
  | "report";

// ===================================================================
// SidebarGroup — группы в боковой панели
// ===================================================================
export type SidebarGroup = "overview" | "data" | "analysis" | "result";

// ===================================================================
// CaseInspectorTarget — цель для правой панели Inspector
// ===================================================================
export type CaseInspectorTarget =
  | { type: "object"; id: string }
  | { type: "material"; id: string }
  | { type: "relation"; id: string }
  | { type: "event"; id: string };

// ===================================================================
// InspectorState — состояние инспектора (target + revision counter)
// ===================================================================
export interface InspectorState {
  target: CaseInspectorTarget | null;
  revision: number;
}

// ===================================================================
// CaseSummaryDto — сводка по делу (счётчики для сайдбара)
// ===================================================================
export interface CaseSummaryDto {
  objectCount: number;
  keyObjectCount: number;
  materialCount: number;
  integrityIssueCount: number;
  relationCount: number;
  eventCount: number;
  reportEventCount: number;
  updatedAt: string;
}

// ===================================================================
// ObjectPreviewDto — краткое представление объекта
// ===================================================================
export interface ObjectPreviewDto {
  id: string;
  objectCode: string;
  objectType: string;
  title: string;
  isKey: boolean;
}

// ===================================================================
// ActivityItemDto — запись в ленте последней активности
// ===================================================================
export interface ActivityItemDto {
  entityType: string;
  entityId: string;
  code: string;
  title: string;
  timestamp: string;
  action: string;
}

// ===================================================================
// CaseOverviewDto — карточка дела с метриками
// ===================================================================
export interface CaseOverviewDto {
  caseItem: CaseDto;
  summary: CaseSummaryDto;
  keyObjects: ObjectPreviewDto[];
  recentActivity: ActivityItemDto[];
}
