import { useCallback, useState } from "react";
import type { CaseInspectorTarget, InspectorState } from "../../../shared/types/workspaceTypes";

/**
 * Хук управления правой панелью Inspector.
 *
 * - `target` — текущая сущность или null (закрыт)
 * - `revision` — счётчик изменений для refetch контента
 * - `open(type, id)` — открыть инспектор для сущности
 * - `close()` — закрыть инспектор
 * - `invalidate()` — инкремент revision для refetch
 */
export function useCaseInspector() {
  const [state, setState] = useState<InspectorState>({
    target: null,
    revision: 0,
  });

  const open = useCallback(
    (type: CaseInspectorTarget["type"], id: string) => {
      log.debug(`Inspector open: ${type} ${id}`);

      setState({
        target: { type, id } as CaseInspectorTarget,
        revision: 0,
      });
    },
    []
  );

  const close = useCallback(() => {
    log.debug("Inspector close");

    setState({ target: null, revision: 0 });
  }, []);

  const invalidate = useCallback(() => {
    setState((prev) => {
      const nextRev = prev.revision + 1;
      log.debug(`Inspector invalidate: revision ${prev.revision} → ${nextRev}`);
      return { ...prev, revision: nextRev };
    });
  }, []);

  return {
    target: state.target,
    revision: state.revision,
    open,
    close,
    invalidate,
  };
}

/** Простой логгер с префиксом */
const log = {
  debug: (msg: string) => console.debug(`[useCaseInspector] ${msg}`),
  info: (msg: string) => console.info(`[useCaseInspector] ${msg}`),
};
