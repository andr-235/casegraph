import type { CreateEventPayload } from "./timelineTypes";

export function createEmptyEventPayload(caseId: string): CreateEventPayload {
  return {
    caseId,
    eventType: "fact",
    title: "",
    description: "",
    eventDate: "",
    eventTime: undefined,
    datePrecision: "day",
    periodStart: undefined,
    periodEnd: undefined,
    sourceNote: "",
    analystComment: "",
    includeInReport: true,
    objectIds: [],
    materialIds: [],
    linkNote: "",
  };
}
