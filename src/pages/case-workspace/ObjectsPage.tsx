import { useEffect, useState } from "react";

import { getObjects } from "../../features/objects/api/objectsApi";
import { getObjectTypeLabel } from "../../features/objects/model/objectOptions";
import type { ObjectListItemDto } from "../../features/objects/model/objectTypes";
import { ObjectCardModal } from "../../features/objects/ui/ObjectCardModal";
import { CreateObjectModal } from "../../features/objects/ui/CreateObjectModal";
import type { CaseDto } from "../../features/cases/model/caseTypes";

type ObjectsPageProps = {
  caseItem: CaseDto;
};

export function ObjectsPage({ caseItem }: ObjectsPageProps) {
  const [items, setItems] = useState<ObjectListItemDto[]>([]);
  const [isLoading, setIsLoading] = useState(true);
  const [error, setError] = useState("");
  const [isCreateOpen, setIsCreateOpen] = useState(false);
  const [selectedObjectId, setSelectedObjectId] = useState<string | null>(null);

  async function loadObjects() {
    setIsLoading(true);
    setError("");

    try {
      const response = await getObjects(caseItem.id);
      setItems(response.items);
    } catch (unknownError) {
      setError(
        unknownError instanceof Error
          ? unknownError.message
          : "Не удалось загрузить объекты.",
      );
    } finally {
      setIsLoading(false);
    }
  }

  useEffect(() => {
    void loadObjects();
  }, [caseItem.id]);

  function handleCreated(objectItem: ObjectListItemDto) {
    setIsCreateOpen(false);
    setItems((currentItems) => [objectItem, ...currentItems]);
  }

  function handleUpdated(updatedObject: ObjectListItemDto) {
    setItems((currentItems) =>
      currentItems.map((item) =>
        item.id === updatedObject.id ? updatedObject : item,
      ),
    );
  }

  return (
    <section>
      <header className="page-header">
        <div>
          <h1>Объекты</h1>
          <p>{caseItem.caseCode} · {caseItem.title}</p>
        </div>

        <button type="button" onClick={() => setIsCreateOpen(true)}>
          Создать объект
        </button>
      </header>

      {error && <div className="error-message">{error}</div>}

      {isLoading ? (
        <p>Загрузка объектов...</p>
      ) : items.length === 0 ? (
        <div className="empty-state">
          <h2>Объекты ещё не добавлены</h2>
          <p>Создайте первый объект анализа для текущего дела.</p>
        </div>
      ) : (
        <table>
          <thead>
            <tr>
              <th>Код</th>
              <th>Тип</th>
              <th>Название</th>
              <th>Значение</th>
              <th>Ключевой</th>
              <th>Материалы</th>
              <th>Связи</th>
              <th>DOCX</th>
              <th>Обновлено</th>
            </tr>
          </thead>
          <tbody>
            {items.map((objectItem) => (
              <tr
                key={objectItem.id}
                onClick={() => setSelectedObjectId(objectItem.id)}
                style={{ cursor: "pointer" }}
              >
                <td>{objectItem.objectCode}</td>
                <td>{getObjectTypeLabel(objectItem.objectType)}</td>
                <td>{objectItem.title}</td>
                <td>{objectItem.value || "—"}</td>
                <td>{objectItem.isKey ? "Да" : "Нет"}</td>
                <td>{objectItem.linkedMaterialCount}</td>
                <td>{objectItem.relationCount}</td>
                <td>{objectItem.includeInReport ? "Да" : "Нет"}</td>
                <td>{objectItem.updatedAt}</td>
              </tr>
            ))}
          </tbody>
        </table>
      )}

      {isCreateOpen && (
        <CreateObjectModal
          caseId={caseItem.id}
          onClose={() => setIsCreateOpen(false)}
          onCreated={handleCreated}
        />
      )}

      {selectedObjectId && (
        <ObjectCardModal
          caseId={caseItem.id}
          objectId={selectedObjectId}
          onClose={() => setSelectedObjectId(null)}
          onUpdated={handleUpdated}
        />
      )}
    </section>
  );
}
