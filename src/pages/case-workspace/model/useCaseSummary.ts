import { useCallback, useEffect, useState } from "react";
import { getCaseSummary } from "../../../features/cases/api/casesApi";
import type { CaseSummaryDto } from "../../../features/cases/model/caseTypes";

/**
 * Хук для загрузки сводки по делу (счётчики для боковой панели).
 * Автоматически загружается при монтировании / смене caseId.
 * Возвращает `refetch` для ручного обновления после структурных мутаций.
 */
export function useCaseSummary(caseId: string) {
  const [summary, setSummary] = useState<CaseSummaryDto | null>(null);
  const [loading, setLoading] = useState(true);
  const [error, setError] = useState<string | null>(null);

  const fetch = useCallback(async () => {
    setLoading(true);
    setError(null);

    try {
      const data = await getCaseSummary({ caseId });
      setSummary(data);
    } catch (err) {
      const message =
        err instanceof Error ? err.message : "Ошибка загрузки сводки";
      setError(message);
    } finally {
      setLoading(false);
    }
  }, [caseId]);

  useEffect(() => {
    fetch();
  }, [fetch]);

  return { summary, loading, error, refetch: fetch };
}
