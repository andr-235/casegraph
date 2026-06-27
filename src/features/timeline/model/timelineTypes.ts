export type EventType =
  | "fact"
  | "action"
  | "observation"
  | "document_fixation"
  | "relation_established"
  | "material_received"
  | "other";

export type DatePrecision =
  | "day"
  | "month"
  | "year"
  | "approximate"
  | "period";

export type TimelineEventDto = {
  id: string;
  caseId: string;
  eventCode: string;
  eventType: EventType;
  title: string;
  description: string;
  eventDate: string;
  eventTime?: string;
  datePrecision: DatePrecision;
  periodStart?: string;
  periodEnd?: string;
  sourceNote: string;
  analystComment: string;
  includeInReport: boolean;
  linkedObjectCount: number;
  linkedMaterialCount: number;
  createdByUserId: string;
  createdAt: string;
  updatedAt: string;
};

export type GetTimelinePayload = {
  caseId: string;
};

export type GetTimelineResponse = {
  items: TimelineEventDto[];
};

export type CreateEventPayload = {
  caseId: string;
  eventType: EventType;
  title: string;
  description: string;
  eventDate: string;
  eventTime?: string;
  datePrecision: DatePrecision;
  periodStart?: string;
  periodEnd?: string;
  sourceNote: string;
  analystComment: string;
  includeInReport: boolean;
  objectIds: string[];
  materialIds: string[];
  linkNote: string;
};

export type CreateEventResponse = {
  eventItem: TimelineEventDto;
};
