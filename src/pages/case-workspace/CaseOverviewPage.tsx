import { FormEvent, useEffect, useState } from "react";
import { updateCase } from "../../features/cases/api/casesApi";
import type { CaseDto } from "../../features/cases/model/caseTypes";

type Props = {
  caseItem: CaseDto;
  onCaseUpdated: (caseItem: CaseDto) => void;
};

function formatOptionalDate(value: string | null) {
  return value && value.trim().length > 0 ? value : "Не указано";
}

export function CaseOverviewPage({ caseItem, onCaseUpdated }: Props) {
  const [editing, setEditing] = useState(false);
  const [title, setTitle] = useState(caseItem.title);
  const [subject, setSubject] = useState(caseItem.subject);
  const [description, setDescription] = useState(caseItem.description);
  const [periodStart, setPeriodStart] = useState(caseItem.periodStart ?? "");
  const [periodEnd, setPeriodEnd] = useState(caseItem.periodEnd ?? "");
  const [saving, setSaving] = useState(false);
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

  function handleCancel() {
    setTitle(caseItem.title);
    setSubject(caseItem.subject);
    setDescription(caseItem.description);
    setPeriodStart(caseItem.periodStart ?? "");
    setPeriodEnd(caseItem.periodEnd ?? "");
    setEditing(false);
    setError(null);
  }

  async function handleSubmit(event: FormEvent) {
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
      const message =
        err instanceof Error ? err.message : "Не удалось сохранить дело.";
      setError(message);
    } finally {
      setSaving(false);
    }
  }

  if (!editing) {
    return (
      <section>
        <header style={{ display: "flex", justifyContent: "space-between" }}>
          <div>
            <h2>
              {caseItem.caseCode} · {caseItem.title}
            </h2>

            <p>
              Статус: <strong>{caseItem.status}</strong>
            </p>
          </div>

          <button type="button" onClick={() => setEditing(true)}>
            Редактировать
          </button>
        </header>

        <div
          style={{
            display: "grid",
            gridTemplateColumns: "180px 1fr",
            gap: 12,
            maxWidth: 760,
            marginTop: 24,
          }}
        >
          <strong>Объект анализа</strong>
          <span>{caseItem.subject}</span>

          <strong>Описание</strong>
          <span>{caseItem.description || "Описание не заполнено."}</span>

          <strong>Период с</strong>
          <span>{formatOptionalDate(caseItem.periodStart)}</span>

          <strong>Период по</strong>
          <span>{formatOptionalDate(caseItem.periodEnd)}</span>

          <strong>Создано</strong>
          <span>{caseItem.createdAt}</span>

          <strong>Обновлено</strong>
          <span>{caseItem.updatedAt}</span>
        </div>

        <hr style={{ margin: "24px 0" }} />

        <section>
          <h3>Следующие разделы</h3>

          <p>
            Материалы, объекты, связи, граф, хронология и справка будут
            подключены следующими vertical slices.
          </p>
        </section>
      </section>
    );
  }

  return (
    <section>
      <header>
        <h2>
          Редактирование: {caseItem.caseCode}
        </h2>
      </header>

      <form onSubmit={handleSubmit} style={{ maxWidth: 760 }}>
        <div style={{ marginBottom: 12 }}>
          <label>
            Название дела
            <input
              value={title}
              onChange={(event) => setTitle(event.target.value)}
              minLength={3}
              required
              disabled={saving}
              style={{ display: "block", width: "100%" }}
            />
          </label>
        </div>

        <div style={{ marginBottom: 12 }}>
          <label>
            Объект анализа
            <input
              value={subject}
              onChange={(event) => setSubject(event.target.value)}
              minLength={2}
              required
              disabled={saving}
              style={{ display: "block", width: "100%" }}
            />
          </label>
        </div>

        <div style={{ marginBottom: 12 }}>
          <label>
            Описание
            <textarea
              value={description}
              onChange={(event) => setDescription(event.target.value)}
              rows={5}
              disabled={saving}
              style={{ display: "block", width: "100%" }}
            />
          </label>
        </div>

        <div style={{ display: "flex", gap: 12, marginBottom: 12 }}>
          <label>
            Период с
            <input
              type="date"
              value={periodStart}
              onChange={(event) => setPeriodStart(event.target.value)}
              disabled={saving}
            />
          </label>

          <label>
            Период по
            <input
              type="date"
              value={periodEnd}
              onChange={(event) => setPeriodEnd(event.target.value)}
              disabled={saving}
            />
          </label>
        </div>

        {error && <p style={{ color: "crimson" }}>{error}</p>}

        <div style={{ display: "flex", gap: 8 }}>
          <button type="submit" disabled={saving}>
            {saving ? "Сохранение..." : "Сохранить"}
          </button>

          <button type="button" onClick={handleCancel} disabled={saving}>
            Отмена
          </button>
        </div>
      </form>
    </section>
  );
}