import type { MaterialType } from "../model/materialTypes";

export const materialTypeOptions: Array<{
  value: MaterialType;
  label: string;
}> = [
  { value: "image", label: "Изображение" },
  { value: "pdf", label: "PDF" },
  { value: "document", label: "Документ" },
  { value: "spreadsheet", label: "Таблица" },
  { value: "text", label: "Текст" },
  { value: "html", label: "HTML" },
  { value: "other", label: "Другое" },
];

export function getMaterialTypeLabel(value: string): string {
  return (
    materialTypeOptions.find((option) => option.value === value)?.label ?? value
  );
}

const integrityStatusLabels: Record<string, string> = {
  not_checked: "Не проверено",
  ok: "OK",
  mismatch: "Несовпадение",
  missing: "Файл отсутствует",
  read_error: "Ошибка чтения",
};

export function getIntegrityStatusLabel(status: string): string {
  return integrityStatusLabels[status] ?? status;
}
