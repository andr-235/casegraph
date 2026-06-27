import type { TimelineEventDto } from "../model/timelineTypes";

export function replaceTimelineEvent(
  events: TimelineEventDto[],
  updatedEvent: TimelineEventDto,
): TimelineEventDto[] {
  return events.map((eventItem) =>
    eventItem.id === updatedEvent.id ? updatedEvent : eventItem,
  );
}

export function setTimelineEventIncludeInReport(
  events: TimelineEventDto[],
  eventId: string,
  includeInReport: boolean,
): TimelineEventDto[] {
  return events.map((eventItem) =>
    eventItem.id === eventId
      ? {
          ...eventItem,
          includeInReport,
        }
      : eventItem,
  );
}
