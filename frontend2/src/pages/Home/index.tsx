import { useEffect, useState } from 'preact/hooks';
import preactLogo from '../../assets/preact.svg';
import './style.css';

import tx from "./account_transactions.json";
import { AccountData, GraphNode } from './types';
import MyGraph from './Graph';

export function useWindowSize() {
	const [size, setSize] = useState<{ width: number; height: number }>({
		width: window.innerWidth,
		height: window.innerHeight,
	});

	useEffect(() => {
		function handleResize() {
			setSize({ width: window.innerWidth, height: window.innerHeight });
		}
		window.addEventListener("resize", handleResize);
		return () => window.removeEventListener("resize", handleResize);
	}, []);

	return size;
}

export function Home() {

	useEffect(() => {
		fetch("/account_transactions.json")
			.then((res) => res.json())
			.then((data) => {
				setData(data);
			})
			.catch((err) => console.error("Failed to load JSON:", err));
	}, []);
	const [selectedNode, setSelectedNode] = useState<GraphNode | null>(null);
	const handleNodeClick = (node: GraphNode) => {
		setSelectedNode(node);
	};

	const { width, height } = useWindowSize();
	const [data, setData] = useState<AccountData[]>([]);
	const graphWidth = Math.max(0, width - 300);
	return (
		<div className="container-fluid" style={{ height: "100vh" }}>
      <div className="row h-100">
        {/* Left Column: Graph (occupies 9 of 12 columns) */}
        <div className="col-md-9 h-100">
          {/* Optionally, if you want to pass width based on container, you could calculate:
              const graphWidth = width * (9 / 12);
              Otherwise, pass full window width or a fixed value.
          */}
          <MyGraph 
            data={data}
            width={width * 0.75} // approx 9/12 of the window width
            height={height}
            onNodeClick={handleNodeClick}
          />
        </div>

        {/* Right Column: Side Panel as a Bootstrap Card */}
        <div className="col-md-3">
          <div className="card mt-3">
            <div className="card-header">
              <h5 className="card-title">Selected Node</h5>
            </div>
            <div className="card-body">
              {selectedNode ? (
                <div>
                  <p><strong>Label:</strong> {selectedNode.label}</p>
                  <p className={"account-id"}><strong>Account ID:</strong> {selectedNode.id}</p>
                  <p><strong>Type:</strong> {selectedNode.group}</p>
                </div>
              ) : (
                <p>No node selected.</p>
              )}
            </div>
          </div>
        </div>
      </div>
    </div>
	);
}

function Resource(props) {
	return (
		<a href={props.href} target="_blank" class="resource">
			<h2>{props.title}</h2>
			<p>{props.description}</p>
		</a>
	);
}
