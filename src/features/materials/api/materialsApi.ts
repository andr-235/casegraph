import { invokeCommand } from "../../../shared/api/invoke";
import type {
  CreateMaterialPayload,
  CreateMaterialResponse,
  DeleteMaterialPayload,
  DeleteMaterialResponse,
  MaterialDto,
  UpdateMaterialPayload,
  UpdateMaterialResponse,
} from "../model/materialTypes";

export function getMaterials(caseId: string): Promise<MaterialDto[]> {
  return invokeCommand<MaterialDto[]>("get_materials", {
    payload: {
      caseId,
    },
  });
}

export function createMaterial(
  payload: CreateMaterialPayload
): Promise<CreateMaterialResponse> {
  return invokeCommand<CreateMaterialResponse>("create_material", {
    payload,
  });
}

export function updateMaterial(
  payload: UpdateMaterialPayload
): Promise<UpdateMaterialResponse> {
  return invokeCommand<UpdateMaterialResponse>("update_material", {
    payload,
  });
}

export function deleteMaterial(
  payload: DeleteMaterialPayload
): Promise<DeleteMaterialResponse> {
  return invokeCommand<DeleteMaterialResponse>("delete_material", {
    payload,
  });
}