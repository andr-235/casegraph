import { useState, type FormEvent } from "react";

import { createEvent } from "../api/timelineApi";
import { toggleSelectedId } from "../lib/toggleSelectedId";
import { createEmptyEventPayload } from "../model/createEventDefaults";
import {
  datePrecisionOptions,
  eventTypeOptions,
} from "../model/timelineOptions";
import type {
  CreateEventPayload,
  TimelineEventDto,
} from "../model/timelineTypes";

type LinkOption = {
  id: string;
  label: string;
};

type CreateEventModalProps = {
  caseId: string;
  objectOptions: LinkOption[];
  materialOptions: LinkOption[];
  onClose: () => void;
  onCreated: (eventItem: TimelineEventDto) => void;
};

export function CreateEventModal({
  caseId,
  objectOptions,
  materialOptions,
  onClose,
  onCreated,
}: CreateEventModalProps) {
  const [form, setForm] = useState<CreateEventPayload>(() =>
    createEmptyEventPayload(caseId),
  );
  const [submitting, setSubmitting] = useState(false);
  const [errorMessage, setErrorMessage] = useState("");

  function updateForm<K extends keyof CreateEventPayload>(
    key: K,
    value: CreateEventPayload[K],
  ) {
    setForm((current) => ({
      ...current,
      [key]: value,
    }));
  }

  async function handleSubmit(event: FormEvent<HTMLFormElement>) {
    event.preventDefault();

    if (submitting) {
      return;
    }

    setSubmitting(true);
    setErrorMessage("");

    try {
      const payload: CreateEventPayload = {
        ...form,
        title: form.title.trim(),
        description: form.description.trim(),
        sourceNote: form.sourceNote.trim(),
        analystComment: form.analystComment.trim(),
        linkNote: form.linkNote.trim(),
        eventDate: form.eventDate.trim(),
        eventTime: form.eventTime?.trim() || undefined,
        periodStart: form.periodStart?.trim() || undefined,
        periodEnd: form.periodEnd?.trim() || undefined,
      };

      const result = await createEvent(payload);

      onCreated(result.eventItem);
    } catch (unknownError) {
      setErrorMessage(
        unknownError instanceof Error
          ? unknownError.message
          : "Не удалось создать событие",
      );
    } finally {
      setSubmitting(false);
    }
  }

  const isPeriod = form.datePrecision === "period";

  return (
    <div className="modal-backdrop">
      <div className="modal-card">
        <div className="modal-header">
          <div>
            <h2>Создать событие</h2>
            <p>Событие будет добавлено в хронологию текущего дела.</p>
          </div>

          <button type="button" onClick={onClose} disabled={submitting}>
            ×
          </button>
        </div>

        <form onSubmit={handleSubmit} className="form-grid">
          {errorMessage && (
            <div className="error-box">{errorMessage}</div>
          )}

          <label>
            Тип события
            <select
              value={form.eventType}
              onChange={(event) =>
                updateForm("eventType", event.target.value as CreateEventPayload["eventType"])
              }
              disabled={submitting}
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
              disabled={submitting}
              required
              maxLength={200}
            />
          </label>

          <label>
            Описание
            <textarea
              value={form.description}
              onChange={(event) =>
                updateForm("description", event.target.value)
              }
              disabled={submitting}
              rows={4}
            />
          </label>

          <div className="form-row">
            <label>
              Дата
              <input
                type="date"
                value={form.eventDate}
                onChange={(event) =>
                  updateForm("eventDate", event.target.value)
                }
                disabled={submitting}
                required
              />
            </label>

            <label>
              Время
              <input
                type="time"
                value={form.eventTime ?? ""}
                onChange={(event) =>
                  updateForm("eventTime", event.target.value || undefined)
                }
                disabled={submitting}
              />
            </label>
          </div>

          <label>
            Точность даты
            <select
              value={form.datePrecision}
              onChange={(event) =>
                updateForm(
                  "datePrecision",
                  event.target.value as CreateEventPayload["datePrecision"],
                )
              }
              disabled={submitting}
            >
              {datePrecisionOptions.map((option) => (
                <option key={option.value} value={option.value}>
                  {option.label}
                </option>
              ))}
            </select>
          </label>

          {isPeriod && (
            <div className="form-row">
              <label>
                Начало периода
                <input
                  type="date"
                  value={form.periodStart ?? ""}
                  onChange={(event) =>
                    updateForm("periodStart", event.target.value || undefined)
                  }
                  disabled={submitting}
                />
              </label>

              <label>
                Конец периода
                <input
                  type="date"
                  value={form.periodEnd ?? ""}
                  onChange={(event) =>
                    updateForm("periodEnd", event.target.value || undefined)
                  }
                  disabled={submitting}
                />
              </label>
            </div>
          )}

          <label>
            Основание / источник
            <textarea
              value={form.sourceNote}
              onChange={(event) =>
                updateForm("sourceNote", event.target.value)
              }
              disabled={submitting}
              rows={3}
            />
          </label>

          <label>
            Комментарий аналитика
            <textarea
              value={form.analystComment}
              onChange={(event) =>
                updateForm("analystComment", event.target.value)
              }
              disabled={submitting}
              rows={3}
            />
          </label>

          <label className="checkbox-row">
            <input
              type="checkbox"
              checked={form.includeInReport}
              onChange={(event) =>
                updateForm("includeInReport", event.target.checked)
              }
              disabled={submitting}
            />
            Включить в справку
          </label>

          <fieldset>
            <legend>Связанные объекты</legend>

            {objectOptions.length === 0 ? (
              <p>В деле пока нет объектов.</p>
            ) : (
              <div className="checkbox-list">
                {objectOptions.map((object) => (
                  <label key={object.id} className="checkbox-row">
                    <input
                      type="checkbox"
                      checked={form.objectIds.includes(object.id)}
                      onChange={() =>
                        updateForm(
                          "objectIds",
                          toggleSelectedId(form.objectIds, object.id),
                        )
                      }
                      disabled={submitting}
                    />
                    {object.label}
                  </label>
                ))}
              </div>
            )}
          </fieldset>

          <fieldset>
            <legend>Связанные материалы</legend>

            {materialOptions.length === 0 ? (
              <p>В деле пока нет материалов.</p>
            ) : (
              <div className="checkbox-list">
                {materialOptions.map((material) => (
                  <label key={material.id} className="checkbox-row">
                    <input
                      type="checkbox"
                      checked={form.materialIds.includes(material.id)}
                      onChange={() =>
                        updateForm(
                          "materialIds",
                          toggleSelectedId(form.materialIds, material.id),
                        )
                      }
                      disabled={submitting}
                    />
                    {material.label}
                  </label>
                ))}
              </div>
            )}
          </fieldset>

          <label>
            Комментарий к связям
            <textarea
              value={form.linkNote}
              onChange={(event) => updateForm("linkNote", event.target.value)}
              disabled={submitting}
              rows={2}
            />
          </label>

          <div className="modal-actions">
            <button type="button" onClick={onClose} disabled={submitting}>
              Отмена
            </button>

            <button type="submit" disabled={submitting}>
              {submitting ? "Сохранение..." : "Создать событие"}
            </button>
          </div>
        </form>
      </div>
    </div>
  );
}
