export type MaterialType =
  | "image"
  | "pdf"
  | "document"
  | "spreadsheet"
  | "text"
  | "html"
  | "other";

export type IntegrityStatus =
  | "not_checked"
  | "ok"
  | "mismatch"
  | "missing"
  | "read_error";

export type MaterialDto = {
  id: string;
  caseId: string;
  materialCode: string;
  title: string;
  materialType: MaterialType;
  sourceName: string;
  description: string;
  capturedAt: string | null;
  includeInReport: boolean;

  originalFileName: string | null;
  originalPath: string | null;
  storedFilePath: string | null;
  fileSize: number | null;
  mimeType: string | null;
  sha256: string | null;
  integrityStatus: IntegrityStatus;

  createdByUserId: string;
  createdAt: string;
  updatedAt: string;
};

export type GetMaterialsPayload = {
  caseId: string;
};

export type CreateMaterialPayload = {
  caseId: string;
  title: string;
  materialType: MaterialType;
  sourceName?: string;
  description?: string;
  capturedAt?: string | null;
  includeInReport: boolean;
};

export type CreateMaterialResponse = {
  material: MaterialDto;
};