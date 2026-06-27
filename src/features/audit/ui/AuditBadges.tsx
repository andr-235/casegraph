import { formatAuditAction } from "../model/auditActionLabels";

type BadgeProps = {
  value: string;
};

export function AuditResultBadge({ value }: BadgeProps) {
  return (
    <span className={`audit-badge audit-badge--result-${value}`}>{value}</span>
  );
}

export function AuditSeverityBadge({ value }: BadgeProps) {
  return (
    <span className={`audit-badge audit-badge--severity-${value}`}>
      {value}
    </span>
  );
}

export function AuditActionBadge({ value }: BadgeProps) {
  return (
    <span className="audit-badge audit-badge--action" title={value}>
      {formatAuditAction(value)}
    </span>
  );
}
