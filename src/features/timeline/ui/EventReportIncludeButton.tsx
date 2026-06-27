type EventReportIncludeButtonProps = {
  includeInReport: boolean;
  disabled: boolean;
  loading: boolean;
  onToggle: () => void;
};

export function EventReportIncludeButton({
  includeInReport,
  disabled,
  loading,
  onToggle,
}: EventReportIncludeButtonProps) {
  const label = includeInReport ? "Да" : "Нет";

  return (
    <button
      type="button"
      className="link-button"
      disabled={disabled || loading}
      aria-pressed={includeInReport}
      aria-label={
        includeInReport
          ? "Исключить событие из DOCX"
          : "Включить событие в DOCX"
      }
      onClick={onToggle}
    >
      {loading ? "..." : label}
    </button>
  );
}
