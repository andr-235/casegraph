export type RelationType =
  | "related_to"
  | "uses"
  | "belongs_to"
  | "mentioned_in"
  | "appears_with"
  | "confirmed_by_material"
  | "linked_to_phone"
  | "linked_to_account"
  | "linked_to_document"
  | "linked_to_vehicle"
  | "linked_to_address"
  | "linked_to_organization"
  | "other";

export type ConfidenceLevel = "high" | "medium" | "low" | "requires_check";

export type RelationObjectDto = {
  id: string;
  objectCode: string;
  objectType: string;
  title: string;
  value?: string | null;
  isKey: boolean;
};

export type RelationMaterialDto = {
  id: string;
  materialCode: string;
  title: string;
  materialType: string;
  integrityStatus: string;
};

export type RelationListItemDto = {
  id: string;
  caseId: string;
  relationCode: string;
  relationType: RelationType;
  title?: string | null;
  basis: string;
  confidenceLevel: ConfidenceLevel;
  sourceObject: RelationObjectDto;
  targetObject: RelationObjectDto;
  supportingMaterial?: RelationMaterialDto | null;
  includeInReport: boolean;
  createdAt: string;
  updatedAt: string;
};

export type CreateRelationPayload = {
  caseId: string;
  sourceObjectId: string;
  targetObjectId: string;
  relationType: RelationType;
  title?: string;
  basis: string;
  confidenceLevel: ConfidenceLevel;
  supportingMaterialId?: string;
  analystComment?: string;
  includeInReport?: boolean;
};

export type CreateRelationResponse = {
  relationItem: RelationListItemDto;
};

export type GetRelationsResponse = {
  items: RelationListItemDto[];
};

export type RelationDetailsDto = {
  id: string;
  caseId: string;
  relationCode: string;
  sourceObject: RelationObjectDto;
  targetObject: RelationObjectDto;
  relationType: RelationType;
  title?: string | null;
  basis: string;
  confidenceLevel: ConfidenceLevel;
  supportingMaterial?: RelationMaterialDto | null;
  analystComment?: string | null;
  includeInReport: boolean;
  createdAt: string;
  updatedAt: string;
};

export type GetRelationByIdResponse = {
  relation: RelationDetailsDto;
};

export type UpdateRelationPayload = {
  caseId: string;
  relationId: string;
  relationType: RelationType;
  title?: string;
  basis: string;
  confidenceLevel: ConfidenceLevel;
  supportingMaterialId?: string;
  analystComment?: string;
  includeInReport: boolean;
};

export type UpdateRelationResponse = {
  relation: RelationDetailsDto;
};
