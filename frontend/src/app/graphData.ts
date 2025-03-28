"use client";
import { AccountData, GraphNode, GraphLink, Direction } from "./types";

// Build a graph from an array of AccountData
export function buildGraph(data: AccountData[]): {
  nodes: GraphNode[];
  links: GraphLink[];
} {
   // Create a map of main accounts keyed by the main account id.
   const mainMap = new Map<string, AccountData>();
   data.forEach(acc => {
     mainMap.set(acc.account, acc);
   });
 
   // Create nodes from main accounts only.
   const nodeMap = new Map<string, GraphNode>();
   data.forEach(acc => {
     nodeMap.set(acc.account, {
       id: acc.account,
       label: acc.name,
       group: acc.ty,
     });
   });

   const connectorMap = new Map<string, Set<string>>();

   // 4. Process transactions to create links.
   const links: GraphLink[] = [];
   data.forEach(acc => {
     acc.transactions.forEach(tx => {
       if (tx.op_type === "Transfer") {
         const from = tx.from;
         const to = tx.to;
         const fromIsMain = mainMap.has(from);
         const toIsMain = mainMap.has(to);
 
         if (fromIsMain && toIsMain) {
           // Both endpoints are main: add direct link (if not already present).
           let existing = links.find(l =>
             (l.source === from && l.target === to) ||
             (l.source === to && l.target === from)
           );
           if (!existing) {
             let direction: Direction = (from === acc.account) ? Direction.SEND : Direction.RECEIVE;
             links.push({
               source: from,
               target: to,
               direction,
             });
           } else {
             let direction: Direction = (from === acc.account) ? Direction.SEND : Direction.RECEIVE;
             if (existing.direction !== direction) {
               existing.direction = Direction.BOTH;
             }
           }
         } else {
           // At least one endpoint is extra.
           // Record extra account(s) and the main account from the current AccountData.
           if (!fromIsMain) {
             if (!connectorMap.has(from)) {
               connectorMap.set(from, new Set());
             }
             connectorMap.get(from)!.add(acc.account);
           }
           if (!toIsMain) {
             if (!connectorMap.has(to)) {
               connectorMap.set(to, new Set());
             }
             connectorMap.get(to)!.add(acc.account);
           }
           // Do not add a link here yet.
         }
       }
     });
   });
 
  
   connectorMap.forEach((mainSet, extraId) => {
    if (mainSet.size > 1) {
      // Check if at least one connected main account is not an Exchange.
      const hasNonExchange = Array.from(mainSet).some(mainAccId => {
        const mainData = mainMap.get(mainAccId);
        return mainData && mainData.ty !== "Exchange";
      });
      if (hasNonExchange) {
        // Build a label by concatenating initials of each main account's name.
        let label = "";
        mainSet.forEach(mainAccId => {
          const mainData = mainMap.get(mainAccId);
          if (mainData) {
            label += initials(mainData.name);
          }
        });
        // Create the connector node.
        nodeMap.set(extraId, {
          id: extraId,
          label,
          group: "connector",
        });
        // Create links from each main node to this connector node.
        mainSet.forEach(mainAccId => {
          links.push({
            source: mainAccId,
            target: extraId,
            direction: Direction.SEND,
          });
        });
      }
      // Otherwise, if all connected main accounts are exchanges, skip creating a connector node.
    }
  });
 
   const nodes = Array.from(nodeMap.values());
   return { nodes, links };
}

function initials(name: string): string {
  return name.replace(/\s+/g, "").slice(0, 2);
}

function recordConnection(map: Map<string, Set<string>>, connectingId: string, mainName: string) {
  if (!map.has(connectingId)) {
    map.set(connectingId, new Set());
  }
  map.get(connectingId)!.add(mainName);
}