import { FormEvent, useState } from "react";
import { createMaterial } from "../../features/materials/api/materialsApi";
import { pickMaterialFile } from "../../features/materials/api/materialFilePicker";
import type {
  MaterialDto,
  MaterialType,
} from "../../features/materials/model/materialTypes";

type Props = {
  caseId: string;
  onCreated: (material: MaterialDto) => void;
  onClose: () => void;
};

const materialTypeOptions: Array<{
  value: MaterialType;
  label: string;
}> = [
  { value: "image", label: "Изображение" },
  { value: "pdf", label: "PDF" },
  { value: "document", label: "Документ" },
  { value: "spreadsheet", label: "Таблица" },
  { value: "text", label: "Текст" },
  { value: "html", label: "HTML" },
  { value: "other", label: "Другое" },
];

export function AddMaterialModal({ caseId, onCreated, onClose }: Props) {
  const [title, setTitle] = useState("");
  const [materialType, setMaterialType] = useState<MaterialType>("document");
  const [sourceName, setSourceName] = useState("");
  const [description, setDescription] = useState("");
  const [capturedAt, setCapturedAt] = useState("");
  const [includeInReport, setIncludeInReport] = useState(true);
  const [sourceFilePath, setSourceFilePath] = useState("");
  const [submitting, setSubmitting] = useState(false);
  const [error, setError] = useState<string | null>(null);

  async function handlePickFile() {
    setError(null);

    try {
      const selectedPath = await pickMaterialFile();

      if (!selectedPath) {
        return;
      }

      setSourceFilePath(selectedPath);

      if (!title.trim()) {
        const fileName = selectedPath.split(/[\\/]/).pop();

        if (fileName) {
          setTitle(fileName);
        }
      }
    } catch (err) {
      setError(
        err instanceof Error ? err.message : "Не удалось выбрать файл."
      );
    }
  }

  async function handleSubmit(event: FormEvent) {
    event.preventDefault();
    setError(null);

    try {
      setSubmitting(true);

      const response = await createMaterial({
        caseId,
        title,
        materialType,
        sourceName,
        description,
        capturedAt: capturedAt || null,
        includeInReport,
        sourceFilePath: sourceFilePath.trim() || null,
      });

      onCreated(response.material);
    } catch (err) {
      setError(
        err instanceof Error ? err.message : "Не удалось добавить материал."
      );
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
          width: 560,
          background: "white",
          padding: 24,
          border: "1px solid #ddd",
        }}
      >
        <h2>Добавить материал</h2>

        <p>
          Выберите локальный файл. CaseGraph скопирует его во внутреннее хранилище
          и рассчитает SHA-256.
        </p>

        <div style={{ marginBottom: 12 }}>
          <label>
            Название материала
            <input
              value={title}
              onChange={(event) => setTitle(event.target.value)}
              minLength={2}
              required
              disabled={submitting}
              style={{ display: "block", width: "100%" }}
            />
          </label>
        </div>

        <div style={{ marginBottom: 12 }}>
          <label>
            Тип материала
            <select
              value={materialType}
              onChange={(event) =>
                setMaterialType(event.target.value as MaterialType)
              }
              disabled={submitting}
              style={{ display: "block", width: "100%" }}
            >
              {materialTypeOptions.map((option) => (
                <option key={option.value} value={option.value}>
                  {option.label}
                </option>
              ))}
            </select>
          </label>
        </div>

        <div style={{ marginBottom: 12 }}>
          <label>
            Источник
            <input
              value={sourceName}
              onChange={(event) => setSourceName(event.target.value)}
              disabled={submitting}
              style={{ display: "block", width: "100%" }}
            />
          </label>
        </div>

        <div style={{ marginBottom: 12 }}>
          <label>
            Путь к локальному файлу
            <div style={{ display: "flex", gap: 8 }}>
              <input
                value={sourceFilePath}
                onChange={(event) => setSourceFilePath(event.target.value)}
                disabled={submitting}
                placeholder="C:\Users\...\Documents\file.pdf"
                style={{ flex: 1 }}
              />

              <button
                type="button"
                onClick={handlePickFile}
                disabled={submitting}
              >
                Выбрать файл
              </button>
            </div>
          </label>
        </div>

        <div style={{ marginBottom: 12 }}>
          <label>
            Дата фиксации
            <input
              type="date"
              value={capturedAt}
              onChange={(event) => setCapturedAt(event.target.value)}
              disabled={submitting}
              style={{ display: "block" }}
            />
          </label>
        </div>

        <div style={{ marginBottom: 12 }}>
          <label>
            Описание
            <textarea
              value={description}
              onChange={(event) => setDescription(event.target.value)}
              rows={4}
              disabled={submitting}
              style={{ display: "block", width: "100%" }}
            />
          </label>
        </div>

        <div style={{ marginBottom: 12 }}>
          <label>
            <input
              type="checkbox"
              checked={includeInReport}
              onChange={(event) => setIncludeInReport(event.target.checked)}
              disabled={submitting}
            />{" "}
            Включать в справку
          </label>
        </div>

        {error && <p style={{ color: "crimson" }}>{error}</p>}

        <div style={{ display: "flex", gap: 8 }}>
          <button type="submit" disabled={submitting}>
            {submitting ? "Добавление..." : "Добавить"}
          </button>

          <button type="button" onClick={onClose} disabled={submitting}>
            Отмена
          </button>
        </div>
      </form>
    </div>
  );
}