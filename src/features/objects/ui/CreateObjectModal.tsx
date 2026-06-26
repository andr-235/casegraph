import { FormEvent, useState } from "react";

import { createObject } from "../api/objectsApi";
import { objectTypeOptions } from "../model/objectOptions";
import type { CreateObjectPayload, ObjectListItemDto, ObjectType } from "../model/objectTypes";

type CreateObjectModalProps = {
  caseId: string;
  onClose: () => void;
  onCreated: (objectItem: ObjectListItemDto) => void;
};

export function CreateObjectModal({
  caseId,
  onClose,
  onCreated,
}: CreateObjectModalProps) {
  const [objectType, setObjectType] = useState<ObjectType>("person");
  const [title, setTitle] = useState("");
  const [value, setValue] = useState("");
  const [description, setDescription] = useState("");
  const [isKey, setIsKey] = useState(false);
  const [includeInReport, setIncludeInReport] = useState(true);
  const [error, setError] = useState("");
  const [isSubmitting, setIsSubmitting] = useState(false);

  async function handleSubmit(event: FormEvent) {
    event.preventDefault();

    const normalizedTitle = title.trim();

    if (normalizedTitle.length < 2) {
      setError("Название объекта должно содержать минимум 2 символа.");
      return;
    }

    const payload: CreateObjectPayload = {
      caseId,
      objectType,
      title: normalizedTitle,
      value: value.trim() || undefined,
      description: description.trim() || undefined,
      isKey,
      includeInReport,
    };

    setIsSubmitting(true);
    setError("");

    try {
      const response = await createObject(payload);
      onCreated(response.objectItem);
    } catch (unknownError) {
      setError(
        unknownError instanceof Error
          ? unknownError.message
          : "Не удалось создать объект.",
      );
    } finally {
      setIsSubmitting(false);
    }
  }

  return (
    <div className="modal-backdrop">
      <div className="modal">
        <h2>Создать объект</h2>

        {error && <div className="error-message">{error}</div>}

        <form onSubmit={handleSubmit}>
          <label>
            Тип объекта
            <select
              value={objectType}
              onChange={(event) => setObjectType(event.target.value as ObjectType)}
            >
              {objectTypeOptions.map((option) => (
                <option key={option.value} value={option.value}>
                  {option.label}
                </option>
              ))}
            </select>
          </label>

          <label>
            Название
            <input
              value={title}
              onChange={(event) => setTitle(event.target.value)}
              placeholder="Например: Иванов И.И."
            />
          </label>

          <label>
            Значение
            <input
              value={value}
              onChange={(event) => setValue(event.target.value)}
              placeholder="Например: +7..."
            />
          </label>

          <label>
            Описание
            <textarea
              value={description}
              onChange={(event) => setDescription(event.target.value)}
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

          <div className="modal-actions">
            <button type="button" onClick={onClose} disabled={isSubmitting}>
              Отмена
            </button>
            <button type="submit" disabled={isSubmitting}>
              {isSubmitting ? "Создание..." : "Создать"}
            </button>
          </div>
        </form>
      </div>
    </div>
  );
}
