import { FormEvent, useState } from "react";
import { createCase } from "../../features/cases/api/casesApi";
import type { CaseDto } from "../../features/cases/model/caseTypes";
import type { AppCommandError } from "../../shared/api/commandResult";

type Props = {
  onCreated: (caseItem: CaseDto) => void;
  onClose: () => void;
};

export function CreateCaseModal({ onCreated, onClose }: Props) {
  const [title, setTitle] = useState("");
  const [subject, setSubject] = useState("");
  const [description, setDescription] = useState("");
  const [periodStart, setPeriodStart] = useState("");
  const [periodEnd, setPeriodEnd] = useState("");
  const [submitting, setSubmitting] = useState(false);
  const [error, setError] = useState<string | null>(null);

  async function handleSubmit(event: FormEvent) {
    event.preventDefault();
    setError(null);

    try {
      setSubmitting(true);

      const response = await createCase({
        title,
        subject,
        description,
        periodStart: periodStart || null,
        periodEnd: periodEnd || null,
      });

      onCreated(response.caseItem);
    } catch (err) {
      const appError = err as AppCommandError;
      setError(appError.message || "Не удалось создать дело.");
    } finally {
      setSubmitting(false);
    }
  }

  return (
    <div
      style={{
        position: "fixed",
        inset: 0,
        background: "rgba(0,0,0,0.25)",
        display: "flex",
        alignItems: "center",
        justifyContent: "center",
      }}
    >
      <form
        onSubmit={handleSubmit}
        style={{
          width: 520,
          background: "white",
          padding: 24,
          border: "1px solid #ddd",
        }}
      >
        <h2>Создать дело</h2>

        <div>
          <label>
            Название дела
            <input
              value={title}
              onChange={(event) => setTitle(event.target.value)}
              minLength={3}
              required
            />
          </label>
        </div>

        <div>
          <label>
            Объект анализа
            <input
              value={subject}
              onChange={(event) => setSubject(event.target.value)}
              minLength={2}
              required
            />
          </label>
        </div>

        <div>
          <label>
            Описание
            <textarea
              value={description}
              onChange={(event) => setDescription(event.target.value)}
              rows={4}
            />
          </label>
        </div>

        <div>
          <label>
            Период с
            <input
              type="date"
              value={periodStart}
              onChange={(event) => setPeriodStart(event.target.value)}
            />
          </label>
        </div>

        <div>
          <label>
            Период по
            <input
              type="date"
              value={periodEnd}
              onChange={(event) => setPeriodEnd(event.target.value)}
            />
          </label>
        </div>

        {error && <p style={{ color: "crimson" }}>{error}</p>}

        <div style={{ display: "flex", gap: 8, marginTop: 16 }}>
          <button type="submit" disabled={submitting}>
            {submitting ? "Создание..." : "Создать"}
          </button>

          <button type="button" onClick={onClose} disabled={submitting}>
            Отмена
          </button>
        </div>
      </form>
    </div>
  );
}