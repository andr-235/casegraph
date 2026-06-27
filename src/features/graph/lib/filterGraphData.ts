import type { GraphEdge, GraphNode } from "../model/graphTypes";

export type GraphFilters = {
  objectType: string;
  relationType: string;
  confidenceLevel: string;
  onlyKeyObjects: boolean;
};

export type FilteredGraphData = {
  nodes: GraphNode[];
  edges: GraphEdge[];
};

export const defaultGraphFilters: GraphFilters = {
  objectType: "all",
  relationType: "all",
  confidenceLevel: "all",
  onlyKeyObjects: false,
};

export function filterGraphData(
  nodes: GraphNode[],
  edges: GraphEdge[],
  filters: GraphFilters,
): FilteredGraphData {
  const filteredNodes = nodes.filter((node) => {
    if (filters.objectType !== "all" && node.objectType !== filters.objectType) {
      return false;
    }

    if (filters.onlyKeyObjects && !node.isKey) {
      return false;
    }

    return true;
  });

  const visibleNodeIds = new Set(filteredNodes.map((node) => node.id));

  const filteredEdges = edges.filter((edge) => {
    if (!visibleNodeIds.has(edge.sourceObjectId)) {
      return false;
    }

    if (!visibleNodeIds.has(edge.targetObjectId)) {
      return false;
    }

    if (
      filters.relationType !== "all" &&
      edge.relationType !== filters.relationType
    ) {
      return false;
    }

    if (
      filters.confidenceLevel !== "all" &&
      edge.confidenceLevel !== filters.confidenceLevel
    ) {
      return false;
    }

    return true;
  });

  return {
    nodes: filteredNodes,
    edges: filteredEdges,
  };
}
