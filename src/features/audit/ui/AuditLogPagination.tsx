import { getAuditLogTotalPages } from "../model/auditLogPagination";

type AuditLogPaginationProps = {
  page: number;
  pageSize: number;
  total: number;
  loading: boolean;
  onPageChange: (page: number) => void;
};

export function AuditLogPagination({
  page,
  pageSize,
  total,
  loading,
  onPageChange,
}: AuditLogPaginationProps) {
  const totalPages = getAuditLogTotalPages(total, pageSize);

  return (
    <footer className="pagination">
      <span>
        Страница {page} из {totalPages} · всего {total}
      </span>

      <div className="pagination-actions">
        <button
          type="button"
          disabled={loading || page <= 1}
          onClick={() => onPageChange(page - 1)}
        >
          Назад
        </button>

        <button
          type="button"
          disabled={loading || page >= totalPages}
          onClick={() => onPageChange(page + 1)}
        >
          Вперёд
        </button>
      </div>
    </footer>
  );
}
