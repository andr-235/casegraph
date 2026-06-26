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
