import type {
  EventDetailsDto,
  UpdateEventPayload,
} from "./timelineTypes";

export function createUpdateEventPayloadFromDetails(
  caseId: string,
  eventId: string,
  details: EventDetailsDto,
): UpdateEventPayload {
  return {
    caseId,
    eventId,
    eventType: details.eventItem.eventType,
    title: details.eventItem.title,
    description: details.eventItem.description,
    eventDate: details.eventItem.eventDate,
    eventTime: details.eventItem.eventTime ?? undefined,
    datePrecision: details.eventItem.datePrecision,
    periodStart: details.eventItem.periodStart ?? undefined,
    periodEnd: details.eventItem.periodEnd ?? undefined,
    sourceNote: details.eventItem.sourceNote,
    analystComment: details.eventItem.analystComment,
    includeInReport: details.eventItem.includeInReport,
    objectIds: details.linkedObjects.map((item) => item.objectId),
    materialIds: details.linkedMaterials.map((item) => item.materialId),
    linkNote: getInitialEventLinkNote(details),
  };
}

export function sanitizeUpdateEventPayload(
  form: UpdateEventPayload,
): UpdateEventPayload {
  return {
    ...form,
    title: form.title.trim(),
    description: form.description.trim(),
    sourceNote: form.sourceNote.trim(),
    analystComment: form.analystComment.trim(),
    linkNote: form.linkNote.trim(),
    eventDate: form.eventDate.trim(),
    eventTime: form.eventTime?.trim() || undefined,
    periodStart: form.periodStart?.trim() || undefined,
    periodEnd: form.periodEnd?.trim() || undefined,
  };
}

function getInitialEventLinkNote(details: EventDetailsDto): string {
  return (
    details.linkedObjects.find((item) => item.linkNote.trim().length > 0)
      ?.linkNote ??
    details.linkedMaterials.find((item) => item.linkNote.trim().length > 0)
      ?.linkNote ??
    ""
  );
}
