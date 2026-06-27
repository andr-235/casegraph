import type { ReactNode } from "react";

type ConfirmModalTone = "danger" | "warning" | "neutral";

type ConfirmModalProps = {
  title: string;
  confirmText: string;
  cancelText: string;
  tone?: ConfirmModalTone;
  disabled?: boolean;
  onConfirm: () => void;
  onCancel: () => void;
  children?: ReactNode;
};

export function ConfirmModal({
  title,
  confirmText,
  cancelText,
  tone = "neutral",
  disabled = false,
  onConfirm,
  onCancel,
  children,
}: ConfirmModalProps) {
  const confirmButtonStyle: React.CSSProperties = {
    padding: "8px 18px",
    borderRadius: 6,
    border: "none",
    cursor: disabled ? "default" : "pointer",
    fontWeight: 600,
    fontSize: "0.9rem",
    opacity: disabled ? 0.6 : 1,
    background:
      tone === "danger"
        ? "#dc2626"
        : tone === "warning"
          ? "#d97706"
          : "#2563eb",
    color: "#fff",
  };

  return (
    <>
      {/* Backdrop */}
      <div
        onClick={onCancel}
        style={{
          position: "fixed",
          inset: 0,
          background: "rgba(0,0,0,0.45)",
          backdropFilter: "blur(2px)",
          zIndex: 1000,
        }}
      />

      {/* Dialog */}
      <div
        role="dialog"
        aria-modal="true"
        aria-labelledby="confirm-modal-title"
        style={{
          position: "fixed",
          top: "50%",
          left: "50%",
          transform: "translate(-50%, -50%)",
          zIndex: 1001,
          background: "#1c1f26",
          border: "1px solid #2d3142",
          borderRadius: 12,
          padding: "28px 32px",
          width: 420,
          maxWidth: "90vw",
          boxShadow: "0 24px 64px rgba(0,0,0,0.5)",
          color: "#e2e8f0",
        }}
      >
        <h2
          id="confirm-modal-title"
          style={{ margin: "0 0 16px", fontSize: "1.15rem", fontWeight: 700 }}
        >
          {title}
        </h2>

        <div style={{ fontSize: "0.93rem", color: "#94a3b8", lineHeight: 1.6 }}>
          {children}
        </div>

        <div
          style={{
            display: "flex",
            justifyContent: "flex-end",
            gap: 10,
            marginTop: 24,
          }}
        >
          <button
            type="button"
            onClick={onCancel}
            disabled={disabled}
            style={{
              padding: "8px 18px",
              borderRadius: 6,
              border: "1px solid #374151",
              background: "transparent",
              color: "#94a3b8",
              cursor: disabled ? "default" : "pointer",
              fontSize: "0.9rem",
            }}
          >
            {cancelText}
          </button>

          <button
            type="button"
            onClick={onConfirm}
            disabled={disabled}
            style={confirmButtonStyle}
          >
            {confirmText}
          </button>
        </div>
      </div>
    </>
  );
}
