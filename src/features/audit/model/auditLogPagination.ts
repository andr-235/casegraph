export function getAuditLogTotalPages(total: number, pageSize: number) {
  if (pageSize <= 0) {
    return 1;
  }
  return Math.max(1, Math.ceil(total / pageSize));
}

export function clampAuditLogPage(
  page: number,
  total: number,
  pageSize: number,
) {
  const totalPages = getAuditLogTotalPages(total, pageSize);
  if (page < 1) return 1;
  if (page > totalPages) return totalPages;
  return page;
}
