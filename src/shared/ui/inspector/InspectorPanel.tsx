import type { ReactNode } from "react";

type Props = {
  children: ReactNode;
  visible: boolean;
};

/**
 * InspectorPanel — правая панель инспектора.
 * Появляется/исчезает с CSS transition ширины.
 */
export function InspectorPanel({ children, visible }: Props) {
  return (
    <div
      style={{
        width: visible ? "var(--inspector-width)" : 0,
        overflow: "hidden",
        borderLeft: visible ? "1px solid var(--border-subtle)" : "none",
        background: "var(--bg-surface)",
        transition: "width 200ms ease",
        flexShrink: 0,
        display: "flex",
        flexDirection: "column",
      }}
    >
      <div
        style={{
          width: "var(--inspector-width)",
          height: "100%",
          padding: "var(--space-5)",
          boxSizing: "border-box",
          overflowY: "auto",
          display: "flex",
          flexDirection: "column",
          gap: "var(--space-4)",
        }}
      >
        {children}
      </div>
    </div>
  );
}
