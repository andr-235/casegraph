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
