import { FormEvent, useState } from "react";

import type { MaterialDto } from "../../materials/model/materialTypes";
import type { ObjectListItemDto } from "../../objects/model/objectTypes";
import { getObjectTypeLabel } from "../../objects/model/objectOptions";
import { createRelation } from "../api/relationsApi";
import {
  relationConfidenceOptions,
  relationTypeOptions,
} from "../model/relationConstants";
import type { ConfidenceLevel, RelationType } from "../model/relationTypes";

type CreateRelationModalProps = {
  caseId: string;
  objects: ObjectListItemDto[];
  materials: MaterialDto[];
  onClose: () => void;
  onCreated: () => void;
};

export function CreateRelationModal({
  caseId,
  objects,
  materials,
  onClose,
  onCreated,
}: CreateRelationModalProps) {
  const [sourceObjectId, setSourceObjectId] = useState("");
  const [targetObjectId, setTargetObjectId] = useState("");
  const [relationType, setRelationType] = useState<RelationType>("related_to");
  const [title, setTitle] = useState("");
  const [basis, setBasis] = useState("");
  const [confidenceLevel, setConfidenceLevel] = useState<ConfidenceLevel>("medium");
  const [supportingMaterialId, setSupportingMaterialId] = useState("");
  const [analystComment, setAnalystComment] = useState("");
  const [includeInReport, setIncludeInReport] = useState(true);
  const [error, setError] = useState<string | null>(null);
  const [isSaving, setIsSaving] = useState(false);

  async function handleSubmit(event: FormEvent) {
    event.preventDefault();

    if (!sourceObjectId) {
      setError("Выберите первый объект.");
      return;
    }

    if (!targetObjectId) {
      setError("Выберите второй объект.");
      return;
    }

    if (sourceObjectId === targetObjectId) {
      setError("Нельзя создать связь объекта с самим собой.");
      return;
    }

    if (!basis.trim()) {
      setError("Укажите основание связи.");
      return;
    }

    setIsSaving(true);
    setError(null);

    try {
      await createRelation({
        caseId,
        sourceObjectId,
        targetObjectId,
        relationType,
        title: title.trim() || undefined,
        basis: basis.trim(),
        confidenceLevel,
        supportingMaterialId: supportingMaterialId || undefined,
        analystComment: analystComment.trim() || undefined,
        includeInReport,
      });

      onCreated();
      onClose();
    } catch (err) {
      setError(err instanceof Error ? err.message : "Не удалось создать связь.");
    } finally {
      setIsSaving(false);
    }
  }

  return (
    <div className="modal-backdrop">
      <div className="modal">
        <div className="modal__header">
          <h2>Создать связь</h2>
          <button type="button" onClick={onClose}>
            ×
          </button>
        </div>

        <form onSubmit={handleSubmit} className="form">
          {error && <div className="error-box">{error}</div>}

          <label>
            Первый объект
            <select
              value={sourceObjectId}
              onChange={(event) => setSourceObjectId(event.target.value)}
            >
              <option value="">Выберите объект</option>
              {objects.map((object) => (
                <option key={object.id} value={object.id}>
                  {object.objectCode} · {getObjectTypeLabel(object.objectType)} · {object.title}
                </option>
              ))}
            </select>
          </label>

          <label>
            Второй объект
            <select
              value={targetObjectId}
              onChange={(event) => setTargetObjectId(event.target.value)}
            >
              <option value="">Выберите объект</option>
              {objects.map((object) => (
                <option key={object.id} value={object.id}>
                  {object.objectCode} · {getObjectTypeLabel(object.objectType)} · {object.title}
                </option>
              ))}
            </select>
          </label>

          <label>
            Тип связи
            <select
              value={relationType}
              onChange={(event) => setRelationType(event.target.value as RelationType)}
            >
              {relationTypeOptions.map((option) => (
                <option key={option.value} value={option.value}>
                  {option.label}
                </option>
              ))}
            </select>
          </label>

          <label>
            Краткое название
            <input value={title} onChange={(event) => setTitle(event.target.value)} />
          </label>

          <label>
            Основание связи
            <textarea
              value={basis}
              onChange={(event) => setBasis(event.target.value)}
              rows={4}
            />
          </label>

          <label>
            Достоверность
            <select
              value={confidenceLevel}
              onChange={(event) =>
                setConfidenceLevel(event.target.value as ConfidenceLevel)
              }
            >
              {relationConfidenceOptions.map((option) => (
                <option key={option.value} value={option.value}>
                  {option.label}
                </option>
              ))}
            </select>
          </label>

          <label>
            Подтверждающий материал
            <select
              value={supportingMaterialId}
              onChange={(event) => setSupportingMaterialId(event.target.value)}
            >
              <option value="">Без материала</option>
              {materials.map((material) => (
                <option key={material.id} value={material.id}>
                  {material.materialCode} · {material.title}
                </option>
              ))}
            </select>
          </label>

          <label>
            Комментарий аналитика
            <textarea
              value={analystComment}
              onChange={(event) => setAnalystComment(event.target.value)}
              rows={3}
            />
          </label>

          <label>
            <input
              type="checkbox"
              checked={includeInReport}
              onChange={(event) => setIncludeInReport(event.target.checked)}
            />
            Включить в DOCX
          </label>

          <div className="modal__footer">
            <button type="button" onClick={onClose}>
              Отмена
            </button>
            <button type="submit" disabled={isSaving}>
              {isSaving ? "Сохранение..." : "Создать связь"}
            </button>
          </div>
        </form>
      </div>
    </div>
  );
}
