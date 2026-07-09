import { useCallback, useEffect, useRef, useState } from "react";

type UserMenuItem =
  | "settings"
  | "backup"
  | "audit"
  | "users"
  | "logout";

type Props = {
  /** Код дела (показывается в breadcrumb только внутри workspace) */
  caseCode?: string;
  /** Название дела (обрезается до ~40 символов) */
  title?: string;
  /** Отображаемое имя пользователя */
  displayName: string;
  /** Коллбэк обновления (⟳) */
  onRefresh?: () => void;
  onOpenSettings?: () => void;
  onOpenBackup?: () => void;
  onOpenAuditLog?: () => void;
  onOpenUsers?: () => void;
  onLogout?: () => void;
};

function truncate(text: string, max: number): string {
  if (text.length <= max) return text;
  return text.slice(0, max - 1) + "\u2026";
}

export function TopBar({
  caseCode,
  title,
  displayName,
  onRefresh,
  onOpenSettings,
  onOpenBackup,
  onOpenAuditLog,
  onOpenUsers,
  onLogout,
}: Props) {
  const [menuOpen, setMenuOpen] = useState(false);
  const menuRef = useRef<HTMLDivElement>(null);

  // Закрытие меню по клику вне компонента
  useEffect(() => {
    if (!menuOpen) return;

    const handleClick = (e: MouseEvent) => {
      if (menuRef.current && !menuRef.current.contains(e.target as Node)) {
        setMenuOpen(false);
      }
    };

    document.addEventListener("mousedown", handleClick);
    return () => document.removeEventListener("mousedown", handleClick);
  }, [menuOpen]);

  const handleMenuSelect = useCallback(
    (item: UserMenuItem) => {
      setMenuOpen(false);

      switch (item) {
        case "settings":
          onOpenSettings?.();
          break;
        case "backup":
          onOpenBackup?.();
          break;
        case "audit":
          onOpenAuditLog?.();
          break;
        case "users":
          onOpenUsers?.();
          break;
        case "logout":
          onLogout?.();
          break;
      }
    },
    [onOpenSettings, onOpenBackup, onOpenAuditLog, onOpenUsers, onLogout]
  );

  return (
    <header
      style={{
        height: "var(--topbar-height)",
        borderBottom: "1px solid var(--border-subtle)",
        display: "flex",
        alignItems: "center",
        padding: "0 var(--space-5)",
        boxSizing: "border-box",
        flexShrink: 0,
        background: "var(--bg-app)",
      }}
    >
      {/* Левая часть: логотип + breadcrumb */}
      <div style={{ display: "flex", alignItems: "center", gap: "var(--space-2)", flex: 1, minWidth: 0 }}>
        <span style={{ fontWeight: 700, fontSize: 16, color: "var(--accent)", whiteSpace: "nowrap" }}>
          ◈ CaseGraph
        </span>

        {caseCode && (
          <>
            <span style={{ color: "var(--text-muted)", userSelect: "none" }}>/</span>
            <span style={{ color: "var(--text-secondary)", whiteSpace: "nowrap", fontFamily: "var(--font-mono)", fontSize: 13 }}>
              {caseCode}
            </span>
          </>
        )}

        {title && (
          <>
            <span style={{ color: "var(--text-muted)", userSelect: "none" }}>/</span>
            <span
              style={{
                color: "var(--text-primary)",
                whiteSpace: "nowrap",
                overflow: "hidden",
                textOverflow: "ellipsis",
                fontSize: 14,
              }}
              title={title}
            >
              {truncate(title, 40)}
            </span>
          </>
        )}
      </div>

      {/* Правая часть: поиск, обновление, пользователь */}
      <div style={{ display: "flex", alignItems: "center", gap: "var(--space-3)", flexShrink: 0 }}>
        {/* Поиск (заглушка) */}
        <div
          style={{
            display: "flex",
            alignItems: "center",
            gap: 6,
            padding: "4px 10px",
            borderRadius: "var(--radius-sm)",
            border: "1px solid var(--border-subtle)",
            color: "var(--text-muted)",
            fontSize: 12,
            cursor: "default",
            userSelect: "none",
          }}
        >
          <span>🔍</span>
          <span style={{ fontSize: 11, opacity: 0.6 }}>Ctrl+K</span>
        </div>

        {/* Кнопка обновления */}
        {onRefresh && (
          <button
            type="button"
            onClick={onRefresh}
            title="Обновить"
            style={{
              background: "none",
              border: "1px solid var(--border-subtle)",
              borderRadius: "var(--radius-sm)",
              color: "var(--text-secondary)",
              cursor: "pointer",
              padding: "4px 8px",
              fontSize: 16,
              lineHeight: 1,
            }}
          >
            ⟳
          </button>
        )}

        {/* Меню пользователя */}
        <div ref={menuRef} style={{ position: "relative" }}>
          <button
            type="button"
            onClick={() => setMenuOpen((prev) => !prev)}
            style={{
              display: "flex",
              alignItems: "center",
              gap: 6,
              background: "none",
              border: "1px solid var(--border-subtle)",
              borderRadius: "var(--radius-sm)",
              color: "var(--text-primary)",
              cursor: "pointer",
              padding: "4px 10px",
              fontSize: 13,
            }}
          >
            <span>👤</span>
            <span>{displayName}</span>
            <span style={{ fontSize: 10, marginLeft: 2 }}>▾</span>
          </button>

          {menuOpen && (
            <div
              style={{
                position: "absolute",
                right: 0,
                top: "calc(100% + 4px)",
                minWidth: 200,
                background: "var(--bg-elevated)",
                border: "1px solid var(--border-default)",
                borderRadius: "var(--radius-md)",
                boxShadow: "0 8px 24px rgba(0,0,0,0.4)",
                zIndex: 1000,
                padding: "var(--space-1) 0",
              }}
            >
              {(
                [
                  ["settings", "Настройки"],
                  ["backup", "Резервные копии"],
                  ["audit", "Журнал аудита"],
                  ["users", "Пользователи"],
                ] as [UserMenuItem, string][]
              ).map(([key, label]) => (
                <button
                  key={key}
                  type="button"
                  onClick={() => handleMenuSelect(key)}
                  style={{
                    display: "block",
                    width: "100%",
                    textAlign: "left",
                    background: "none",
                    border: "none",
                    color: "var(--text-primary)",
                    padding: "8px 16px",
                    cursor: "pointer",
                    fontSize: 13,
                  }}
                  onMouseEnter={(e) => {
                    e.currentTarget.style.background = "var(--bg-hover)";
                  }}
                  onMouseLeave={(e) => {
                    e.currentTarget.style.background = "none";
                  }}
                >
                  {label}
                </button>
              ))}
              <div
                style={{
                  height: 1,
                  background: "var(--border-subtle)",
                  margin: "var(--space-1) 0",
                }}
              />
              <button
                type="button"
                onClick={() => handleMenuSelect("logout")}
                style={{
                  display: "block",
                  width: "100%",
                  textAlign: "left",
                  background: "none",
                  border: "none",
                  color: "var(--danger)",
                  padding: "8px 16px",
                  cursor: "pointer",
                  fontSize: 13,
                }}
                onMouseEnter={(e) => {
                  e.currentTarget.style.background = "var(--bg-hover)";
                }}
                onMouseLeave={(e) => {
                  e.currentTarget.style.background = "none";
                }}
              >
                Выйти
              </button>
            </div>
          )}
        </div>
      </div>
    </header>
  );
}
