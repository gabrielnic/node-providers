/* eslint-disable @typescript-eslint/no-explicit-any */
/* eslint-disable @typescript-eslint/no-unused-vars */
"use client";
import React, { useRef, useEffect } from "react";
import { select } from "d3-selection";
import {
  forceSimulation,
  forceLink,
  forceManyBody,
  forceCenter,
  forceCollide,
} from "d3-force";
import { D3DragEvent, drag } from "d3-drag";
import { zoom, zoomIdentity } from "d3-zoom";
import { AccountData, GraphNode } from "./types";
import { buildGraph } from "./graphData";

// Define a type for links.
export interface GraphLink {
  source: string | GraphNode;
  target: string | GraphNode;
}

interface GraphProps {
  data: AccountData[];
  width?: number;
  height?: number;
  onNodeClick?: (node: GraphNode) => void;
  highlightNodeId?: string; // Node to highlight (by id)
}

const Graph: React.FC<GraphProps> = ({
  data,
  width = 800,
  height = 600,
  onNodeClick,
  highlightNodeId,
}) => {
  const svgRef = useRef<SVGSVGElement>(null);
  const zoomRef = useRef<any>(null);
  // Ref to store node positions (keyed by node id).
  const positionsRef = useRef<{ [key: string]: { x: number; y: number } }>({});

  function isGraphNode(obj: any): obj is GraphNode {
    return obj && typeof obj.id === "string";
  }

  useEffect(() => {
    const svg = select(svgRef.current!)
      .attr("width", width)
      .attr("height", height);

    if (!data || data.length === 0) return;
    const newData: any = data.filter((d) => d.ty !== "Spammer");
    const { nodes, links } = buildGraph(newData);
    // Create a container group for simulation elements.
    const container = svg.append("g").attr("class", "container");

    // Define zoom behavior that updates the container’s transform.
    const zoomBehavior = zoom<SVGSVGElement, any>()
      .filter((event: any) => {
        if (event.sourceEvent && event.sourceEvent.type === "click") return false;
        if (
          event.sourceEvent &&
          event.sourceEvent.target &&
          (event.sourceEvent.target as HTMLElement).tagName === "CIRCLE"
        )
          return false;
        return true;
      })
      .on("zoom", (event) => {
        container.attr("transform", event.transform.toString());
      });
    zoomRef.current = zoomBehavior;
    svg.call(zoomBehavior);

    // Define arrow marker for links.
    const defs = svg.append("defs");
    defs.append("marker")
      .attr("id", "arrowhead")
      .attr("viewBox", "0 -5 10 10")
      .attr("refX", 75)
      .attr("refY", 0)
      .attr("markerWidth", 6)
      .attr("markerHeight", 6)
      .attr("orient", "auto")
      .append("path")
      .attr("d", "M0,-5L10,0L0,5")
      .attr("fill", "#999");

    // Create force simulation.
    const simulation = forceSimulation(nodes)
      .force("link", forceLink<GraphNode, GraphLink>(links).id((d) => d.id).distance(80))
      .force("charge", forceManyBody().strength(-300))
      .force("center", forceCenter(width / 2, height / 2))
      .force("collide", forceCollide(25))
      .on("tick", tick);

    // Create link lines.
    const linkSelection = container
      .append("g")
      .attr("class", "links")
      .selectAll("line")
      .data(links)
      .join("line")
      .attr("stroke", "#999")
      .attr("stroke-opacity", 0.6)
      .attr("stroke-width", 1.5)
      .attr("marker-end", "url(#arrowhead)");

    // Create node circles.
    const nodeSelection = container
      .append("g")
      .attr("class", "nodes")
      .selectAll("circle")
      .data(nodes)
      .join("circle")
      .attr("id", (d: any) => d.id ?? "")
      .attr("r", 10)
      .attr("fill", (d) => {
        switch (d.group) {
            case "Cex":
                return "blue";
            case "Dex":
                return "lightblue";
            case "Identified":
                return "green";
            case "Foundation":
                return "yellow";
            case "NodeProvider":
                return "darkred";
            case "Spammer":
                return "salmon";
            case "Sns":
                return "purple";
            case "Suspect":
                return "orange";
            default:
                return "gray";
        }
      })
      .attr("stroke-width", 1)
      .attr("stroke", (d) => d.color ?? "#999")
      .call(
        (drag<SVGCircleElement, GraphNode, GraphNode>() as any)
          .on("start", (event: D3DragEvent<SVGCircleElement, GraphNode, GraphNode>, d: GraphNode) => {
            if (!event.active) simulation.alphaTarget(0.3).restart();
            d.fx = d.x;
            d.fy = d.y;
          })
          .on("drag", (event: D3DragEvent<SVGCircleElement, GraphNode, GraphNode>, d: GraphNode) => {
            d.fx = event.x;
            d.fy = event.y;
          })
          .on("end", (event: D3DragEvent<SVGCircleElement, GraphNode, GraphNode>, d: GraphNode) => {
            if (!event.active) simulation.alphaTarget(0);
          })
      )
      .on("mouseover", function (event, d) {
        (select(this) as any)
          .transition()
          .duration(200)
          .attr("r", 15)
          .attr("stroke-width", 2);
        select(this).style("cursor", "pointer");
      })
      .on("mouseout", function (event, d) {
        (select(this) as any)
          .transition()
          .duration(200)
          .attr("r", 10)
          .attr("stroke-width", 1);
      })
      .on("click", (event, d) => {
        event.preventDefault();
        event.stopPropagation();
        event.stopImmediatePropagation();

        // Reset styles on all nodes.
        (nodeSelection as any)
          .transition()
          .duration(200)
          .attr("stroke", (d: any) => d.color ?? "#999")
          .attr("stroke-width", 1)
          .attr("r", 10);

        // Determine connected node IDs.
        const connectedNodeIds = new Set<string>();
        links.forEach((link) => {
          if (isGraphNode(link.source) && isGraphNode(link.target)) {
            if (link.source.id === d.id) {
              connectedNodeIds.add(link.target.id);
            } else if (link.target.id === d.id) {
              connectedNodeIds.add(link.source.id);
            }
          }
        });
        // Include the clicked node.
        connectedNodeIds.add(d.id);

        // Highlight connected nodes.
        (nodeSelection as any)
          .filter((node: GraphNode) => connectedNodeIds.has(node.id))
          .transition()
          .duration(200)
          .attr("stroke", "#fa00f2")
          .attr("stroke-width", 3)
          .attr("r", 15);

        if (onNodeClick) {
          onNodeClick(d);
        }
      });

    // Create labels for nodes.
    const labelSelection = container
      .append("g")
      .attr("class", "labels")
      .selectAll("text")
      .data(nodes)
      .join("text")
      .text((d) => d.label)
      .attr("font-size", 12)
      .attr("dx", 12)
      .attr("dy", "0.35em");

    // Update positions on each tick.
    function tick() {
      // Update links.
      linkSelection
        .attr("x1", (d) =>
          typeof d.source !== "string" ? (d.source as GraphNode).x || 0 : 0
        )
        .attr("y1", (d) =>
          typeof d.source !== "string" ? (d.source as GraphNode).y || 0 : 0
        )
        .attr("x2", (d) =>
          typeof d.target !== "string" ? (d.target as GraphNode).x || 0 : 0
        )
        .attr("y2", (d) =>
          typeof d.target !== "string" ? (d.target as GraphNode).y || 0 : 0
        );

      // Update nodes.
      nodeSelection
        .attr("cx", (d) => d.x || 0)
        .attr("cy", (d) => d.y || 0);

      // Update labels.
      labelSelection
        .attr("x", (d) => d.x || 0)
        .attr("y", (d) => d.y || 0);

      // Record each node's position in the ref.
      nodes.forEach((node: GraphNode) => {
        positionsRef.current[node.id] = { x: node.x || 0, y: node.y || 0 };
      });
    }

    // Cleanup on component unmount.
    return () => {
      simulation.stop();
    };
    // eslint-disable-next-line react-hooks/exhaustive-deps
  }, [data, width, height]);

  useEffect(() => {
    if (!svgRef.current) return;
    if (!highlightNodeId) return;
    const svgEl: any = select(svgRef.current);
    // Find the circle by id.
    const node = svgEl.select(`circle[id="${highlightNodeId}"]`);
    if (!node.empty()) {
      // Retrieve the node's recorded position.
      const pos = positionsRef.current[highlightNodeId];
      if (!pos) {
        console.warn("Recorded node position not available; skipping zoom.");
        return;
      }

      // Highlight the node visually.
      node.transition().duration(500)
        .attr("r", 20)
        .attr("stroke", "rgb(255,192,203)");

      // Get the SVG dimensions.
      const svgWidth = +svgEl.attr("width");
      const svgHeight = +svgEl.attr("height");

      // Define desired zoom level.
      const zoomLevel = 2; // Adjust as needed

      // Calculate translation to center the node.
      const translateX = (svgWidth / 2) - pos.x * zoomLevel;
      const translateY = (svgHeight / 2) - pos.y * zoomLevel;

      const transform = zoomIdentity
        .translate(translateX, translateY)
        .scale(zoomLevel);

      // Apply the zoom transform.
      svgEl.transition().duration(500)
        .call(zoomRef.current.transform, transform);
    }
  }, [highlightNodeId]);

  return <svg ref={svgRef} />;
};

export default Graph;
