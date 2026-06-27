import { invokeCommand } from "../../../shared/api/invoke";
import type {
  CreateEventPayload,
  CreateEventResponse,
  GetTimelinePayload,
  GetTimelineResponse,
} from "../model/timelineTypes";

export function getTimeline(payload: GetTimelinePayload) {
  return invokeCommand<GetTimelineResponse>("get_timeline", { payload });
}

export function createEvent(payload: CreateEventPayload) {
  return invokeCommand<CreateEventResponse>("create_event", { payload });
}
