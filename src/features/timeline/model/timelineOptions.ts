import type { DatePrecision, EventType } from "./timelineTypes";

export const eventTypeOptions: Array<{ value: EventType; label: string }> = [
  { value: "fact", label: "Факт" },
  { value: "action", label: "Действие" },
  { value: "observation", label: "Наблюдение" },
  { value: "document_fixation", label: "Фиксация документа" },
  { value: "relation_established", label: "Установление связи" },
  { value: "material_received", label: "Получение материала" },
  { value: "other", label: "Иное" },
];

export const datePrecisionOptions: Array<{
  value: DatePrecision;
  label: string;
}> = [
  { value: "day", label: "Точная дата" },
  { value: "month", label: "Месяц" },
  { value: "year", label: "Год" },
  { value: "approximate", label: "Примерно" },
  { value: "period", label: "Период" },
];

export function getEventTypeLabel(value: string): string {
  return eventTypeOptions.find((option) => option.value === value)?.label ?? value;
}

export function getDatePrecisionLabel(value: string): string {
  return (
    datePrecisionOptions.find((option) => option.value === value)?.label ?? value
  );
}
