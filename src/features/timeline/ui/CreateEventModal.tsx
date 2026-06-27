import { FormEvent, useState } from "react";

import { createEvent } from "../api/timelineApi";
import {
  datePrecisionOptions,
  eventTypeOptions,
} from "../model/timelineOptions";
import type {
  DatePrecision,
  EventType,
  TimelineEventDto,
} from "../model/timelineTypes";

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
  const [eventType, setEventType] = useState<EventType>("fact");
  const [title, setTitle] = useState("");
  const [description, setDescription] = useState("");
  const [eventDate, setEventDate] = useState("");
  const [eventTime, setEventTime] = useState("");
  const [datePrecision, setDatePrecision] = useState<DatePrecision>("day");
  const [periodStart, setPeriodStart] = useState("");
  const [periodEnd, setPeriodEnd] = useState("");
  const [sourceNote, setSourceNote] = useState("");
  const [analystComment, setAnalystComment] = useState("");
  const [includeInReport, setIncludeInReport] = useState(true);
  const [objectIds, setObjectIds] = useState<string[]>([]);
  const [materialIds, setMaterialIds] = useState<string[]>([]);
  const [linkNote, setLinkNote] = useState("");

  const [isSubmitting, setIsSubmitting] = useState(false);
  const [errorMessage, setErrorMessage] = useState("");

  function toggleSelected(
    current: string[],
    value: string,
    setter: (next: string[]) => void,
  ) {
    if (current.includes(value)) {
      setter(current.filter((item) => item !== value));
      return;
    }

    setter([...current, value]);
  }

  async function handleSubmit(event: FormEvent) {
    event.preventDefault();
    setErrorMessage("");
    setIsSubmitting(true);

    try {
      const result = await createEvent({
        caseId,
        eventType,
        title,
        description,
        eventDate,
        eventTime: eventTime || undefined,
        datePrecision,
        periodStart: periodStart || undefined,
        periodEnd: periodEnd || undefined,
        sourceNote,
        analystComment,
        includeInReport,
        objectIds,
        materialIds,
        linkNote,
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
              value={eventType}
              onChange={(event) => setEventType(event.target.value as EventType)}
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
              value={title}
              onChange={(event) => setTitle(event.target.value)}
              required
            />
          </label>

          <label>
            Дата события
            <input
              type="date"
              value={eventDate}
              onChange={(event) => setEventDate(event.target.value)}
              required
            />
          </label>

          <label>
            Время
            <input
              type="time"
              value={eventTime}
              onChange={(event) => setEventTime(event.target.value)}
            />
          </label>

          <label>
            Точность даты
            <select
              value={datePrecision}
              onChange={(event) =>
                setDatePrecision(event.target.value as DatePrecision)
              }
            >
              {datePrecisionOptions.map((option) => (
                <option key={option.value} value={option.value}>
                  {option.label}
                </option>
              ))}
            </select>
          </label>

          {datePrecision === "period" && (
            <>
              <label>
                Начало периода
                <input
                  type="date"
                  value={periodStart}
                  onChange={(event) => setPeriodStart(event.target.value)}
                />
              </label>

              <label>
                Конец периода
                <input
                  type="date"
                  value={periodEnd}
                  onChange={(event) => setPeriodEnd(event.target.value)}
                />
              </label>
            </>
          )}

          <label>
            Описание
            <textarea
              value={description}
              onChange={(event) => setDescription(event.target.value)}
              rows={4}
            />
          </label>

          <label>
            Основание / источник
            <textarea
              value={sourceNote}
              onChange={(event) => setSourceNote(event.target.value)}
              rows={3}
            />
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
            Общий комментарий к связям
            <input
              value={linkNote}
              onChange={(event) => setLinkNote(event.target.value)}
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
                    checked={objectIds.includes(object.id)}
                    onChange={() =>
                      toggleSelected(objectIds, object.id, setObjectIds)
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
                    checked={materialIds.includes(material.id)}
                    onChange={() =>
                      toggleSelected(materialIds, material.id, setMaterialIds)
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
              checked={includeInReport}
              onChange={(event) => setIncludeInReport(event.target.checked)}
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
