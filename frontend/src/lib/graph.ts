import {
	forceSimulation,
	type SimulationNodeDatum,
	type SimulationLinkDatum,
	forceLink,
	create,
	forceManyBody,
	forceCenter,
	drag,
	zoom,
	forceCollide
} from 'd3';

export enum Direction {
	SEND = 'SEND',
	RECEIVE = 'RECEIVE',
	BOTH = 'BOTH'
}

export interface Node extends SimulationNodeDatum {
	id: string;
	account: string;
	color?: string;
}

export interface Link extends SimulationLinkDatum<Node> {
	direction: Direction;
}

export function newSimulation(
	nodes: Node[],
	links: Link[],
	width: number = 200,
	height: number = 200
): SVGSVGElement {
	const r = 5;

	const simulation = forceSimulation(nodes)
		.force('link', forceLink(links).distance(50 + 2 * r))
		.force('charge', forceManyBody().strength(-30).distanceMax(300))
		.force('center', forceCenter(width / 2, height / 2))
		.force('collide', forceCollide(r + 2))
		.on('tick', tick);

	const svg = create('svg')
		.attr('id', 'graph')
		.attr('width', '100%')
		.attr('viewBox', [0, 0, width, height])
		.attr('style', 'max-width: 100%; height: auto;')
		.call(zoom<SVGSVGElement, undefined>().on('zoom', zoomBehavior));

	function zoomBehavior(this: SVGSVGElement, event: any, _: undefined) {
		const { transform } = event;
		svg.selectAll('.links').attr('transform', transform);
		svg.selectAll('.nodes').attr('transform', transform);
	}

	const linkGroup = svg
		.append('g')
		.attr('class', 'links')
		.selectAll()
		.data(links)
		.enter()
		.append('g')
		.attr('class', 'link')
		.append('line')
		.attr('stroke', '#999')
		.attr('stroke-opacity', 0.5)
		.attr('stroke-width', 0.5)
		.attr('class', 'link-line');

	const linkLabels = svg
		.selectAll<SVGGElement, Link>('.link')
		.append('text')
		.attr('font-size', '8px')
		.attr('text-anchor', 'middle')
		.text('');

	const nodeGroup = svg
		.append('g')
		.attr('class', 'nodes')
		.selectAll()
		.data(nodes)
		.enter()
		.append('g')
		.attr('class', 'node')
		.append('circle')
		.attr('stroke-width', 1)
		.attr('fill', 'white')
		.attr('stroke', (n) => n.color ?? '#999')
		.attr('class', 'node')
		.attr('id', (n) => n.id)
		.attr('r', r)
		.attr('class', 'node-circle');

	const nodeLabels = svg
		.selectAll<SVGGElement, Node>('.node')
		.append('text')
		.attr('font-size', '5px')
		.attr('text-anchor', 'middle')
		.text((d) => d.id.slice(0, 2).toUpperCase());

	function nodeStartDrag(this: SVGCircleElement, event: any, n: Node) {
		if (!event.active) simulation.alphaTarget(0.1).restart();
		n.fx = n.x;
		n.fy = n.y;
	}

	function nodeDrag(this: SVGCircleElement, event: any, n: Node) {
		n.fx = event.x;
		n.fy = event.y;
	}

	function nodeEndDrag(this: SVGCircleElement, event: any, n: Node) {
		if (!event.active) simulation.alphaTarget(0);
		n.fx = null;
		n.fy = null;
	}

	function linkText(this: SVGTextElement, l: Link): string {
		const x1 = (l.source as Node).x!;
		const x2 = (l.target as Node).x!;

		switch (l.direction) {
			case Direction.SEND:
				return x2 < x1 ? '←' : '→';
			case Direction.RECEIVE:
				return x2 < x1 ? '→' : '←';
			default:
				return '↔';
		}
	}

	svg
		.selectAll<SVGCircleElement, Node>('.node')
		.call(
			drag<SVGCircleElement, Node>()
				.on('start', nodeStartDrag)
				.on('drag', nodeDrag)
				.on('end', nodeEndDrag)
		);

	function tick() {
		linkGroup
			.attr('x1', (d) => (d.source as Node).x!)
			.attr('y1', (d) => (d.source as Node).y!)
			.attr('x2', (d) => (d.target as Node).x!)
			.attr('y2', (d) => (d.target as Node).y!);
		linkLabels
			.attr('transform', (d) => {
				const x1 = (d.source as Node).x!;
				const y1 = (d.source as Node).y!;
				const x2 = (d.target as Node).x!;
				const y2 = (d.target as Node).y!;
				let angle = Math.atan2(y2 - y1, x2 - x1) * (180 / Math.PI);
				if (x2 < x1) angle += 180;
				return `rotate(${angle}, ${(x1 + x2) / 2}, ${(y1 + y2) / 2})`;
			})
			.attr('x', (d) => ((d.source as Node).x! + (d.target as Node).x!) / 2)
			.attr('y', (d) => ((d.source as Node).y! + (d.target as Node).y!) / 2)
			.text(linkText);

		nodeGroup.attr('cx', (d) => d.x!).attr('cy', (d) => d.y!);
		nodeLabels.attr('x', (d) => d.x!).attr('y', (d) => d.y! + 2);
	}

	while (simulation.alpha() > 0.01) {
		simulation.tick();
	}

	return svg.node()!;
}
