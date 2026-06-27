import { useCallback, useEffect, useMemo, useState } from "react";

import { getGraphData } from "../../features/graph/api/graphApi";
import type { GraphEdge, GraphNode } from "../../features/graph/model/graphTypes";
import { getObjectTypeLabel } from "../../features/objects/model/objectOptions";
import { ObjectCardModal } from "../../features/objects/ui/ObjectCardModal";
import { RelationCardModal } from "../../features/relations/ui/RelationCardModal";
import { GraphVisualPreview } from "../../features/graph/ui/GraphVisualPreview";
import {
  getRelationConfidenceLabel,
  getRelationTypeLabel,
} from "../../features/relations/model/relationConstants";
import {
  defaultGraphFilters,
  filterGraphData,
  type GraphFilters,
} from "../../features/graph/lib/filterGraphData";
import { GraphFiltersPanel } from "../../features/graph/ui/GraphFiltersPanel";

type GraphPageProps = {
  caseId: string;
};

export function GraphPage({ caseId }: GraphPageProps) {
  const [nodes, setNodes] = useState<GraphNode[]>([]);
  const [edges, setEdges] = useState<GraphEdge[]>([]);
  const [isLoading, setIsLoading] = useState(false);
  const [errorText, setErrorText] = useState<string | null>(null);
  const [filters, setFilters] = useState<GraphFilters>(defaultGraphFilters);
  const [selectedObjectId, setSelectedObjectId] = useState<string | null>(null);
  const [selectedRelationId, setSelectedRelationId] = useState<string | null>(null);

  const loadGraphData = useCallback(async () => {
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
  }, [caseId]);

  useEffect(() => {
    void loadGraphData();
  }, [loadGraphData]);

  const filteredGraphData = useMemo(() => {
    return filterGraphData(nodes, edges, filters);
  }, [nodes, edges, filters]);

  const filteredNodeById = useMemo(() => {
    return new Map(filteredGraphData.nodes.map((node) => [node.id, node]));
  }, [filteredGraphData.nodes]);

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
              <span>Узлы после фильтра</span>
              <strong>{filteredGraphData.nodes.length}</strong>
            </div>

            <div className="summary-card">
              <span>Связи после фильтра</span>
              <strong>{filteredGraphData.edges.length}</strong>
            </div>

            <div className="summary-card">
              <span>Ключевые после фильтра</span>
              <strong>{filteredGraphData.nodes.filter((node) => node.isKey).length}</strong>
            </div>
          </div>

          <GraphFiltersPanel
            filters={filters}
            onChange={setFilters}
            onReset={() => setFilters(defaultGraphFilters)}
          />

          <GraphVisualPreview
            nodes={filteredGraphData.nodes}
            edges={filteredGraphData.edges}
            onNodeClick={setSelectedObjectId}
            onEdgeClick={setSelectedRelationId}
          />

          {nodes.length === 0 ? (
            <div className="empty-state">
              В деле пока нет объектов. Создай объекты в разделе «Объекты»,
              затем добавь связи.
            </div>
          ) : filteredGraphData.nodes.length === 0 ? (
            <div className="empty-state">
              По выбранным фильтрам нет объектов или связей. Сбрось фильтры или измени условия.
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
                    {filteredGraphData.nodes.map((node) => (
                      <tr
                        key={node.id}
                        className="graph-table-row-clickable"
                        onClick={() => setSelectedObjectId(node.id)}
                      >
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

                {filteredGraphData.edges.length === 0 ? (
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
                      {filteredGraphData.edges.map((edge) => {
                        const sourceNode = filteredNodeById.get(edge.sourceObjectId);
                        const targetNode = filteredNodeById.get(edge.targetObjectId);

                        return (
                          <tr
                            key={edge.id}
                            className="graph-table-row-clickable"
                            onClick={() => setSelectedRelationId(edge.id)}
                          >
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

      {selectedObjectId ? (
        <ObjectCardModal
          caseId={caseId}
          objectId={selectedObjectId}
          onClose={() => setSelectedObjectId(null)}
          onUpdated={() => {
            setSelectedObjectId(null);
            void loadGraphData();
          }}
          onDeleted={() => {
            setSelectedObjectId(null);
            void loadGraphData();
          }}
        />
      ) : null}

      {selectedRelationId ? (
        <RelationCardModal
          caseId={caseId}
          relationId={selectedRelationId}
          materials={[]}
          canEdit
          onClose={() => setSelectedRelationId(null)}
          onUpdated={() => {
            setSelectedRelationId(null);
            void loadGraphData();
          }}
          onDeleted={() => {
            setSelectedRelationId(null);
            void loadGraphData();
          }}
        />
      ) : null}
    </section>
  );
}
