"use client";
import { AccountData, GraphNode, GraphLink } from "./types";

// Build a graph from an array of AccountData
export function buildGraph(data: AccountData[]): {
  nodes: GraphNode[];
  links: GraphLink[];
} {
  // Build a map of accounts from your data.
  const accountMap = new Map<string, AccountData>();
  data.forEach(acc => {
    accountMap.set(acc.account, acc);
  });

  // Create nodes from each account.
  const nodes: GraphNode[] = data.map(acc => ({
    id: acc.account,  // using the account id (64 hex characters)
    label: acc.name,
    group: acc.ty,
  }));

  // Create links based on transfer transactions.
  const links: GraphLink[] = [];
  data.forEach(acc => {
    acc.transactions.forEach(tx => {
      const op = tx.transaction.operation;
      if ("Transfer" in op) {
        const { from, to } = op.Transfer;
        // Only create a link if both 'from' and 'to' exist in the dataset.
        if (accountMap.has(from) && accountMap.has(to)) {
          links.push({
            source: from,
            target: to,
          });
        }
      }
    });
  });

  return { nodes, links };
}