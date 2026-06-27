import type { AuditJsonValue } from "../model/auditTypes";

type AuditJsonBlockProps = {
  title: string;
  value?: AuditJsonValue;
};

function formatAuditJson(value?: AuditJsonValue) {
  if (value === undefined || value === null) {
    return "\u2014";
  }

  if (typeof value === "string") {
    return value;
  }

  return JSON.stringify(value, null, 2);
}

export function AuditJsonBlock({ title, value }: AuditJsonBlockProps) {
  return (
    <section className="audit-json-block">
      <h3>{title}</h3>
      <pre>{formatAuditJson(value)}</pre>
    </section>
  );
}
