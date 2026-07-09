import { useEffect, useState } from "react";

import { getMaterials } from "../../features/materials/api/materialsApi";
import type { MaterialDto } from "../../features/materials/model/materialTypes";
import { getObjects } from "../../features/objects/api/objectsApi";
import type { ObjectListItemDto } from "../../features/objects/model/objectTypes";
import { getObjectTypeLabel } from "../../features/objects/model/objectOptions";
import { getRelations } from "../../features/relations/api/relationsApi";
import {
  getRelationConfidenceLabel,
  getRelationTypeLabel,
} from "../../features/relations/model/relationConstants";
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
      <div
        style={{
          display: "flex",
          justifyContent: "space-between",
          alignItems: "center",
          marginBottom: "var(--space-4)",
        }}
      >
        <h2 style={{ margin: 0, color: "var(--text-primary)", fontSize: 18 }}>
          Связи
        </h2>

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

      {error && (
        <div
          style={{
            padding: "var(--space-2) var(--space-3)",
            border: "1px solid var(--danger)",
            borderRadius: "var(--radius-sm)",
            background: "color-mix(in srgb, var(--danger) 10%, transparent)",
            color: "var(--danger)",
            marginBottom: "var(--space-4)",
            fontSize: 13,
          }}
        >
          {error}
        </div>
      )}

      {isLoading ? (
        <p style={{ color: "var(--text-muted)" }}>Загрузка связей...</p>
      ) : relations.length === 0 ? (
        <div
          style={{
            marginTop: "var(--space-5)",
            padding: "var(--space-6)",
            border: "1px dashed var(--border-subtle)",
            borderRadius: "var(--radius-md)",
            color: "var(--text-muted)",
            fontSize: 13,
          }}
        >
          <h3 style={{ margin: "0 0 var(--space-2)", color: "var(--text-primary)" }}>
            Связей пока нет
          </h3>
          <p style={{ margin: 0 }}>Создайте минимум два объекта, затем добавьте связь между ними.</p>
        </div>
      ) : (
        <table
          style={{
            width: "100%",
            borderCollapse: "collapse",
            color: "var(--text-primary)",
          }}
        >
          <thead>
            <tr
              style={{
                borderBottom: "1px solid var(--border-subtle)",
                color: "var(--text-secondary)",
                fontSize: 13,
                textAlign: "left",
              }}
            >
              <th style={{ padding: "var(--space-2) var(--space-3)" }}>Код</th>
              <th style={{ padding: "var(--space-2) var(--space-3)" }}>Тип</th>
              <th style={{ padding: "var(--space-2) var(--space-3)" }}>Источник</th>
              <th style={{ padding: "var(--space-2) var(--space-3)" }}>Цель</th>
              <th style={{ padding: "var(--space-2) var(--space-3)" }}>Достоверность</th>
              <th style={{ padding: "var(--space-2) var(--space-3)" }}>Материал</th>
              <th style={{ padding: "var(--space-2) var(--space-3)" }}>В DOCX</th>
            </tr>
          </thead>
          <tbody>
            {relations.map((relation) => (
              <tr
                key={relation.id}
                onClick={() => setSelectedRelationId(relation.id)}
                style={{
                  cursor: "pointer",
                  borderBottom: "1px solid var(--border-subtle)",
                  fontSize: 13,
                }}
              >
                <td style={{ padding: "var(--space-3)" }}>
                  {relation.relationCode}
                </td>
                <td style={{ padding: "var(--space-3)" }}>
                  {getRelationTypeLabel(relation.relationType)}
                </td>
                <td style={{ padding: "var(--space-3)" }}>
                  {relation.sourceObject.objectCode} ·{" "}
                  {getObjectTypeLabel(relation.sourceObject.objectType)} ·{" "}
                  {relation.sourceObject.title}
                </td>
                <td style={{ padding: "var(--space-3)" }}>
                  {relation.targetObject.objectCode} ·{" "}
                  {getObjectTypeLabel(relation.targetObject.objectType)} ·{" "}
                  {relation.targetObject.title}
                </td>
                <td style={{ padding: "var(--space-3)" }}>
                  {getRelationConfidenceLabel(relation.confidenceLevel)}
                </td>
                <td style={{ padding: "var(--space-3)" }}>
                  {relation.supportingMaterial
                    ? `${relation.supportingMaterial.materialCode} · ${relation.supportingMaterial.title}`
                    : "—"}
                </td>
                <td style={{ padding: "var(--space-3)" }}>
                  {relation.includeInReport ? "Да" : "Нет"}
                </td>
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
          onDeleted={loadData}
        />
      )}
    </section>
  );
}
