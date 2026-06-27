import { useEffect, useState } from "react";

import {
  getEventById,
  softDeleteEvent,
  updateEvent,
} from "../api/timelineApi";
import { toggleSelectedId } from "../lib/toggleSelectedId";
import {
  datePrecisionOptions,
  eventTypeOptions,
} from "../model/timelineOptions";
import {
  createUpdateEventPayloadFromDetails,
  sanitizeUpdateEventPayload,
} from "../model/eventFormMappers";
import type {
  EventDetailsDto,
  UpdateEventPayload,
} from "../model/timelineTypes";

type LinkOption = {
  id: string;
  label: string;
};

type EventCardModalProps = {
  caseId: string;
  eventId: string;
  objectOptions: LinkOption[];
  materialOptions: LinkOption[];
  readonly: boolean;
  onClose: () => void;
  onSaved: () => void;
  onDeleted: () => void;
};

export function EventCardModal({
  caseId,
  eventId,
  objectOptions,
  materialOptions,
  readonly,
  onClose,
  onSaved,
  onDeleted,
}: EventCardModalProps) {
  const [eventDetails, setEventDetails] = useState<EventDetailsDto | null>(null);
  const [form, setForm] = useState<UpdateEventPayload | null>(null);
  const [isLoading, setIsLoading] = useState(true);
  const [isSaving, setIsSaving] = useState(false);
  const [errorMessage, setErrorMessage] = useState("");

  async function loadEvent() {
    setIsLoading(true);
    setErrorMessage("");

    try {
      const response = await getEventById({ caseId, eventId });
      const details = response.eventDetails;

      setEventDetails(details);
      setForm(createUpdateEventPayloadFromDetails(caseId, eventId, details));
    } catch (unknownError) {
      setErrorMessage(
        unknownError instanceof Error
          ? unknownError.message
          : "Не удалось загрузить событие",
      );
    } finally {
      setIsLoading(false);
    }
  }

  useEffect(() => {
    void loadEvent();
  }, [caseId, eventId]);

  function updateForm<K extends keyof UpdateEventPayload>(
    key: K,
    value: UpdateEventPayload[K],
  ) {
    setForm((current) => {
      if (!current) {
        return current;
      }

      return {
        ...current,
        [key]: value,
      };
    });
  }

  async function handleSave() {
    if (!form || isSaving || readonly) {
      return;
    }

    setIsSaving(true);
    setErrorMessage("");

    const payload = sanitizeUpdateEventPayload(form);

    if (!payload.title) {
      setErrorMessage("Название события обязательно");
      setIsSaving(false);
      return;
    }

    if (!payload.eventDate) {
      setErrorMessage("Дата события обязательна");
      setIsSaving(false);
      return;
    }

    try {
      await updateEvent(payload);
      onSaved();
      onClose();
    } catch (unknownError) {
      setErrorMessage(
        unknownError instanceof Error
          ? unknownError.message
          : "Не удалось сохранить событие",
      );
    } finally {
      setIsSaving(false);
    }
  }

  async function handleDelete() {
    if (isSaving || readonly) {
      return;
    }

    const confirmed = window.confirm(
      "Удалить событие из хронологии? Действие скроет событие из списка, но не удалит данные физически.",
    );

    if (!confirmed) {
      return;
    }

    setIsSaving(true);
    setErrorMessage("");

    try {
      await softDeleteEvent({ caseId, eventId });
      onDeleted();
      onClose();
    } catch (unknownError) {
      setErrorMessage(
        unknownError instanceof Error
          ? unknownError.message
          : "Не удалось удалить событие",
      );
    } finally {
      setIsSaving(false);
    }
  }

  const isReadonly = readonly || isSaving;
  const isPeriod = form?.datePrecision === "period";

  return (
    <div className="modal-backdrop">
      <div className="modal modal--wide">
        <div className="modal__header">
          <div>
            <h2>
              {eventDetails?.eventItem.eventCode ?? "Событие"}
            </h2>
            <p>Карточка события хронологии.</p>
          </div>

          <button type="button" onClick={onClose} disabled={isSaving}>
            ×
          </button>
        </div>

        <div className="modal__body">
          {isLoading && <p>Загрузка события...</p>}

          {errorMessage && (
            <div className="error-state">{errorMessage}</div>
          )}

          {!isLoading && form && (
            <>
              <label>
                Тип события
                <select
                  value={form.eventType}
                  onChange={(event) =>
                    updateForm(
                      "eventType",
                      event.target.value as UpdateEventPayload["eventType"],
                    )
                  }
                  disabled={isReadonly}
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
                  disabled={isReadonly}
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
                  disabled={isReadonly}
                  rows={4}
                />
              </label>

              <div className="form-grid">
                <label>
                  Дата
                  <input
                    type="date"
                    value={form.eventDate}
                    onChange={(event) =>
                      updateForm("eventDate", event.target.value)
                    }
                    disabled={isReadonly}
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
                    disabled={isReadonly}
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
                      event.target.value as UpdateEventPayload["datePrecision"],
                    )
                  }
                  disabled={isReadonly}
                >
                  {datePrecisionOptions.map((option) => (
                    <option key={option.value} value={option.value}>
                      {option.label}
                    </option>
                  ))}
                </select>
              </label>

              {isPeriod && (
                <div className="form-grid">
                  <label>
                    Начало периода
                    <input
                      type="date"
                      value={form.periodStart ?? ""}
                      onChange={(event) =>
                        updateForm("periodStart", event.target.value || undefined)
                      }
                      disabled={isReadonly}
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
                      disabled={isReadonly}
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
                  disabled={isReadonly}
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
                  disabled={isReadonly}
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
                  disabled={isReadonly}
                />
                Включить в справку
              </label>

              <section>
                <h3>Связанные объекты</h3>

                {objectOptions.length === 0 ? (
                  <p className="muted">В деле пока нет объектов.</p>
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
                          disabled={isReadonly}
                        />
                        {object.label}
                      </label>
                    ))}
                  </div>
                )}
              </section>

              <section>
                <h3>Связанные материалы</h3>

                {materialOptions.length === 0 ? (
                  <p className="muted">В деле пока нет материалов.</p>
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
                          disabled={isReadonly}
                        />
                        {material.label}
                      </label>
                    ))}
                  </div>
                )}
              </section>

              <label>
                Комментарий к связям
                <textarea
                  value={form.linkNote}
                  onChange={(event) =>
                    updateForm("linkNote", event.target.value)
                  }
                  disabled={isReadonly}
                  rows={2}
                />
              </label>
            </>
          )}
        </div>

        <div className="modal__footer">
          <button type="button" onClick={onClose} disabled={isSaving}>
            Закрыть
          </button>

          {!readonly && (
            <div className="modal__footer-actions">
              <button
                type="button"
                onClick={handleDelete}
                disabled={isSaving || isLoading}
              >
                Удалить
              </button>

              <button
                type="button"
                onClick={handleSave}
                disabled={isSaving || isLoading || !form}
              >
                {isSaving ? "Сохранение..." : "Сохранить"}
              </button>
            </div>
          )}
        </div>
      </div>
    </div>
  );
}
