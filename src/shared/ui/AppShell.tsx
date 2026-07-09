import type { ReactNode } from "react";

type Props = {
  children: ReactNode;
};

/**
 * AppShell — основной каркас приложения.
 * Рендерит flex-контейнер на всю высоту окна с тёмным фоном.
 */
export function AppShell({ children }: Props) {
  return (
    <div
      style={{
        height: "100vh",
        display: "flex",
        flexDirection: "column",
        background: "var(--bg-app)",
        color: "var(--text-primary)",
        fontFamily: "var(--font-ui)",
      }}
    >
      {children}
    </div>
  );
}
