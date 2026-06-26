import { FormEvent, useEffect, useState } from "react";

import type { MaterialDto } from "../../materials/model/materialTypes";
import { getObjectTypeLabel } from "../../objects/model/objectOptions";
import { getRelationById, updateRelation } from "../api/relationsApi";
import {
  confidenceLevelOptions,
  relationTypeOptions,
} from "../model/relationOptions";
import type {
  ConfidenceLevel,
  RelationDetailsDto,
  RelationType,
} from "../model/relationTypes";

type RelationCardModalProps = {
  caseId: string;
  relationId: string;
  materials: MaterialDto[];
  canEdit: boolean;
  onClose: () => void;
  onUpdated: () => void;
};

export function RelationCardModal({
  caseId,
  relationId,
  materials,
  canEdit,
  onClose,
  onUpdated,
}: RelationCardModalProps) {
  const [relation, setRelation] = useState<RelationDetailsDto | null>(null);
  const [relationType, setRelationType] = useState<RelationType>("related_to");
  const [title, setTitle] = useState("");
  const [basis, setBasis] = useState("");
  const [confidenceLevel, setConfidenceLevel] =
    useState<ConfidenceLevel>("medium");
  const [supportingMaterialId, setSupportingMaterialId] = useState("");
  const [analystComment, setAnalystComment] = useState("");
  const [includeInReport, setIncludeInReport] = useState(true);

  const [isLoading, setIsLoading] = useState(true);
  const [isSaving, setIsSaving] = useState(false);
  const [error, setError] = useState<string | null>(null);

  async function loadRelation() {
    setIsLoading(true);
    setError(null);

    try {
      const response = await getRelationById(caseId, relationId);
      const item = response.relation;

      setRelation(item);
      setRelationType(item.relationType);
      setTitle(item.title ?? "");
      setBasis(item.basis);
      setConfidenceLevel(item.confidenceLevel);
      setSupportingMaterialId(item.supportingMaterial?.id ?? "");
      setAnalystComment(item.analystComment ?? "");
      setIncludeInReport(item.includeInReport);
    } catch (err) {
      setError(err instanceof Error ? err.message : "Не удалось загрузить связь.");
    } finally {
      setIsLoading(false);
    }
  }

  useEffect(() => {
    void loadRelation();
  }, [caseId, relationId]);

  async function handleSubmit(event: FormEvent) {
    event.preventDefault();

    if (!canEdit) {
      return;
    }

    if (!basis.trim()) {
      setError("Укажите основание связи.");
      return;
    }

    setIsSaving(true);
    setError(null);

    try {
      const response = await updateRelation({
        caseId,
        relationId,
        relationType,
        title: title.trim() || undefined,
        basis: basis.trim(),
        confidenceLevel,
        supportingMaterialId: supportingMaterialId || undefined,
        analystComment: analystComment.trim() || undefined,
        includeInReport,
      });

      setRelation(response.relation);
      onUpdated();
      onClose();
    } catch (err) {
      setError(err instanceof Error ? err.message : "Не удалось сохранить связь.");
    } finally {
      setIsSaving(false);
    }
  }

  return (
    <div className="modal-backdrop">
      <div className="modal modal--wide">
        <div className="modal__header">
          <div>
            <h2>{relation ? relation.relationCode : "Карточка связи"}</h2>
            <p>Просмотр и редактирование аналитической связи.</p>
          </div>

          <button type="button" onClick={onClose}>
            ×
          </button>
        </div>

        {error && <div className="error-box">{error}</div>}

        {isLoading ? (
          <p>Загрузка связи...</p>
        ) : relation ? (
          <form onSubmit={handleSubmit} className="form">
            <div className="details-grid">
              <div>
                <strong>Первый объект</strong>
                <p>
                  {relation.sourceObject.objectCode} ·{" "}
                  {getObjectTypeLabel(relation.sourceObject.objectType)}
                </p>
                <p>{relation.sourceObject.title}</p>
              </div>

              <div>
                <strong>Второй объект</strong>
                <p>
                  {relation.targetObject.objectCode} ·{" "}
                  {getObjectTypeLabel(relation.targetObject.objectType)}
                </p>
                <p>{relation.targetObject.title}</p>
              </div>
            </div>

            <label>
              Тип связи
              <select
                value={relationType}
                disabled={!canEdit}
                onChange={(event) =>
                  setRelationType(event.target.value as RelationType)
                }
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
              <input
                value={title}
                disabled={!canEdit}
                onChange={(event) => setTitle(event.target.value)}
              />
            </label>

            <label>
              Основание связи
              <textarea
                value={basis}
                disabled={!canEdit}
                rows={5}
                onChange={(event) => setBasis(event.target.value)}
              />
            </label>

            <label>
              Достоверность
              <select
                value={confidenceLevel}
                disabled={!canEdit}
                onChange={(event) =>
                  setConfidenceLevel(event.target.value as ConfidenceLevel)
                }
              >
                {confidenceLevelOptions.map((option) => (
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
                disabled={!canEdit}
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
                disabled={!canEdit}
                rows={3}
                onChange={(event) => setAnalystComment(event.target.value)}
              />
            </label>

            <label>
              <input
                type="checkbox"
                checked={includeInReport}
                disabled={!canEdit}
                onChange={(event) => setIncludeInReport(event.target.checked)}
              />
              Включить в DOCX
            </label>

            <div className="details-grid">
              <div>
                <strong>Создано</strong>
                <p>{relation.createdAt}</p>
              </div>

              <div>
                <strong>Обновлено</strong>
                <p>{relation.updatedAt}</p>
              </div>
            </div>

            <div className="modal__footer">
              <button type="button" onClick={onClose}>
                Закрыть
              </button>

              {canEdit && (
                <button type="submit" disabled={isSaving}>
                  {isSaving ? "Сохранение..." : "Сохранить"}
                </button>
              )}
            </div>
          </form>
        ) : (
          <p>Связь не найдена.</p>
        )}
      </div>
    </div>
  );
}
