// src/components/GraphContainer.tsx
"use client";
import React from "react";
import Graph from "./Graph";
import { AccountData, GraphNode } from "../app/types"; // adjust the path as needed

interface GraphContainerProps {
  data: AccountData[];
  width: number;
  height: number;
  onNodeClick: (node: GraphNode) => void;
  loading: boolean;
  highlightNodeId?: string; 
}

export function GraphContainer({
  data,
  width,
  height,
  onNodeClick,
  loading,
  highlightNodeId
}: GraphContainerProps) {
  if (loading) {
    return (
      <div
        className="d-flex justify-content-center align-items-center"
        style={{ height: "100vh" }}
      >
        <div className="spinner-border" role="status">
          <span className="visually-hidden">Loading...</span>
        </div>
      </div>
    );
  }
  return (
    <div className="graph-container">
      <Graph data={data} width={width} height={height} onNodeClick={onNodeClick} highlightNodeId={highlightNodeId} />
    </div>
  );
}