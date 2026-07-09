import { useEffect, useState } from "react";
import type { CaseDto } from "../../features/cases/model/caseTypes";
import { getMaterials } from "../../features/materials/api/materialsApi";
import {
  getIntegrityStatusLabel,
  getMaterialTypeLabel,
} from "../../features/materials/lib/materialOptions";
import type { MaterialDto } from "../../features/materials/model/materialTypes";
import { AddMaterialModal } from "./AddMaterialModal";
import { MaterialCardModal } from "./MaterialCardModal";

type Props = {
  caseItem: CaseDto;
};

export function MaterialsPage({ caseItem }: Props) {
  const [materials, setMaterials] = useState<MaterialDto[]>([]);
  const [loading, setLoading] = useState(true);
  const [error, setError] = useState<string | null>(null);
  const [addModalOpen, setAddModalOpen] = useState(false);
  const [selectedMaterial, setSelectedMaterial] = useState<MaterialDto | null>(
    null
  );

  async function loadMaterials() {
    try {
      setLoading(true);
      setError(null);

      const response = await getMaterials(caseItem.id);
      setMaterials(response);
    } catch (err) {
      console.error(err);
      setError("Не удалось загрузить материалы.");
    } finally {
      setLoading(false);
    }
  }

  useEffect(() => {
    loadMaterials();
  }, [caseItem.id]);

  function handleMaterialCreated(material: MaterialDto) {
    setMaterials((current) => [material, ...current]);
    setAddModalOpen(false);
  }

  function handleMaterialDeleted(materialId: string) {
    setMaterials((current) =>
      current.filter((material) => material.id !== materialId)
    );

    setSelectedMaterial(null);
  }

  function handleMaterialUpdated(updatedMaterial: MaterialDto) {
    setMaterials((current) =>
      current.map((material) =>
        material.id === updatedMaterial.id ? updatedMaterial : material
      )
    );

    setSelectedMaterial(updatedMaterial);
  }

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
          Материалы
        </h2>

        <button type="button" onClick={() => setAddModalOpen(true)}>
          Добавить материал
        </button>
      </div>

      {loading && (
        <p style={{ color: "var(--text-muted)" }}>Загрузка материалов...</p>
      )}

      {error && <p style={{ color: "var(--danger)" }}>{error}</p>}

      {!loading && !error && materials.length === 0 && (
        <section
          style={{
            marginTop: "var(--space-5)",
            padding: "var(--space-6)",
            border: "1px dashed var(--border-subtle)",
            background: "var(--bg-elevated)",
            borderRadius: "var(--radius-md)",
          }}
        >
          <h3 style={{ margin: "0 0 var(--space-2)", color: "var(--text-primary)" }}>
            Материалов пока нет
          </h3>
          <p style={{ margin: 0, color: "var(--text-muted)", fontSize: 13 }}>
            Добавьте первый материал. Файл будет скопирован во внутреннее хранилище,
            а SHA-256 рассчитан автоматически.
          </p>
        </section>
      )}

      {!loading && !error && materials.length > 0 && (
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
              <th style={{ padding: "var(--space-2) var(--space-3)" }}>Название</th>
              <th style={{ padding: "var(--space-2) var(--space-3)" }}>Тип</th>
              <th style={{ padding: "var(--space-2) var(--space-3)" }}>Источник</th>
              <th style={{ padding: "var(--space-2) var(--space-3)" }}>Дата фиксации</th>
              <th style={{ padding: "var(--space-2) var(--space-3)" }}>Файл</th>
              <th style={{ padding: "var(--space-2) var(--space-3)" }}>SHA-256</th>
              <th style={{ padding: "var(--space-2) var(--space-3)" }}>Целостность</th>
              <th style={{ padding: "var(--space-2) var(--space-3)" }}>В справку</th>
              <th style={{ padding: "var(--space-2) var(--space-3)" }}>Создано</th>
              <th style={{ padding: "var(--space-2) var(--space-3)" }}>Действия</th>
            </tr>
          </thead>

          <tbody>
            {materials.map((material) => (
              <tr
                key={material.id}
                style={{
                  borderBottom: "1px solid var(--border-subtle)",
                  fontSize: 13,
                }}
              >
                <td style={{ padding: "var(--space-3)" }}>{material.materialCode}</td>
                <td style={{ padding: "var(--space-3)" }}>{material.title}</td>
                <td style={{ padding: "var(--space-3)" }}>
                  {getMaterialTypeLabel(material.materialType)}
                </td>
                <td style={{ padding: "var(--space-3)" }}>
                  {material.sourceName || "Не указано"}
                </td>
                <td style={{ padding: "var(--space-3)" }}>
                  {material.capturedAt || "Не указано"}
                </td>
                <td style={{ padding: "var(--space-3)" }}>
                  {material.originalFileName || "—"}
                </td>
                <td style={{ padding: "var(--space-3)" }}>
                  {material.sha256 ? (
                    <code title={material.sha256}>
                      {material.sha256.slice(0, 16)}...
                    </code>
                  ) : (
                    "—"
                  )}
                </td>
                <td style={{ padding: "var(--space-3)" }}>
                  {getIntegrityStatusLabel(material.integrityStatus)}
                </td>
                <td style={{ padding: "var(--space-3)" }}>
                  {material.includeInReport ? "Да" : "Нет"}
                </td>
                <td style={{ padding: "var(--space-3)" }}>{material.createdAt}</td>
                <td style={{ padding: "var(--space-3)" }}>
                  <button
                    type="button"
                    onClick={() => setSelectedMaterial(material)}
                  >
                    Открыть
                  </button>
                </td>
              </tr>
            ))}
          </tbody>
        </table>
      )}

      {addModalOpen && (
        <AddMaterialModal
          caseId={caseItem.id}
          onCreated={handleMaterialCreated}
          onClose={() => setAddModalOpen(false)}
        />
      )}

      {selectedMaterial && (
        <MaterialCardModal
          material={selectedMaterial}
          onUpdated={handleMaterialUpdated}
          onDeleted={handleMaterialDeleted}
          onClose={() => setSelectedMaterial(null)}
        />
      )}
    </section>
  );
}