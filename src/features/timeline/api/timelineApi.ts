import { invokeCommand } from "../../../shared/api/invoke";
import type {
  CreateEventPayload,
  CreateEventResponse,
  GetEventByIdPayload,
  GetEventByIdResponse,
  GetTimelinePayload,
  GetTimelineResponse,
  UpdateEventPayload,
  UpdateEventResponse,
} from "../model/timelineTypes";

export function getTimeline(payload: GetTimelinePayload) {
  return invokeCommand<GetTimelineResponse>("get_timeline", { payload });
}

export function createEvent(payload: CreateEventPayload) {
  return invokeCommand<CreateEventResponse>("create_event", { payload });
}

export function getEventById(payload: GetEventByIdPayload) {
  return invokeCommand<GetEventByIdResponse>("get_event_by_id", { payload });
}

export function updateEvent(payload: UpdateEventPayload) {
  return invokeCommand<UpdateEventResponse>("update_event", { payload });
}
