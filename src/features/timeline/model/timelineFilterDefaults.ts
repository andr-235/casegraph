import type { GetTimelinePayload } from "./timelineTypes";

export type TimelineFiltersState = Omit<GetTimelinePayload, "caseId">;

export function createEmptyTimelineFilters(): TimelineFiltersState {
  return {
    query: "",
    eventType: "",
    objectId: "",
    materialId: "",
    dateFrom: "",
    dateTo: "",
    includeInReport: undefined,
  };
}

export function buildGetTimelinePayload(
  caseId: string,
  filters: TimelineFiltersState,
): GetTimelinePayload {
  return {
    caseId,
    query: filters.query?.trim() || undefined,
    eventType: filters.eventType || undefined,
    objectId: filters.objectId || undefined,
    materialId: filters.materialId || undefined,
    dateFrom: filters.dateFrom || undefined,
    dateTo: filters.dateTo || undefined,
    includeInReport: filters.includeInReport,
  };
}
