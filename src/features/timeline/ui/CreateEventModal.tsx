import { FormEvent, useState } from "react";

import { createEvent } from "../api/timelineApi";
import { createEmptyEventPayload } from "../model/createEventDefaults";
import { toggleSelectedId } from "../lib/toggleSelectedId";
import {
  datePrecisionOptions,
  eventTypeOptions,
} from "../model/timelineOptions";
import type { TimelineEventDto } from "../model/timelineTypes";

type SelectOption = {
  id: string;
  label: string;
};

type CreateEventModalProps = {
  caseId: string;
  objectOptions: SelectOption[];
  materialOptions: SelectOption[];
  onClose: () => void;
  onCreated: (event: TimelineEventDto) => void;
};

export function CreateEventModal({
  caseId,
  objectOptions,
  materialOptions,
  onClose,
  onCreated,
}: CreateEventModalProps) {
  const [form, setForm] = useState(() => createEmptyEventPayload(caseId));
  const [isSubmitting, setIsSubmitting] = useState(false);
  const [errorMessage, setErrorMessage] = useState("");

  function updateForm<K extends keyof typeof form>(key: K, value: (typeof form)[K]) {
    setForm((current) => ({
      ...current,
      [key]: value,
    }));
  }

  async function handleSubmit(event: FormEvent) {
    event.preventDefault();
    setErrorMessage("");
    setIsSubmitting(true);

    try {
      const result = await createEvent({
        ...form,
        eventTime: form.eventTime || undefined,
        periodStart: form.periodStart || undefined,
        periodEnd: form.periodEnd || undefined,
      });

      onCreated(result.eventItem);
    } catch (unknownError) {
      setErrorMessage(
        unknownError instanceof Error
          ? unknownError.message
          : "Не удалось создать событие",
      );
    } finally {
      setIsSubmitting(false);
    }
  }

  return (
    <div className="modal-backdrop">
      <div className="modal-card">
        <div className="modal-header">
          <h2>Создать событие</h2>
          <button type="button" onClick={onClose}>
            ×
          </button>
        </div>

        <form onSubmit={handleSubmit} className="form-grid">
          {errorMessage && <div className="error-box">{errorMessage}</div>}

          <label>
            Тип события
            <select
              value={form.eventType}
              onChange={(event) => updateForm("eventType", event.target.value as typeof form.eventType)}
            >
              {eventTypeOptions.map((option) => (
                <option key={option.value} value={option.value}>
                  {option.label}
                </option>
              ))}
            </select>
          </label>

          <label>
            Название
            <input
              value={form.title}
              onChange={(event) => updateForm("title", event.target.value)}
              required
            />
          </label>

          <label>
            Дата события
            <input
              type="date"
              value={form.eventDate}
              onChange={(event) => updateForm("eventDate", event.target.value)}
              required
            />
          </label>

          <label>
            Время
            <input
              type="time"
              value={form.eventTime ?? ""}
              onChange={(event) => updateForm("eventTime", event.target.value || undefined)}
            />
          </label>

          <label>
            Точность даты
            <select
              value={form.datePrecision}
              onChange={(event) =>
                updateForm("datePrecision", event.target.value as typeof form.datePrecision)
              }
            >
              {datePrecisionOptions.map((option) => (
                <option key={option.value} value={option.value}>
                  {option.label}
                </option>
              ))}
            </select>
          </label>

          {form.datePrecision === "period" && (
            <>
              <label>
                Начало периода
                <input
                  type="date"
                  value={form.periodStart ?? ""}
                  onChange={(event) => updateForm("periodStart", event.target.value || undefined)}
                />
              </label>

              <label>
                Конец периода
                <input
                  type="date"
                  value={form.periodEnd ?? ""}
                  onChange={(event) => updateForm("periodEnd", event.target.value || undefined)}
                />
              </label>
            </>
          )}

          <label>
            Описание
            <textarea
              value={form.description}
              onChange={(event) => updateForm("description", event.target.value)}
              rows={4}
            />
          </label>

          <label>
            Основание / источник
            <textarea
              value={form.sourceNote}
              onChange={(event) => updateForm("sourceNote", event.target.value)}
              rows={3}
            />
          </label>

          <label>
            Комментарий аналитика
            <textarea
              value={form.analystComment}
              onChange={(event) => updateForm("analystComment", event.target.value)}
              rows={3}
            />
          </label>

          <label>
            Общий комментарий к связям
            <input
              value={form.linkNote}
              onChange={(event) => updateForm("linkNote", event.target.value)}
            />
          </label>

          <fieldset>
            <legend>Связанные объекты</legend>
            {objectOptions.length === 0 ? (
              <p>Объектов пока нет.</p>
            ) : (
              objectOptions.map((object) => (
                <label key={object.id}>
                  <input
                    type="checkbox"
                    checked={form.objectIds.includes(object.id)}
                    onChange={() =>
                      updateForm("objectIds", toggleSelectedId(form.objectIds, object.id))
                    }
                  />
                  {object.label}
                </label>
              ))
            )}
          </fieldset>

          <fieldset>
            <legend>Связанные материалы</legend>
            {materialOptions.length === 0 ? (
              <p>Материалов пока нет.</p>
            ) : (
              materialOptions.map((material) => (
                <label key={material.id}>
                  <input
                    type="checkbox"
                    checked={form.materialIds.includes(material.id)}
                    onChange={() =>
                      updateForm("materialIds", toggleSelectedId(form.materialIds, material.id))
                    }
                  />
                  {material.label}
                </label>
              ))
            )}
          </fieldset>

          <label>
            <input
              type="checkbox"
              checked={form.includeInReport}
              onChange={(event) => updateForm("includeInReport", event.target.checked)}
            />
            Включить в справку
          </label>

          <div className="modal-actions">
            <button type="button" onClick={onClose}>
              Отмена
            </button>
            <button type="submit" disabled={isSubmitting}>
              {isSubmitting ? "Сохранение..." : "Создать"}
            </button>
          </div>
        </form>
      </div>
    </div>
  );
}
