import { FormEvent, useCallback, useEffect, useState } from "react";
import {
  updateCase,
  updateCaseStatus,
} from "../../features/cases/api/casesApi";
import type {
  CaseDto,
  EditableCaseStatus,
} from "../../features/cases/model/caseTypes";
import {
  editableCaseStatusOptions,
  isEditableCaseStatus,
} from "../../features/cases/model/caseStatus";
import { getCaseOverview } from "../../features/cases/api/casesApi";
import type { CaseOverviewDto } from "../../features/cases/model/caseTypes";
import type { CaseWorkspaceSection } from "../../shared/types/workspaceTypes";
import { InspectorSection } from "../../shared/ui/inspector/InspectorSection";

type Props = {
  caseItem: CaseDto;
  onCaseUpdated: (caseItem: CaseDto) => void;
  /** Навигация на другой раздел (например, "objects") */
  onNavigateToSection?: (section: CaseWorkspaceSection) => void;
  /** Открыть Inspector для объекта */
  onNavigateToObject?: (objectId: string) => void;
};

export function CaseOverviewPage({
  caseItem,
  onCaseUpdated,
  onNavigateToSection,
  onNavigateToObject,
}: Props) {
  // ================================================================
  // Загрузка overview
  // ================================================================

  const [overview, setOverview] = useState<CaseOverviewDto | null>(null);
  const [overviewLoading, setOverviewLoading] = useState(true);
  const [overviewError, setOverviewError] = useState<string | null>(null);

  const loadOverview = useCallback(async () => {
    setOverviewLoading(true);
    setOverviewError(null);

    try {
      const data = await getCaseOverview({ caseId: caseItem.id });
      setOverview(data);
    } catch (err) {
      setOverviewError(
        err instanceof Error ? err.message : "Ошибка загрузки сводки",
      );
    } finally {
      setOverviewLoading(false);
    }
  }, [caseItem.id]);

  useEffect(() => {
    loadOverview();
  }, [loadOverview]);

  // ================================================================
  // Форма редактирования
  // ================================================================

  const [editing, setEditing] = useState(false);
  const [title, setTitle] = useState(caseItem.title);
  const [subject, setSubject] = useState(caseItem.subject);
  const [description, setDescription] = useState(caseItem.description);
  const [periodStart, setPeriodStart] = useState(caseItem.periodStart ?? "");
  const [periodEnd, setPeriodEnd] = useState(caseItem.periodEnd ?? "");
  const [saving, setSaving] = useState(false);
  const [statusSaving, setStatusSaving] = useState(false);
  const [error, setError] = useState<string | null>(null);

  useEffect(() => {
    setTitle(caseItem.title);
    setSubject(caseItem.subject);
    setDescription(caseItem.description);
    setPeriodStart(caseItem.periodStart ?? "");
    setPeriodEnd(caseItem.periodEnd ?? "");
    setEditing(false);
    setError(null);
  }, [caseItem]);

  // ================================================================
  // Обработчики
  // ================================================================

  const handleCancel = useCallback(() => {
    setTitle(caseItem.title);
    setSubject(caseItem.subject);
    setDescription(caseItem.description);
    setPeriodStart(caseItem.periodStart ?? "");
    setPeriodEnd(caseItem.periodEnd ?? "");
    setEditing(false);
    setError(null);
  }, [caseItem]);

  const handleSubmit = useCallback(
    async (event: FormEvent) => {
      event.preventDefault();
      setError(null);

      try {
        setSaving(true);

        const response = await updateCase({
          caseId: caseItem.id,
          title,
          subject,
          description,
          periodStart: periodStart || null,
          periodEnd: periodEnd || null,
        });

        onCaseUpdated(response.caseItem);
        setEditing(false);
      } catch (err) {
        setError(
          err instanceof Error ? err.message : "Не удалось сохранить дело.",
        );
      } finally {
        setSaving(false);
      }
    },
    [caseItem.id, title, subject, description, periodStart, periodEnd, onCaseUpdated],
  );

  const handleStatusChange = useCallback(
    async (nextStatus: EditableCaseStatus) => {
      if (nextStatus === caseItem.status) return;

      setError(null);

      try {
        setStatusSaving(true);

        const response = await updateCaseStatus({
          caseId: caseItem.id,
          status: nextStatus,
        });

        onCaseUpdated(response.caseItem);
      } catch (err) {
        setError(
          err instanceof Error ? err.message : "Не удалось изменить статус дела.",
        );
      } finally {
        setStatusSaving(false);
      }
    },
    [caseItem.id, caseItem.status, onCaseUpdated],
  );

  const handleKeyObjectClick = useCallback(
    (objectId: string) => {
      onNavigateToSection?.("objects");
      onNavigateToObject?.(objectId);
    },
    [onNavigateToSection, onNavigateToObject],
  );

  // ================================================================
  // Рендер
  // ================================================================

  return (
    <section style={{ display: "flex", flexDirection: "column", gap: "var(--space-5)" }}>
      {/* Секция: статус + редактирование */}
      <div
        style={{
          background: "var(--bg-surface)",
          borderRadius: "var(--radius-md)",
          border: "1px solid var(--border-subtle)",
          padding: "var(--space-5)",
          display: "flex",
          flexDirection: "column",
          gap: "var(--space-4)",
        }}
      >
        {/* Статус */}
        <div style={{ display: "flex", alignItems: "center", justifyContent: "space-between" }}>
          <div style={{ display: "flex", alignItems: "center", gap: "var(--space-3)" }}>
            <span style={{ fontSize: 13, color: "var(--text-muted)" }}>Статус:</span>

            <select
              value={caseItem.status}
              onChange={(event) => {
                const nextStatus = event.target.value;
                if (!isEditableCaseStatus(nextStatus)) {
                  setError("Недопустимый статус дела.");
                  return;
                }
                handleStatusChange(nextStatus);
              }}
              disabled={statusSaving}
              style={{
                padding: "6px 10px",
                borderRadius: "var(--radius-sm)",
                border: "1px solid var(--border-default)",
                background: "var(--bg-elevated)",
                color: "var(--text-primary)",
                fontSize: 13,
                outline: "none",
              }}
            >
              {editableCaseStatusOptions.map((option) => (
                <option key={option.value} value={option.value}>
                  {option.label}
                </option>
              ))}
            </select>

            {statusSaving && (
              <span style={{ fontSize: 12, color: "var(--text-muted)" }}>
                Сохранение...
              </span>
            )}
          </div>

          <button
            type="button"
            onClick={() => setEditing((prev) => !prev)}
            style={{
              padding: "6px 14px",
              borderRadius: "var(--radius-sm)",
              border: "1px solid var(--border-default)",
              background: "none",
              color: "var(--accent)",
              cursor: "pointer",
              fontSize: 12,
            }}
          >
            {editing ? "Отменить" : "Редактировать"}
          </button>
        </div>

        {/* Форма редактирования (раскрывающаяся) */}
        {editing && (
          <form
            onSubmit={handleSubmit}
            style={{
              display: "flex",
              flexDirection: "column",
              gap: "var(--space-3)",
              borderTop: "1px solid var(--border-subtle)",
              paddingTop: "var(--space-4)",
            }}
          >
            <div style={{ display: "grid", gridTemplateColumns: "1fr 1fr", gap: "var(--space-3)" }}>
              <label style={{ display: "flex", flexDirection: "column", gap: 4, fontSize: 12, color: "var(--text-secondary)" }}>
                Название дела
                <input
                  value={title}
                  onChange={(e) => setTitle(e.target.value)}
                  minLength={3}
                  required
                  disabled={saving}
                  style={{
                    padding: "6px 10px",
                    borderRadius: "var(--radius-sm)",
                    border: "1px solid var(--border-default)",
                    background: "var(--bg-elevated)",
                    color: "var(--text-primary)",
                    fontSize: 13,
                    outline: "none",
                  }}
                />
              </label>

              <label style={{ display: "flex", flexDirection: "column", gap: 4, fontSize: 12, color: "var(--text-secondary)" }}>
                Объект анализа
                <input
                  value={subject}
                  onChange={(e) => setSubject(e.target.value)}
                  minLength={2}
                  required
                  disabled={saving}
                  style={{
                    padding: "6px 10px",
                    borderRadius: "var(--radius-sm)",
                    border: "1px solid var(--border-default)",
                    background: "var(--bg-elevated)",
                    color: "var(--text-primary)",
                    fontSize: 13,
                    outline: "none",
                  }}
                />
              </label>
            </div>

            <label style={{ display: "flex", flexDirection: "column", gap: 4, fontSize: 12, color: "var(--text-secondary)" }}>
              Описание
              <textarea
                value={description}
                onChange={(e) => setDescription(e.target.value)}
                rows={4}
                disabled={saving}
                style={{
                  padding: "6px 10px",
                  borderRadius: "var(--radius-sm)",
                  border: "1px solid var(--border-default)",
                  background: "var(--bg-elevated)",
                  color: "var(--text-primary)",
                  fontSize: 13,
                  outline: "none",
                  resize: "vertical",
                }}
              />
            </label>

            <div style={{ display: "flex", gap: "var(--space-3)" }}>
              <label style={{ display: "flex", flexDirection: "column", gap: 4, fontSize: 12, color: "var(--text-secondary)" }}>
                Период с
                <input
                  type="date"
                  value={periodStart}
                  onChange={(e) => setPeriodStart(e.target.value)}
                  disabled={saving}
                  style={{
                    padding: "6px 10px",
                    borderRadius: "var(--radius-sm)",
                    border: "1px solid var(--border-default)",
                    background: "var(--bg-elevated)",
                    color: "var(--text-primary)",
                    fontSize: 13,
                    outline: "none",
                  }}
                />
              </label>

              <label style={{ display: "flex", flexDirection: "column", gap: 4, fontSize: 12, color: "var(--text-secondary)" }}>
                Период по
                <input
                  type="date"
                  value={periodEnd}
                  onChange={(e) => setPeriodEnd(e.target.value)}
                  disabled={saving}
                  style={{
                    padding: "6px 10px",
                    borderRadius: "var(--radius-sm)",
                    border: "1px solid var(--border-default)",
                    background: "var(--bg-elevated)",
                    color: "var(--text-primary)",
                    fontSize: 13,
                    outline: "none",
                  }}
                />
              </label>
            </div>

            {error && (
              <div style={{ color: "var(--danger)", fontSize: 13 }}>{error}</div>
            )}

            <div style={{ display: "flex", gap: "var(--space-2)" }}>
              <button
                type="submit"
                disabled={saving}
                style={{
                  padding: "8px 16px",
                  borderRadius: "var(--radius-sm)",
                  border: "none",
                  background: "var(--accent)",
                  color: "#fff",
                  cursor: "pointer",
                  fontSize: 13,
                  fontWeight: 600,
                }}
              >
                {saving ? "Сохранение..." : "Сохранить"}
              </button>

              <button
                type="button"
                onClick={handleCancel}
                disabled={saving}
                style={{
                  padding: "8px 16px",
                  borderRadius: "var(--radius-sm)",
                  border: "1px solid var(--border-default)",
                  background: "none",
                  color: "var(--text-secondary)",
                  cursor: "pointer",
                  fontSize: 13,
                }}
              >
                Отмена
              </button>
            </div>
          </form>
        )}
      </div>

      {/* KPI-карточки */}
      {overviewLoading ? (
        <div
          style={{
            display: "grid",
            gridTemplateColumns: "repeat(4, 1fr)",
            gap: "var(--space-4)",
          }}
        >
          {[1, 2, 3, 4].map((i) => (
            <div
              key={i}
              style={{
                height: 100,
                borderRadius: "var(--radius-md)",
                background: "var(--bg-surface)",
                border: "1px solid var(--border-subtle)",
              }}
            />
          ))}
        </div>
      ) : overview ? (
        <div
          style={{
            display: "grid",
            gridTemplateColumns: "repeat(4, 1fr)",
            gap: "var(--space-4)",
          }}
        >
          <KpiCard
            icon="◈"
            value={overview.summary.objectCount}
            label="Объекты"
          />
          <KpiCard
            icon="⌕"
            value={overview.summary.materialCount}
            label="Материалы"
          />
          <KpiCard
            icon="⇄"
            value={overview.summary.relationCount}
            label="Связи"
          />
          <KpiCard
            icon="◷"
            value={overview.summary.eventCount}
            label="События"
          />
        </div>
      ) : overviewError ? (
        <div style={{ color: "var(--danger)", fontSize: 13 }}>
          {overviewError}
        </div>
      ) : null}

      {/* Ключевые объекты */}
      {overview && overview.keyObjects.length > 0 && (
        <InspectorSection title="Ключевые объекты">
          <div style={{ display: "flex", flexWrap: "wrap", gap: "var(--space-2)" }}>
            {overview.keyObjects.map((obj) => (
              <button
                key={obj.id}
                type="button"
                onClick={() => handleKeyObjectClick(obj.id)}
                style={{
                  display: "flex",
                  alignItems: "center",
                  gap: "var(--space-1)",
                  padding: "4px 10px",
                  borderRadius: "var(--radius-sm)",
                  border: "1px solid var(--border-subtle)",
                  background: "var(--bg-elevated)",
                  color: "var(--text-primary)",
                  cursor: "pointer",
                  fontSize: 12,
                }}
              >
                <span style={{ color: "var(--warning)" }}>★</span>
                <span style={{ fontFamily: "var(--font-mono)", color: "var(--text-muted)" }}>
                  {obj.objectCode}
                </span>
                <span>{obj.title}</span>
              </button>
            ))}
          </div>
        </InspectorSection>
      )}

      {/* Последняя активность */}
      {overview && overview.recentActivity.length > 0 && (
        <InspectorSection title="Последняя активность">
          <div style={{ display: "flex", flexDirection: "column", gap: 2 }}>
            {overview.recentActivity.map((activity, idx) => (
              <div
                key={`${activity.entityId}-${idx}`}
                style={{
                  display: "flex",
                  alignItems: "center",
                  gap: "var(--space-2)",
                  padding: "6px 8px",
                  borderRadius: "var(--radius-sm)",
                  fontSize: 12,
                  color: "var(--text-secondary)",
                }}
              >
                <span style={{ fontFamily: "var(--font-mono)", color: "var(--text-muted)", minWidth: 60 }}>
                  {activity.code}
                </span>
                <span style={{ flex: 1, minWidth: 0, overflow: "hidden", textOverflow: "ellipsis", whiteSpace: "nowrap" }}>
                  {activity.title}
                </span>
                <span style={{ color: "var(--text-muted)", whiteSpace: "nowrap" }}>
                  {activity.timestamp}
                </span>
              </div>
            ))}
          </div>
        </InspectorSection>
      )}

      {/* Поля дела (read-only) */}
      <div
        style={{
          display: "grid",
          gridTemplateColumns: "140px 1fr",
          gap: "var(--space-2)",
          fontSize: 13,
          color: "var(--text-secondary)",
        }}
      >
        <span style={{ color: "var(--text-muted)" }}>Код</span>
        <span style={{ fontFamily: "var(--font-mono)" }}>{caseItem.caseCode}</span>

        <span style={{ color: "var(--text-muted)" }}>Создано</span>
        <span>{caseItem.createdAt}</span>

        <span style={{ color: "var(--text-muted)" }}>Обновлено</span>
        <span>{caseItem.updatedAt}</span>
      </div>
    </section>
  );
}

// ===================================================================
// KpiCard
// ===================================================================

function KpiCard({
  icon,
  value,
  label,
}: {
  icon: string;
  value: number;
  label: string;
}) {
  return (
    <div
      style={{
        minWidth: 140,
        padding: "var(--space-5)",
        background: "var(--bg-surface)",
        borderRadius: "var(--radius-md)",
        border: "1px solid var(--border-subtle)",
        display: "flex",
        flexDirection: "column",
        gap: "var(--space-1)",
      }}
    >
      <span style={{ fontSize: 20, color: "var(--text-muted)" }}>{icon}</span>
      <span
        style={{
          fontSize: 28,
          fontWeight: 700,
          color: "var(--text-primary)",
          fontFamily: "var(--font-mono)",
          lineHeight: 1.1,
        }}
      >
        {value}
      </span>
      <span style={{ fontSize: 13, color: "var(--text-secondary)" }}>
        {label}
      </span>
    </div>
  );
}
