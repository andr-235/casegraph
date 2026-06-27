export const relationTypeOptions = [
  { value: "related_to", label: "Связан с" },
  { value: "uses", label: "Использует" },
  { value: "belongs_to", label: "Принадлежит" },
  { value: "mentioned_in", label: "Упоминается в" },
  { value: "appears_with", label: "Фигурирует совместно" },
  { value: "confirmed_by_material", label: "Подтверждается материалом" },
  { value: "linked_to_phone", label: "Связан с номером" },
  { value: "linked_to_account", label: "Связан с аккаунтом" },
  { value: "linked_to_document", label: "Связан с документом" },
  { value: "linked_to_vehicle", label: "Связан с транспортом" },
  { value: "linked_to_address", label: "Связан с адресом" },
  { value: "linked_to_organization", label: "Связан с организацией" },
  { value: "other", label: "Иная связь" },
] as const;

export const relationConfidenceOptions = [
  { value: "high", label: "Высокая" },
  { value: "medium", label: "Средняя" },
  { value: "low", label: "Низкая" },
  { value: "requires_check", label: "Требует проверки" },
] as const;

export function getRelationTypeLabel(value: string): string {
  return relationTypeOptions.find((option) => option.value === value)?.label ?? value;
}

export function getRelationConfidenceLabel(value: string): string {
  return relationConfidenceOptions.find((option) => option.value === value)?.label ?? value;
}
