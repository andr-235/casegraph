import type { ReactNode } from "react";

type Props = {
  title: string;
  children: ReactNode;
};

/**
 * InspectorSection — группа контента в инспекторе.
 */
export function InspectorSection({ title, children }: Props) {
  return (
    <div style={{ display: "flex", flexDirection: "column", gap: "var(--space-2)" }}>
      <span
        style={{
          fontSize: 11,
          fontWeight: 600,
          color: "var(--text-muted)",
          textTransform: "uppercase",
          letterSpacing: "0.5px",
        }}
      >
        {title}
      </span>
      {children}
    </div>
  );
}
