import { useEffect, useState } from "react";

import { getMaterials } from "../../features/materials/api/materialsApi";
import type { MaterialDto } from "../../features/materials/model/materialTypes";
import { getObjects } from "../../features/objects/api/objectsApi";
import type { ObjectListItemDto } from "../../features/objects/model/objectTypes";
import { getObjectTypeLabel } from "../../features/objects/model/objectOptions";
import { getRelations } from "../../features/relations/api/relationsApi";
import {
  getConfidenceLevelLabel,
  getRelationTypeLabel,
} from "../../features/relations/model/relationOptions";
import type { RelationListItemDto } from "../../features/relations/model/relationTypes";
import { CreateRelationModal } from "../../features/relations/ui/CreateRelationModal";
import { RelationCardModal } from "../../features/relations/ui/RelationCardModal";

type RelationsPageProps = {
  caseId: string;
  canEdit: boolean;
};

export function RelationsPage({ caseId, canEdit }: RelationsPageProps) {
  const [relations, setRelations] = useState<RelationListItemDto[]>([]);
  const [objects, setObjects] = useState<ObjectListItemDto[]>([]);
  const [materials, setMaterials] = useState<MaterialDto[]>([]);
  const [isLoading, setIsLoading] = useState(true);
  const [isCreateOpen, setIsCreateOpen] = useState(false);
  const [selectedRelationId, setSelectedRelationId] = useState<string | null>(null);
  const [error, setError] = useState<string | null>(null);

  async function loadData() {
    setIsLoading(true);
    setError(null);

    try {
      const [relationsResponse, objectsResponse, materialsResponse] =
        await Promise.all([
          getRelations(caseId),
          getObjects(caseId),
          getMaterials(caseId),
        ]);

      setRelations(relationsResponse.items);
      setObjects(objectsResponse.items);
      setMaterials(materialsResponse);
    } catch (err) {
      setError(err instanceof Error ? err.message : "Не удалось загрузить связи.");
    } finally {
      setIsLoading(false);
    }
  }

  useEffect(() => {
    void loadData();
  }, [caseId]);

  return (
    <section>
      <div className="page-header">
        <div>
          <h1>Связи</h1>
          <p>Связи между объектами анализа внутри текущего дела.</p>
        </div>

        {canEdit && (
          <button
            type="button"
            onClick={() => setIsCreateOpen(true)}
            disabled={objects.length < 2}
          >
            Создать связь
          </button>
        )}
      </div>

      {error && <div className="error-box">{error}</div>}

      {isLoading ? (
        <p>Загрузка связей...</p>
      ) : relations.length === 0 ? (
        <div className="empty-state">
          <h2>Связей пока нет</h2>
          <p>Создайте минимум два объекта, затем добавьте связь между ними.</p>
        </div>
      ) : (
        <table className="data-table">
          <thead>
            <tr>
              <th>Код</th>
              <th>Тип</th>
              <th>Источник</th>
              <th>Цель</th>
              <th>Достоверность</th>
              <th>Материал</th>
              <th>В DOCX</th>
            </tr>
          </thead>
          <tbody>
            {relations.map((relation) => (
              <tr
                key={relation.id}
                onClick={() => setSelectedRelationId(relation.id)}
                style={{ cursor: "pointer" }}
              >
                <td>{relation.relationCode}</td>
                <td>{getRelationTypeLabel(relation.relationType)}</td>
                <td>
                  {relation.sourceObject.objectCode} ·{" "}
                  {getObjectTypeLabel(relation.sourceObject.objectType)} ·{" "}
                  {relation.sourceObject.title}
                </td>
                <td>
                  {relation.targetObject.objectCode} ·{" "}
                  {getObjectTypeLabel(relation.targetObject.objectType)} ·{" "}
                  {relation.targetObject.title}
                </td>
                <td>{getConfidenceLevelLabel(relation.confidenceLevel)}</td>
                <td>
                  {relation.supportingMaterial
                    ? `${relation.supportingMaterial.materialCode} · ${relation.supportingMaterial.title}`
                    : "—"}
                </td>
                <td>{relation.includeInReport ? "Да" : "Нет"}</td>
              </tr>
            ))}
          </tbody>
        </table>
      )}

      {isCreateOpen && (
        <CreateRelationModal
          caseId={caseId}
          objects={objects}
          materials={materials}
          onClose={() => setIsCreateOpen(false)}
          onCreated={loadData}
        />
      )}

      {selectedRelationId && (
        <RelationCardModal
          caseId={caseId}
          relationId={selectedRelationId}
          materials={materials}
          canEdit={canEdit}
          onClose={() => setSelectedRelationId(null)}
          onUpdated={loadData}
        />
      )}
    </section>
  );
}
