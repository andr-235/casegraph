export type ObjectType =
  | "person"
  | "account"
  | "phone"
  | "address"
  | "vehicle"
  | "organization"
  | "document"
  | "image"
  | "publication"
  | "event"
  | "source"
  | "other";

export type ObjectListItemDto = {
  id: string;
  caseId: string;
  objectCode: string;
  objectType: ObjectType;
  title: string;
  value?: string | null;
  description: string;
  isKey: boolean;
  includeInReport: boolean;
  linkedMaterialCount: number;
  relationCount: number;
  createdAt: string;
  updatedAt: string;
};

export type CreateObjectPayload = {
  caseId: string;
  objectType: ObjectType;
  title: string;
  value?: string;
  description?: string;
  isKey?: boolean;
  confidenceNote?: string;
  includeInReport?: boolean;
};

export type CreateObjectResponse = {
  objectItem: ObjectListItemDto;
};

export type GetObjectsResponse = {
  items: ObjectListItemDto[];
};

export type LinkedObjectMaterialDto = {
  id: string;
  materialCode: string;
  title: string;
  materialType: string;
  hashStatus: string;
  linkReason: string;
};

export type ObjectRelationSummaryDto = {
  relationId: string;
  relationCode: string;
  relationType: string;
  counterpartObjectId: string;
  counterpartObjectCode: string;
  counterpartTitle: string;
  confidenceLevel: string;
};

export type ObjectDetailsDto = {
  id: string;
  caseId: string;
  objectCode: string;
  objectType: ObjectType;
  title: string;
  value?: string | null;
  description: string;
  isKey: boolean;
  confidenceNote: string;
  includeInReport: boolean;
  linkedMaterialCount: number;
  relationCount: number;
  createdAt: string;
  updatedAt: string;
  linkedMaterials: LinkedObjectMaterialDto[];
  relations: ObjectRelationSummaryDto[];
};

export type GetObjectByIdResponse = {
  objectItem: ObjectDetailsDto;
};

export type UpdateObjectPayload = {
  caseId: string;
  objectId: string;
  title: string;
  value?: string;
  description?: string;
  isKey: boolean;
  confidenceNote?: string;
  includeInReport: boolean;
};

export type UpdateObjectResponse = {
  objectItem: ObjectDetailsDto;
};
