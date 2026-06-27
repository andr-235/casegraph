import { invokeCommand } from "../../../shared/api/invoke";
import type {
  GetGraphDataPayload,
  GetGraphDataResponse,
} from "../model/graphTypes";

export function getGraphData(
  payload: GetGraphDataPayload,
): Promise<GetGraphDataResponse> {
  return invokeCommand<GetGraphDataResponse>("get_graph_data", { payload });
}
