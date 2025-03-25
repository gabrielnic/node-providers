// Data from your JSON
export interface AccountData {
    name: string;
    principal?: string;
    account: string;
    ty: string;  // "Exchange", "Individual", etc.
    transactions: TransactionWithId[];
  }
  
  export interface TransactionWithId {
    id: number;
    transaction: {
      memo: number;
      icrc1_memo: unknown | null;
      operation: Operation;
      timestamp?: { timestamp_nanos: number };
      created_at_time?: { timestamp_nanos: number };
    };
  }
  
  export type Operation =
    | { Transfer: { from: string; to: string; fee: { e8s: number }; amount: { e8s: number }; spender?: string | null } }
    | { Mint: { to: string; amount: { e8s: number } } }
    | { Burn: { from: string; amount: { e8s: number }; spender?: string | null } }
    | { Approve: { /* omitted for brevity */ } };
  
  // D3 simulation node & link definitions
  
  export interface GraphNode extends SimulationNodeDatum {
    id: string;    // matches the "account" hex string
    label: string; // e.g. the name
    group: string; // e.g. "Exchange", "Individual"
  }
  
  export interface GraphLink extends SimulationLinkDatum<GraphNode> {
    source: string; // account "from"
    target: string; // account "to"
  }


  import { SimulationNodeDatum, SimulationLinkDatum } from "d3-force";

export enum Direction {
  SEND = "SEND",
  RECEIVE = "RECEIVE",
  BOTH = "BOTH",
}

// Each node extends SimulationNodeDatum
export interface Node extends SimulationNodeDatum {
  id: string;
  account: string;
  color?: string;
}

// Each link extends SimulationLinkDatum<Node>
export interface Link extends SimulationLinkDatum<Node> {
  direction: Direction;
}