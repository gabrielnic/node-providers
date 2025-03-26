"use client";
import 'bootstrap/dist/css/bootstrap.min.css';
import styles from "./page.module.css";

import { AccountData, GraphNode } from './types';
import Graph from './Graph';
import { useEffect, useState } from "react";
import { useWindowSize } from './hooks/useWindowSize';

interface GraphContainerProps {
    data: AccountData[];
    width: number;
    height: number;
    onNodeClick: (node: GraphNode) => void;
    loading: boolean;
}

export function GraphContainer({
    data,
    width,
    height,
    onNodeClick,
    loading,
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
            <Graph data={data} width={width} height={height} onNodeClick={onNodeClick} />
        </div>
    );
}

export default function Home() {

    const [loading, setLoading] = useState<boolean>(true);
    const [data, setData] = useState<AccountData[]>([]);
    useEffect(() => {
        fetch("/account_transactions.json")
            .then((res) => res.json())
            .then((data) => {
                setData(data);
                setLoading(false);
            })
            .catch((err) => { console.error("Failed to load JSON:", err); setLoading(false); });
    }, []);

    const [selectedNode, setSelectedNode] = useState<GraphNode | null>(null);
    const handleNodeClick = (node: GraphNode) => {
        setSelectedNode(node);
    };

    const { width, height } = useWindowSize();


    return (
        <div className={styles.page}>
            <main className={styles.main}>

                <div className={styles.ctas}>
                    <div className="container-fluid" style={{ height: "100vh" }}>
                        <div className="row h-100">
                            {/* Full-width graph */}
                            <div className="col-12 h-100 border">
                                <GraphContainer
                                    data={data}
                                    width={width * 0.75} // adjust as needed
                                    height={height}
                                    onNodeClick={handleNodeClick}
                                    loading={loading}
                                />
                            </div>
                        </div>
                    </div>
                </div>
            </main>
            <footer className={styles.footer}></footer>
        </div>
    );
}
