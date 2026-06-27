import type { AuditLogDto } from "../model/auditTypes";
import {
  AuditActionBadge,
  AuditResultBadge,
  AuditSeverityBadge,
} from "./AuditBadges";

type AuditLogTableProps = {
  items: AuditLogDto[];
  selectedAuditLogId: string | null;
  loading: boolean;
  onSelect: (auditLogId: string) => void;
};

export function AuditLogTable({
  items,
  selectedAuditLogId,
  loading,
  onSelect,
}: AuditLogTableProps) {
  if (loading) {
    return <div className="loading-state">Загрузка журнала...</div>;
  }

  if (items.length === 0) {
    return <div className="empty-state">Записи журнала не найдены.</div>;
  }

  return (
    <div className="table-wrapper">
      <table className="data-table audit-log-table">
        <thead>
          <tr>
            <th>Дата</th>
            <th>Пользователь</th>
            <th>Действие</th>
            <th>Сущность</th>
            <th>Результат</th>
            <th>Важность</th>
          </tr>
        </thead>

        <tbody>
          {items.map((item) => (
            <tr
              key={item.id}
              className={
                selectedAuditLogId === item.id ? "selected-row" : undefined
              }
              onClick={() => onSelect(item.id)}
            >
              <td>{item.createdAt}</td>
              <td>
                <div>{item.username}</div>
                <small>{item.userRole}</small>
              </td>
              <td>
                <AuditActionBadge value={item.action} />
              </td>
              <td>
                <div>
                  <code>{item.entityType}</code>
                </div>
                {item.entityId ? <small>{item.entityId}</small> : null}
              </td>
              <td>
                <AuditResultBadge value={item.result} />
              </td>
              <td>
                <AuditSeverityBadge value={item.severity} />
              </td>
            </tr>
          ))}
        </tbody>
      </table>
    </div>
  );
}
