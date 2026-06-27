import { AuditJsonBlock } from "./AuditJsonBlock";
import {
  AuditResultBadge,
  AuditSeverityBadge,
} from "./AuditBadges";
import type { AuditLogDetailsDto } from "../model/auditTypes";

type AuditLogDetailsPanelProps = {
  item: AuditLogDetailsDto | null;
  loading: boolean;
  errorMessage: string;
  onClose: () => void;
};

export function AuditLogDetailsPanel({
  item,
  loading,
  errorMessage,
  onClose,
}: AuditLogDetailsPanelProps) {
  if (!item && !loading && !errorMessage) {
    return (
      <aside className="details-panel audit-details-panel">
        <div className="details-panel-empty">
          Выберите запись журнала, чтобы посмотреть детали.
        </div>
      </aside>
    );
  }

  return (
    <aside className="details-panel audit-details-panel">
      <header className="details-panel-header">
        <div>
          <h2>Детали события</h2>
          {item ? <p>{item.action}</p> : null}
        </div>

        <button type="button" onClick={onClose} aria-label="Закрыть">
          ×
        </button>
      </header>

      {loading ? <div className="loading-state">Загрузка деталей...</div> : null}

      {errorMessage ? <div className="error-box">{errorMessage}</div> : null}

      {item && !loading ? (
        <div className="details-panel-body">
          <section className="audit-details-section">
            <h3>Событие</h3>

            <dl className="meta-list">
              <div>
                <dt>ID</dt>
                <dd>
                  <code>{item.id}</code>
                </dd>
              </div>

              <div>
                <dt>Действие</dt>
                <dd>
                  <code>{item.action}</code>
                </dd>
              </div>

              <div>
                <dt>Результат</dt>
                <dd>
                  <AuditResultBadge value={item.result} />
                </dd>
              </div>

              <div>
                <dt>Важность</dt>
                <dd>
                  <AuditSeverityBadge value={item.severity} />
                </dd>
              </div>

              <div>
                <dt>Дата и время</dt>
                <dd>{item.createdAt}</dd>
              </div>
            </dl>
          </section>

          <section className="audit-details-section">
            <h3>Пользователь</h3>

            <dl className="meta-list">
              <div>
                <dt>Имя</dt>
                <dd>{item.username}</dd>
              </div>

              <div>
                <dt>Роль</dt>
                <dd>{item.userRole}</dd>
              </div>

              <div>
                <dt>User ID</dt>
                <dd>
                  {item.userId ? <code>{item.userId}</code> : "\u2014"}
                </dd>
              </div>
            </dl>
          </section>

          <section className="audit-details-section">
            <h3>Связанная сущность</h3>

            <dl className="meta-list">
              <div>
                <dt>Тип</dt>
                <dd>
                  <code>{item.entityType}</code>
                </dd>
              </div>

              <div>
                <dt>Entity ID</dt>
                <dd>
                  {item.entityId ? <code>{item.entityId}</code> : "\u2014"}
                </dd>
              </div>

              <div>
                <dt>Case ID</dt>
                <dd>
                  {item.caseId ? <code>{item.caseId}</code> : "\u2014"}
                </dd>
              </div>

              <div>
                <dt>Версия приложения</dt>
                <dd>{item.appVersion}</dd>
              </div>
            </dl>
          </section>

          <AuditJsonBlock title="Old value" value={item.oldValue} />
          <AuditJsonBlock title="New value" value={item.newValue} />
          <AuditJsonBlock title="Technical details" value={item.technicalDetails} />
        </div>
      ) : null}
    </aside>
  );
}
