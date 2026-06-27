export type GraphNode = {
  id: string;
  caseId: string;
  objectCode: string;
  objectType: string;
  title: string;
  value: string | null;
  isKey: boolean;
  includeInReport: boolean;
};

export type GraphEdge = {
  id: string;
  caseId: string;
  relationCode: string;
  sourceObjectId: string;
  targetObjectId: string;
  relationType: string;
  title: string | null;
  basis: string;
  confidenceLevel: string;
  supportingMaterialId: string | null;
  includeInReport: boolean;
};

export type GetGraphDataPayload = {
  caseId: string;
};

export type GetGraphDataResponse = {
  nodes: GraphNode[];
  edges: GraphEdge[];
};
