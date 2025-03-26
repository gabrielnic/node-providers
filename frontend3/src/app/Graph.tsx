import React, { useRef, useEffect } from "react";
import { select } from "d3-selection";
import {
  forceSimulation,
  forceLink,
  forceManyBody,
  forceCenter,
  forceCollide
} from "d3-force";
import { drag } from "d3-drag";
import { zoom } from "d3-zoom";
import { AccountData, GraphNode } from "./types";
import { buildGraph } from "./graphData";

// Define a type for links if not already defined.
export interface GraphLink {
  source: string | GraphNode;
  target: string | GraphNode;
  // You can add additional properties if needed.
}

interface MyGraphProps {
  data: AccountData[];
  width?: number;
  height?: number;
  onNodeClick?: (node: GraphNode) => void;
}

const MyGraph: React.FC<MyGraphProps> = ({
  data,
  width = 800,
  height = 600,
  onNodeClick,
}) => {
  const svgRef = useRef<SVGSVGElement>(null);

  useEffect(() => {
    if (!data || data.length === 0) return;
    console.log('refreshed...');
    const { nodes, links } = buildGraph(data);

    // 2. Select/prepare the SVG and clear previous content
    const svg = select(svgRef.current!)
      .attr("width", width)
      .attr("height", height)
      .call(zoom<SVGSVGElement, undefined>().on('zoom', zoomBehavior));
    svg.selectAll("*").remove();
    function zoomBehavior(this: SVGSVGElement, event: any, _: undefined) {
      const { transform } = event;
      svg.selectAll('.links').attr('transform', transform);
      svg.selectAll('.nodes').attr('transform', transform);
    }
  
    // 3. Define arrow marker for links
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

    // 4. Create the force simulation
    const simulation = forceSimulation(nodes)
      .force("link", forceLink<GraphNode, GraphLink>(links).id((d) => d.id).distance(80))
      .force("charge", forceManyBody().strength(-300))
      .force("center", forceCenter(width / 2, height / 2))
      .force("collide", forceCollide(25))
      .on('tick', tick);


      function tick() {
       console.log('tick');
      }

    // 5. Create link lines with arrow markers at the end
    const linkSelection = svg
      .append("g")
      .attr("class", "links")
      .selectAll("line")
      .data(links)
      .join("line")
      .attr("stroke", "#999")
      .attr("stroke-opacity", 0.6)
      .attr("stroke-width", 1.5)
      .attr("marker-end", "url(#arrowhead)"); // Use marker-end

    // 6. Create node circles with drag and click handlers
    const nodeSelection = svg
      .append("g")
      .attr("class", "nodes")
      .selectAll("circle")
      .data(nodes)
      .join("circle")
      .attr("r", 10)
      .attr("fill", (d) => {
        switch (d.group) {
          case "Exchange":
            return "orange";
          case "Individual":
            return "blue";
          case "Foundation":
            return "yellow";
          case "NodeProvider":
            return "red";   
          case "Sns":
            return "green";           
          case "Spammer":
            return "purple";    
          default:
            return "gray";
        }
      })
      .attr("stroke-width", 1)
      .attr("stroke", (d) => d.color ?? "#999")
      .call(
        drag<SVGCircleElement, GraphNode>()
          .on("start", (event, d) => {
            if (!event.active) simulation.alphaTarget(0.3).restart();
            d.fx = d.x;
            d.fy = d.y;
          })
          .on("drag", (event, d) => {
            d.fx = event.x;
            d.fy = event.y;
          })
          .on("end", (event, d) => {
            if (!event.active) simulation.alphaTarget(0);
            d.fx = null;
            d.fy = null;
          })
      )
      .on("click", (event, d) => {
        console.log(event);
        event.preventDefault();
        event.stopPropagation();
        event.stopImmediatePropagation();
        if (onNodeClick) {
          onNodeClick(d);
        }
      });

    // 7. Create labels for nodes
    const labelSelection = svg
      .append("g")
      .selectAll("text")
      .data(nodes)
      .join("text")
      .text((d) => d.label)
      .attr("font-size", 12)
      .attr("dx", 12)
      .attr("dy", "0.35em");

    // 8. Update positions on each tick of the simulation
    simulation.on("tick", () => {
      linkSelection
        .attr("x1", (d) => (typeof d.source !== "string" ? d.source.x : 0))
        .attr("y1", (d) => (typeof d.source !== "string" ? d.source.y : 0))
        .attr("x2", (d) => (typeof d.target !== "string" ? d.target.x : 0))
        .attr("y2", (d) => (typeof d.target !== "string" ? d.target.y : 0));

      nodeSelection
        .attr("cx", (d) => d.x || 0)
        .attr("cy", (d) => d.y || 0);

      labelSelection
        .attr("x", (d) => d.x || 0)
        .attr("y", (d) => d.y || 0);
    });

    // 9. Enable zoom/pan, but filter out clicks on nodes
    svg.call(
      zoom<SVGSVGElement, unknown>()
        .filter((event: any) => {
          console.log(event);
          if (event.sourceEvent && event.sourceEvent.type === "click") {
            return false;
          }
          // If the underlying event's target is a circle (node), don't trigger zoom.
          if (
            event.sourceEvent &&
            event.sourceEvent.target &&
            (event.sourceEvent.target as HTMLElement).tagName === "CIRCLE"
          ) {
            return false;
          }
          return true;
        })
        .on("zoom", (event: any) => {
          svg.selectAll("g").attr("transform", event.transform);
        })
    );


    // Cleanup on component unmount
    return () => {
      simulation.stop();
    };
  }, [data, width, height]);

  return <svg ref={svgRef} />;
};

export default MyGraph;
