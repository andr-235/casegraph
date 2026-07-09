type Props = {
  code: string;
  title: string;
  onClose: () => void;
  onOpenFullCard?: () => void;
};

/**
 * InspectorHeader — заголовок инспектора с кодом, названием и кнопками.
 */
export function InspectorHeader({ code, title, onClose, onOpenFullCard }: Props) {
  return (
    <div
      style={{
        display: "flex",
        flexDirection: "column",
        gap: "var(--space-1)",
        borderBottom: "1px solid var(--border-subtle)",
        paddingBottom: "var(--space-3)",
      }}
    >
      {/* Верхняя строка: код + крестик */}
      <div
        style={{
          display: "flex",
          alignItems: "center",
          justifyContent: "space-between",
        }}
      >
        <span
          style={{
            fontFamily: "var(--font-mono)",
            fontSize: 12,
            color: "var(--text-muted)",
          }}
        >
          {code}
        </span>

        <button
          type="button"
          onClick={onClose}
          title="Закрыть (Esc)"
          style={{
            background: "none",
            border: "none",
            color: "var(--text-muted)",
            cursor: "pointer",
            fontSize: 16,
            lineHeight: 1,
            padding: 2,
          }}
        >
          ×
        </button>
      </div>

      {/* Название */}
      <span
        style={{
          fontWeight: 600,
          fontSize: 15,
          color: "var(--text-primary)",
          lineHeight: 1.3,
        }}
      >
        {title}
      </span>

      {/* Кнопка открытия полной карточки */}
      {onOpenFullCard && (
        <button
          type="button"
          onClick={onOpenFullCard}
          style={{
            alignSelf: "flex-start",
            background: "none",
            border: "none",
            color: "var(--accent)",
            cursor: "pointer",
            fontSize: 12,
            padding: 0,
            marginTop: "var(--space-1)",
          }}
        >
          Открыть карточку →
        </button>
      )}
    </div>
  );
}
