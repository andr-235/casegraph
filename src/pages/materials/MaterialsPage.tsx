import { useEffect, useState } from "react";
import type { CaseDto } from "../../features/cases/model/caseTypes";
import { getMaterials } from "../../features/materials/api/materialsApi";
import type { MaterialDto } from "../../features/materials/model/materialTypes";
import { AddMaterialModal } from "./AddMaterialModal";

type Props = {
  caseItem: CaseDto;
};

function getMaterialTypeLabel(type: string) {
  const labels: Record<string, string> = {
    image: "Изображение",
    pdf: "PDF",
    document: "Документ",
    spreadsheet: "Таблица",
    text: "Текст",
    html: "HTML",
    other: "Другое",
  };

  return labels[type] ?? type;
}

function getIntegrityStatusLabel(status: string) {
  const labels: Record<string, string> = {
    not_checked: "Не проверено",
    ok: "OK",
    mismatch: "Несовпадение",
    missing: "Файл отсутствует",
    read_error: "Ошибка чтения",
  };

  return labels[status] ?? status;
}

export function MaterialsPage({ caseItem }: Props) {
  const [materials, setMaterials] = useState<MaterialDto[]>([]);
  const [loading, setLoading] = useState(true);
  const [error, setError] = useState<string | null>(null);
  const [addModalOpen, setAddModalOpen] = useState(false);

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

  return (
    <section>
      <header style={{ display: "flex", justifyContent: "space-between" }}>
        <div>
          <h2>Материалы</h2>
          <p>
            Дело: <strong>{caseItem.caseCode}</strong> · {caseItem.title}
          </p>
        </div>

        <button type="button" onClick={() => setAddModalOpen(true)}>
          Добавить материал
        </button>
      </header>

      <hr />

      {loading && <p>Загрузка материалов...</p>}

      {error && <p style={{ color: "crimson" }}>{error}</p>}

      {!loading && !error && materials.length === 0 && (
        <section
          style={{
            marginTop: 24,
            padding: 24,
            border: "1px dashed #aaa",
            background: "#fafafa",
          }}
        >
          <h3>Материалов пока нет</h3>
          <p>
            Добавьте первый материал. В этом срезе файл уже копируется во внутреннее
            хранилище, а SHA-256 рассчитывается автоматически.
          </p>
        </section>
      )}

      {!loading && !error && materials.length > 0 && (
        <table border={1} cellPadding={8} style={{ borderCollapse: "collapse" }}>
          <thead>
            <tr>
              <th>Код</th>
              <th>Название</th>
              <th>Тип</th>
              <th>Источник</th>
              <th>Дата фиксации</th>
              <th>Файл</th>
              <th>SHA-256</th>
              <th>Целостность</th>
              <th>В справку</th>
              <th>Создано</th>
            </tr>
          </thead>

          <tbody>
            {materials.map((material) => (
              <tr key={material.id}>
                <td>{material.materialCode}</td>
                <td>{material.title}</td>
                <td>{getMaterialTypeLabel(material.materialType)}</td>
                <td>{material.sourceName || "Не указано"}</td>
                <td>{material.capturedAt || "Не указано"}</td>
                <td>{material.originalFileName || "—"}</td>
                <td>
                  {material.sha256 ? (
                    <code title={material.sha256}>{material.sha256.slice(0, 16)}...</code>
                  ) : (
                    "—"
                  )}
                </td>
                <td>{getIntegrityStatusLabel(material.integrityStatus)}</td>
                <td>{material.includeInReport ? "Да" : "Нет"}</td>
                <td>{material.createdAt}</td>
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
    </section>
  );
}