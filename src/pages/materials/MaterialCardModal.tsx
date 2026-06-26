import { FormEvent, useEffect, useState } from "react";
import {
  deleteMaterial,
  updateMaterial,
} from "../../features/materials/api/materialsApi";
import type {
  MaterialDto,
  MaterialType,
} from "../../features/materials/model/materialTypes";

type Props = {
  material: MaterialDto;
  onUpdated: (material: MaterialDto) => void;
  onDeleted: (materialId: string) => void;
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

function getIntegrityStatusLabel(status: string) {
  const labels: Record<string, string> = {
    not_checked: "Не проверено",
    ok: "OK",
    mismatch: "Несовпадение",
    missing: "Файл отсутствует",
    read_error: "Ошибка чтения",
  };

  return labels[status] ?? status;
}

function formatFileSize(value: number | null) {
  if (value === null) {
    return "—";
  }

  if (value < 1024) {
    return `${value} Б`;
  }

  if (value < 1024 * 1024) {
    return `${(value / 1024).toFixed(1)} КБ`;
  }

  return `${(value / 1024 / 1024).toFixed(1)} МБ`;
}

function readOnlyValue(value: string | number | null) {
  if (value === null || value === "") {
    return "—";
  }

  return String(value);
}

export function MaterialCardModal({
  material,
  onUpdated,
  onDeleted,
  onClose,
}: Props) {
  const [title, setTitle] = useState(material.title);
  const [materialType, setMaterialType] =
    useState<MaterialType>(material.materialType);
  const [sourceName, setSourceName] = useState(material.sourceName);
  const [description, setDescription] = useState(material.description);
  const [capturedAt, setCapturedAt] = useState(material.capturedAt ?? "");
  const [includeInReport, setIncludeInReport] = useState(
    material.includeInReport
  );
  const [saving, setSaving] = useState(false);
  const [error, setError] = useState<string | null>(null);

  useEffect(() => {
    setTitle(material.title);
    setMaterialType(material.materialType);
    setSourceName(material.sourceName);
    setDescription(material.description);
    setCapturedAt(material.capturedAt ?? "");
    setIncludeInReport(material.includeInReport);
    setError(null);
  }, [material]);

  async function handleDelete() {
    const confirmed = window.confirm(
      "Удалить материал из дела? Файл во внутреннем хранилище не будет физически удалён."
    );

    if (!confirmed) {
      return;
    }

    setError(null);

    try {
      setSaving(true);

      const response = await deleteMaterial({
        caseId: material.caseId,
        materialId: material.id,
      });

      onDeleted(response.materialId);
    } catch (err) {
      setError(
        err instanceof Error ? err.message : "Не удалось удалить материал."
      );
    } finally {
      setSaving(false);
    }
  }

  async function handleSubmit(event: FormEvent) {
    event.preventDefault();
    setError(null);

    try {
      setSaving(true);

      const response = await updateMaterial({
        caseId: material.caseId,
        materialId: material.id,
        title,
        materialType,
        sourceName,
        description,
        capturedAt: capturedAt || null,
        includeInReport,
      });

      onUpdated(response.material);
    } catch (err) {
      setError(
        err instanceof Error ? err.message : "Не удалось сохранить материал."
      );
    } finally {
      setSaving(false);
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
        zIndex: 20,
      }}
    >
      <form
        onSubmit={handleSubmit}
        style={{
          width: 760,
          maxHeight: "90vh",
          overflow: "auto",
          background: "white",
          padding: 24,
          border: "1px solid #ddd",
        }}
      >
        <header style={{ display: "flex", justifyContent: "space-between" }}>
          <div>
            <h2>
              {material.materialCode} · Карточка материала
            </h2>

            <p>
              Целостность:{" "}
              <strong>{getIntegrityStatusLabel(material.integrityStatus)}</strong>
            </p>
          </div>

          <button type="button" onClick={onClose} disabled={saving}>
            Закрыть
          </button>
        </header>

        <hr />

        <section
          style={{
            display: "grid",
            gridTemplateColumns: "180px 1fr",
            gap: 8,
            marginBottom: 20,
          }}
        >
          <strong>Имя файла</strong>
          <span>{readOnlyValue(material.originalFileName)}</span>

          <strong>Размер</strong>
          <span>{formatFileSize(material.fileSize)}</span>

          <strong>MIME</strong>
          <span>{readOnlyValue(material.mimeType)}</span>

          <strong>Исходный путь</strong>
          <code>{readOnlyValue(material.originalPath)}</code>

          <strong>Внутренний путь</strong>
          <code>{readOnlyValue(material.storedFilePath)}</code>

          <strong>SHA-256</strong>
          <code style={{ overflowWrap: "anywhere" }}>
            {readOnlyValue(material.sha256)}
          </code>

          <strong>Создано</strong>
          <span>{material.createdAt}</span>

          <strong>Обновлено</strong>
          <span>{material.updatedAt}</span>
        </section>

        <hr />

        <div style={{ marginBottom: 12 }}>
          <label>
            Название материала
            <input
              value={title}
              onChange={(event) => setTitle(event.target.value)}
              minLength={2}
              required
              disabled={saving}
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
              disabled={saving}
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
              disabled={saving}
              style={{ display: "block", width: "100%" }}
            />
          </label>
        </div>

        <div style={{ marginBottom: 12 }}>
          <label>
            Дата фиксации
            <input
              type="date"
              value={capturedAt}
              onChange={(event) => setCapturedAt(event.target.value)}
              disabled={saving}
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
              rows={5}
              disabled={saving}
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
              disabled={saving}
            />{" "}
            Включать в справку
          </label>
        </div>

        {error && <p style={{ color: "crimson" }}>{error}</p>}

        <div style={{ display: "flex", gap: 8 }}>
          <button type="submit" disabled={saving}>
            {saving ? "Сохранение..." : "Сохранить"}
          </button>

          <button type="button" onClick={onClose} disabled={saving}>
            Отмена
          </button>

          <button
            type="button"
            onClick={handleDelete}
            disabled={saving}
            style={{ marginLeft: "auto", color: "crimson" }}
          >
            Удалить
          </button>
        </div>
      </form>
    </div>
  );
}
