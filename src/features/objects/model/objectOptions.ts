import type { ObjectType } from "./objectTypes";

export const objectTypeOptions: Array<{ value: ObjectType; label: string }> = [
  { value: "person", label: "Персона" },
  { value: "account", label: "Аккаунт" },
  { value: "phone", label: "Телефон" },
  { value: "address", label: "Адрес" },
  { value: "vehicle", label: "Транспорт" },
  { value: "organization", label: "Организация" },
  { value: "document", label: "Документ" },
  { value: "image", label: "Изображение" },
  { value: "publication", label: "Публикация" },
  { value: "event", label: "Событие" },
  { value: "source", label: "Источник" },
  { value: "other", label: "Иной объект" },
];

export function getObjectTypeLabel(value: string): string {
  return objectTypeOptions.find((option) => option.value === value)?.label ?? value;
}
