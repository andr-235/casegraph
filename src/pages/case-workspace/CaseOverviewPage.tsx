import type { CaseDto } from "../../features/cases/model/caseTypes";

type Props = {
  caseItem: CaseDto;
};

function formatOptionalDate(value: string | null) {
  return value && value.trim().length > 0 ? value : "Не указано";
}

export function CaseOverviewPage({ caseItem }: Props) {
  return (
    <section>
      <header>
        <h2>
          {caseItem.caseCode} · {caseItem.title}
        </h2>

        <p>
          Статус: <strong>{caseItem.status}</strong>
        </p>
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
          Сейчас рабочая область дела создана как shell. Реальные модули
          материалов, объектов, связей, графа, хронологии и справки подключим
          следующими vertical slices.
        </p>
      </section>
    </section>
  );
}