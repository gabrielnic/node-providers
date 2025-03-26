"use client";
import 'bootstrap/dist/css/bootstrap.min.css';
import styles from "./page.module.css";

import { AccountData, GraphNode } from './types';
import MyGraph from './Graph';
import { useEffect, useState } from "react";
import { useWindowSize } from './hooks/useWindowSize';

export default function Home() {
	const [data, setData] = useState<AccountData[]>([]);
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


  return (
    <div className={styles.page}>
      <main className={styles.main}>
        <ul>
          <li>
            Source it by running this command <code>cargo run np-tool</code> inside the np-tool folder.
          </li>
          <li>
          Legend: 
              <ol>
                <li>Exchange: orange</li>
                <li>Individual: blue</li>
                <li>Foundation: yellow</li>
                <li>NodeProvider: red</li>
                <li>Sns: orange</li>
                <li>Spammer: purple</li>
              </ol>
          </li>
        </ul>

        <div className={styles.ctas}>
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
        </div>
      </main>
      <footer className={styles.footer}>
       
      </footer>
    </div>
  );
}
