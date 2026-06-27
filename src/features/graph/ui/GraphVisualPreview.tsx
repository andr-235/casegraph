import type { CSSProperties } from "react";

import type { GraphEdge, GraphNode } from "../model/graphTypes";
import { getObjectTypeLabel } from "../../objects/model/objectOptions";
import {
  getRelationConfidenceLabel,
  getRelationTypeLabel,
} from "../../relations/model/relationConstants";

type GraphVisualPreviewProps = {
  nodes: GraphNode[];
  edges: GraphEdge[];
  onNodeClick?: (nodeId: string) => void;
  onEdgeClick?: (edgeId: string) => void;
};

type PositionedNode = GraphNode & {
  x: number;
  y: number;
};

const CANVAS_WIDTH = 920;
const CANVAS_HEIGHT = 460;
const NODE_WIDTH = 168;
const NODE_HEIGHT = 76;

export function GraphVisualPreview({
  nodes,
  edges,
  onNodeClick,
  onEdgeClick,
}: GraphVisualPreviewProps) {
  if (nodes.length === 0) {
    return (
      <div className="graph-preview-empty">
        Нет объектов для отображения графа.
      </div>
    );
  }

  const positionedNodes = buildNodeLayout(nodes);
  const positionedNodeById = new Map(
    positionedNodes.map((node) => [node.id, node]),
  );

  return (
    <section className="graph-preview-panel">
      <div className="graph-preview-header">
        <div>
          <h2>Визуальная схема</h2>
          <p>Базовый preview активных объектов и связей дела.</p>
        </div>

        <div className="graph-preview-meta">
          {nodes.length} узл. · {edges.length} св.
        </div>
      </div>

      <div
        className="graph-preview-canvas"
        style={
          {
            "--graph-canvas-width": `${CANVAS_WIDTH}px`,
            "--graph-canvas-height": `${CANVAS_HEIGHT}px`,
          } as CSSProperties
        }
      >
        <svg
          className="graph-preview-svg"
          viewBox={`0 0 ${CANVAS_WIDTH} ${CANVAS_HEIGHT}`}
          role="img"
          aria-label="Визуальный граф связей дела"
        >
          <defs>
            <marker
              id="graph-arrow"
              markerWidth="10"
              markerHeight="10"
              refX="8"
              refY="3"
              orient="auto"
              markerUnits="strokeWidth"
            >
              <path d="M0,0 L0,6 L9,3 z" />
            </marker>
          </defs>

          {edges.map((edge) => {
            const sourceNode = positionedNodeById.get(edge.sourceObjectId);
            const targetNode = positionedNodeById.get(edge.targetObjectId);

            if (!sourceNode || !targetNode) {
              return null;
            }

            const sourcePoint = getNodeCenter(sourceNode);
            const targetPoint = getNodeCenter(targetNode);
            const labelPoint = getMidpoint(sourcePoint, targetPoint);

            return (
              <g
                key={edge.id}
                className={
                  onEdgeClick
                    ? "graph-preview-edge graph-preview-edge-clickable"
                    : "graph-preview-edge"
                }
                onClick={() => onEdgeClick?.(edge.id)}
              >
                <line
                  x1={sourcePoint.x}
                  y1={sourcePoint.y}
                  x2={targetPoint.x}
                  y2={targetPoint.y}
                  markerEnd="url(#graph-arrow)"
                />

                <text x={labelPoint.x} y={labelPoint.y - 8}>
                  {edge.relationCode}
                </text>
              </g>
            );
          })}
        </svg>

        {positionedNodes.map((node) => (
          <button
            key={node.id}
            type="button"
            className={[
              "graph-preview-node",
              node.isKey ? "graph-preview-node-key" : "",
            ]
              .filter(Boolean)
              .join(" ")}
            style={{
              left: `${node.x}px`,
              top: `${node.y}px`,
              width: `${NODE_WIDTH}px`,
              minHeight: `${NODE_HEIGHT}px`,
            }}
            onClick={() => onNodeClick?.(node.id)}
          >
            <div className="graph-preview-node-top">
              <span>{node.objectCode}</span>
              {node.isKey ? <span>★</span> : null}
            </div>

            <strong className="graph-preview-node-title">
              {node.title}
            </strong>

            <span className="graph-preview-node-meta">
              {getObjectTypeLabel(node.objectType)}
            </span>
          </button>
        ))}
      </div>

      {edges.length > 0 ? (
        <div className="graph-preview-legend">
          {edges.slice(0, 5).map((edge) => (
            <div key={edge.id} className="graph-preview-legend-item">
              <strong>{edge.relationCode}</strong>
              <span>
                {getRelationTypeLabel(edge.relationType)} ·{" "}
                {getRelationConfidenceLabel(edge.confidenceLevel)}
              </span>
            </div>
          ))}

          {edges.length > 5 ? (
            <div className="graph-preview-legend-item">
              <strong>+{edges.length - 5}</strong>
              <span>ещё связей в таблице ниже</span>
            </div>
          ) : null}
        </div>
      ) : null}
    </section>
  );
}

function buildNodeLayout(nodes: GraphNode[]): PositionedNode[] {
  if (nodes.length === 1) {
    return [
      {
        ...nodes[0],
        x: CANVAS_WIDTH / 2 - NODE_WIDTH / 2,
        y: CANVAS_HEIGHT / 2 - NODE_HEIGHT / 2,
      },
    ];
  }

  const centerX = CANVAS_WIDTH / 2;
  const centerY = CANVAS_HEIGHT / 2;
  const radiusX = Math.min(330, CANVAS_WIDTH / 2 - NODE_WIDTH);
  const radiusY = Math.min(155, CANVAS_HEIGHT / 2 - NODE_HEIGHT);

  return nodes.map((node, index) => {
    const angle = (2 * Math.PI * index) / nodes.length - Math.PI / 2;

    return {
      ...node,
      x: centerX + radiusX * Math.cos(angle) - NODE_WIDTH / 2,
      y: centerY + radiusY * Math.sin(angle) - NODE_HEIGHT / 2,
    };
  });
}

function getNodeCenter(node: PositionedNode) {
  return {
    x: node.x + NODE_WIDTH / 2,
    y: node.y + NODE_HEIGHT / 2,
  };
}

function getMidpoint(
  source: { x: number; y: number },
  target: { x: number; y: number },
) {
  return {
    x: (source.x + target.x) / 2,
    y: (source.y + target.y) / 2,
  };
}
