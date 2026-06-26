import type { CaseStatus, EditableCaseStatus } from "./caseTypes";

export const editableCaseStatusOptions: ReadonlyArray<{
  value: EditableCaseStatus;
  label: string;
}> = [
  { value: "draft", label: "Черновик" },
  { value: "in_progress", label: "В работе" },
  { value: "prepared", label: "Подготовлено" },
  { value: "completed", label: "Завершено" },
];

export function isEditableCaseStatus(
  value: string
): value is EditableCaseStatus {
  return editableCaseStatusOptions.some((option) => option.value === value);
}

export function getCaseStatusLabel(status: CaseStatus): string {
  const option = editableCaseStatusOptions.find(
    (item) => item.value === status
  );

  if (option) {
    return option.label;
  }

  if (status === "archived") {
    return "Архив";
  }

  return status;
}