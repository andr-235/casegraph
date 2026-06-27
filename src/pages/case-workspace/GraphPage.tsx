import { useEffect, useMemo, useState } from "react";

import { getGraphData } from "../../features/graph/api/graphApi";
import type { GraphEdge, GraphNode } from "../../features/graph/model/graphTypes";
import { getObjectTypeLabel } from "../../features/objects/model/objectOptions";
import { GraphVisualPreview } from "../../features/graph/ui/GraphVisualPreview";
import {
  getRelationConfidenceLabel,
  getRelationTypeLabel,
} from "../../features/relations/model/relationConstants";

type GraphPageProps = {
  caseId: string;
};

export function GraphPage({ caseId }: GraphPageProps) {
  const [nodes, setNodes] = useState<GraphNode[]>([]);
  const [edges, setEdges] = useState<GraphEdge[]>([]);
  const [isLoading, setIsLoading] = useState(false);
  const [errorText, setErrorText] = useState<string | null>(null);

  async function loadGraphData() {
    setIsLoading(true);
    setErrorText(null);

    try {
      const response = await getGraphData({ caseId });

      setNodes(response.nodes);
      setEdges(response.edges);
    } catch (error) {
      setErrorText(error instanceof Error ? error.message : "Не удалось загрузить граф.");
    } finally {
      setIsLoading(false);
    }
  }

  useEffect(() => {
    void loadGraphData();
  }, [caseId]);

  const nodeById = useMemo(() => {
    return new Map(nodes.map((node) => [node.id, node]));
  }, [nodes]);

  return (
    <section className="case-section">
      <div className="section-header">
        <div>
          <h1>Граф связей</h1>
          <p>
            Базовое представление активных объектов и связей дела. Визуальный
            canvas добавим отдельным срезом.
          </p>
        </div>

        <button type="button" onClick={loadGraphData} disabled={isLoading}>
          Обновить
        </button>
      </div>

      {errorText ? <div className="error-state">{errorText}</div> : null}

      {isLoading ? <div className="loading-state">Загрузка графа…</div> : null}

      {!isLoading && !errorText ? (
        <>
          <div className="graph-summary">
            <div className="summary-card">
              <span>Узлы</span>
              <strong>{nodes.length}</strong>
            </div>

            <div className="summary-card">
              <span>Связи</span>
              <strong>{edges.length}</strong>
            </div>

            <div className="summary-card">
              <span>Ключевые объекты</span>
              <strong>{nodes.filter((node) => node.isKey).length}</strong>
            </div>
          </div>

          <GraphVisualPreview nodes={nodes} edges={edges} />

          {nodes.length === 0 ? (
            <div className="empty-state">
              В деле пока нет объектов. Создай объекты в разделе «Объекты»,
              затем добавь связи.
            </div>
          ) : (
            <div className="graph-foundation-layout">
              <section className="graph-panel">
                <h2>Узлы</h2>

                <table>
                  <thead>
                    <tr>
                      <th>Код</th>
                      <th>Тип</th>
                      <th>Название</th>
                      <th>Ключевой</th>
                    </tr>
                  </thead>

                  <tbody>
                    {nodes.map((node) => (
                      <tr key={node.id}>
                        <td>{node.objectCode}</td>
                        <td>{getObjectTypeLabel(node.objectType)}</td>
                        <td>
                          <strong>{node.title}</strong>
                          {node.value ? <div>{node.value}</div> : null}
                        </td>
                        <td>{node.isKey ? "Да" : "Нет"}</td>
                      </tr>
                    ))}
                  </tbody>
                </table>
              </section>

              <section className="graph-panel">
                <h2>Связи</h2>

                {edges.length === 0 ? (
                  <div className="empty-state">
                    Связи ещё не созданы. Создай связь в разделе «Связи».
                  </div>
                ) : (
                  <table>
                    <thead>
                      <tr>
                        <th>Код</th>
                        <th>Связь</th>
                        <th>Тип</th>
                        <th>Достоверность</th>
                      </tr>
                    </thead>

                    <tbody>
                      {edges.map((edge) => {
                        const sourceNode = nodeById.get(edge.sourceObjectId);
                        const targetNode = nodeById.get(edge.targetObjectId);

                        return (
                          <tr key={edge.id}>
                            <td>{edge.relationCode}</td>
                            <td>
                              <strong>
                                {sourceNode?.objectCode ?? "?"} →{" "}
                                {targetNode?.objectCode ?? "?"}
                              </strong>
                              <div>
                                {sourceNode?.title ?? "Неизвестный объект"} →{" "}
                                {targetNode?.title ?? "Неизвестный объект"}
                              </div>
                            </td>
                            <td>{getRelationTypeLabel(edge.relationType)}</td>
                            <td>
                              {getRelationConfidenceLabel(edge.confidenceLevel)}
                            </td>
                          </tr>
                        );
                      })}
                    </tbody>
                  </table>
                )}
              </section>
            </div>
          )}
        </>
      ) : null}
    </section>
  );
}
