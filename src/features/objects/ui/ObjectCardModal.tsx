import { FormEvent, useEffect, useState } from "react";

import { getMaterials } from "../../materials/api/materialsApi";
import type { MaterialDto } from "../../materials/model/materialTypes";
import { getObjectById, linkObjectToMaterials, softDeleteObject, updateObject } from "../api/objectsApi";
import { getObjectTypeLabel } from "../model/objectOptions";
import type { ObjectDetailsDto, ObjectListItemDto } from "../model/objectTypes";

type ObjectCardModalProps = {
  caseId: string;
  objectId: string;
  onClose: () => void;
  onUpdated: (objectItem: ObjectListItemDto) => void;
  onDeleted: (objectId: string) => void;
};

function toListItem(objectItem: ObjectDetailsDto): ObjectListItemDto {
  return {
    id: objectItem.id,
    caseId: objectItem.caseId,
    objectCode: objectItem.objectCode,
    objectType: objectItem.objectType,
    title: objectItem.title,
    value: objectItem.value,
    description: objectItem.description,
    isKey: objectItem.isKey,
    includeInReport: objectItem.includeInReport,
    linkedMaterialCount: objectItem.linkedMaterialCount,
    relationCount: objectItem.relationCount,
    createdAt: objectItem.createdAt,
    updatedAt: objectItem.updatedAt,
  };
}

export function ObjectCardModal({
  caseId,
  objectId,
  onClose,
  onUpdated,
  onDeleted,
}: ObjectCardModalProps) {
  const [objectItem, setObjectItem] = useState<ObjectDetailsDto | null>(null);
  const [title, setTitle] = useState("");
  const [value, setValue] = useState("");
  const [description, setDescription] = useState("");
  const [confidenceNote, setConfidenceNote] = useState("");
  const [isKey, setIsKey] = useState(false);
  const [includeInReport, setIncludeInReport] = useState(true);
  const [isLoading, setIsLoading] = useState(true);
  const [isSaving, setIsSaving] = useState(false);
  const [isDeleting, setIsDeleting] = useState(false);
  const [error, setError] = useState("");
  const [availableMaterials, setAvailableMaterials] = useState<MaterialDto[]>([]);
  const [selectedMaterialIds, setSelectedMaterialIds] = useState<string[]>([]);
  const [linkReason, setLinkReason] = useState("");
  const [isSavingLinks, setIsSavingLinks] = useState(false);

  async function loadAvailableMaterials() {
    try {
      const response = await getMaterials(caseId);
      setAvailableMaterials(response);
    } catch (unknownError) {
      setError(
        unknownError instanceof Error
          ? unknownError.message
          : "Не удалось загрузить материалы дела.",
      );
    }
  }

  async function loadObject() {
    setIsLoading(true);
    setError("");

    try {
      const response = await getObjectById(caseId, objectId);
      const loadedObject = response.objectItem;

      setObjectItem(loadedObject);
      setTitle(loadedObject.title);
      setValue(loadedObject.value ?? "");
      setDescription(loadedObject.description ?? "");
      setConfidenceNote(loadedObject.confidenceNote ?? "");
      setIsKey(loadedObject.isKey);
      setIncludeInReport(loadedObject.includeInReport);
      setSelectedMaterialIds(
        loadedObject.linkedMaterials.map((material) => material.id),
      );
      const firstReason = loadedObject.linkedMaterials.find(
        (material) => material.linkReason,
      )?.linkReason;
      setLinkReason(firstReason ?? "");
    } catch (unknownError) {
      setError(
        unknownError instanceof Error
          ? unknownError.message
          : "Не удалось загрузить объект.",
      );
    } finally {
      setIsLoading(false);
    }
  }

  useEffect(() => {
    void loadObject();
    void loadAvailableMaterials();
  }, [caseId, objectId]);

  function toggleMaterial(materialId: string) {
    setSelectedMaterialIds((currentIds) => {
      if (currentIds.includes(materialId)) {
        return currentIds.filter((id) => id !== materialId);
      }

      return [...currentIds, materialId];
    });
  }

  async function handleSaveMaterialLinks() {
    if (!objectItem) {
      return;
    }

    setIsSavingLinks(true);
    setError("");

    try {
      const response = await linkObjectToMaterials({
        caseId,
        objectId,
        materialIds: selectedMaterialIds,
        linkReason: linkReason.trim() || undefined,
      });

      setObjectItem(response.objectItem);
      onUpdated(toListItem(response.objectItem));
    } catch (unknownError) {
      setError(
        unknownError instanceof Error
          ? unknownError.message
          : "Не удалось сохранить связи объекта с материалами.",
      );
    } finally {
      setIsSavingLinks(false);
    }
  }

  async function handleSubmit(event: FormEvent) {
    event.preventDefault();

    if (!objectItem) {
      return;
    }

    if (!title.trim()) {
      setError("Название объекта обязательно.");
      return;
    }

    const normalizedTitle = title.trim();

    setIsSaving(true);
    setError("");

    try {
      const response = await updateObject({
        caseId,
        objectId,
        title: normalizedTitle,
        value: value.trim() || undefined,
        description: description.trim() || undefined,
        confidenceNote: confidenceNote.trim() || undefined,
        isKey,
        includeInReport,
      });

      setObjectItem(response.objectItem);
      onUpdated(toListItem(response.objectItem));
    } catch (unknownError) {
      setError(
        unknownError instanceof Error
          ? unknownError.message
          : "Не удалось сохранить объект.",
      );
    } finally {
      setIsSaving(false);
    }
  }

  async function handleDeleteObject() {
    const confirmed = window.confirm(
      "Удалить объект? Он будет скрыт из списка, но запись останется в базе как архивная.",
    );

    if (!confirmed) {
      return;
    }

    setIsDeleting(true);
    setError("");

    try {
      const response = await softDeleteObject({
        caseId,
        objectId,
      });

      onDeleted(response.objectId);
      onClose();
    } catch (unknownError) {
      setError(
        unknownError instanceof Error
          ? unknownError.message
          : "Не удалось удалить объект.",
      );
    } finally {
      setIsDeleting(false);
    }
  }

  return (
    <div className="modal-backdrop">
      <div className="modal modal-wide">
        <header className="modal-header">
          <div>
            <h2>Карточка объекта</h2>
            {objectItem && (
              <p>
                {objectItem.objectCode} · {getObjectTypeLabel(objectItem.objectType)}
              </p>
            )}
          </div>

          <button type="button" onClick={onClose}>
            Закрыть
          </button>
        </header>

        {error && <div className="error-message">{error}</div>}

        {isLoading ? (
          <p>Загрузка объекта...</p>
        ) : !objectItem ? (
          <p>Объект не найден.</p>
        ) : (
          <form onSubmit={handleSubmit}>
            <section>
              <h3>Основные сведения</h3>

              <div className="form-grid">
                <label>
                  Код
                  <input value={objectItem.objectCode} disabled />
                </label>

                <label>
                  Тип
                  <input value={getObjectTypeLabel(objectItem.objectType)} disabled />
                </label>

                <label>
                  Название
                  <input
                    value={title}
                    onChange={(event) => setTitle(event.target.value)}
                  />
                </label>

                <label>
                  Значение
                  <input
                    value={value}
                    onChange={(event) => setValue(event.target.value)}
                  />
                </label>
              </div>

              <label>
                Описание
                <textarea
                  value={description}
                  onChange={(event) => setDescription(event.target.value)}
                />
              </label>

              <label>
                Примечание к достоверности
                <textarea
                  value={confidenceNote}
                  onChange={(event) => setConfidenceNote(event.target.value)}
                />
              </label>

              <label>
                <input
                  type="checkbox"
                  checked={isKey}
                  onChange={(event) => setIsKey(event.target.checked)}
                />
                Ключевой объект
              </label>

              <label>
                <input
                  type="checkbox"
                  checked={includeInReport}
                  onChange={(event) => setIncludeInReport(event.target.checked)}
                />
                Включить в DOCX
              </label>
            </section>

            <section>
              <h3>Связанные материалы</h3>

              {availableMaterials.length === 0 ? (
                <p>В деле пока нет материалов.</p>
              ) : (
                <>
                  <label>
                    Общее основание связи
                    <textarea
                      value={linkReason}
                      onChange={(event) => setLinkReason(event.target.value)}
                      placeholder="Например: объект указан в тексте материала или подтверждается вложением"
                    />
                  </label>

                  <div className="checkbox-list">
                    {availableMaterials.map((material) => (
                      <label key={material.id}>
                        <input
                          type="checkbox"
                          checked={selectedMaterialIds.includes(material.id)}
                          onChange={() => toggleMaterial(material.id)}
                        />
                        {material.materialCode} · {material.title} · {material.materialType}
                      </label>
                    ))}
                  </div>

                  <button
                    type="button"
                    onClick={handleSaveMaterialLinks}
                    disabled={isSavingLinks}
                  >
                    {isSavingLinks ? "Сохранение связей..." : "Сохранить связи с материалами"}
                  </button>
                </>
              )}

              <div>
                <h4>Уже связанные материалы</h4>

                {objectItem.linkedMaterials.length === 0 ? (
                  <p>Материалы пока не связаны с объектом.</p>
                ) : (
                  <table>
                    <thead>
                      <tr>
                        <th>Код</th>
                        <th>Название</th>
                        <th>Тип</th>
                        <th>SHA</th>
                        <th>Основание</th>
                      </tr>
                    </thead>
                    <tbody>
                      {objectItem.linkedMaterials.map((material) => (
                        <tr key={material.id}>
                          <td>{material.materialCode}</td>
                          <td>{material.title}</td>
                          <td>{material.materialType}</td>
                          <td>{material.hashStatus}</td>
                          <td>{material.linkReason || "—"}</td>
                        </tr>
                      ))}
                    </tbody>
                  </table>
                )}
              </div>
            </section>

            <section>
              <h3>Связи объекта</h3>

              <p>Связи будут доступны после реализации модуля связей.</p>
            </section>

            <div className="modal-actions">
              <button type="button" onClick={onClose} disabled={isSaving}>
                Закрыть
              </button>
              <button type="submit" disabled={isSaving}>
                {isSaving ? "Сохранение..." : "Сохранить"}
              </button>
              <button
                type="button"
                onClick={handleDeleteObject}
                disabled={isDeleting || isSaving}
              >
                {isDeleting ? "Удаление..." : "Удалить объект"}
              </button>
            </div>
          </form>
        )}
      </div>
    </div>
  );
}
