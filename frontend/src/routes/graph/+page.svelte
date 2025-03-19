<script lang="ts">
	import { Direction, type Link, newSimulation, type Node } from '$lib/graph';
	import { onMount } from 'svelte';
	import nodeProvidersData from '../combined_providers.json';

	let nodeProviders = nodeProvidersData;
	let loadingTransactions = true;
	let processPercentage = 0;
	let selectedAccount: string | null = null;
	let nodeProviderData: any = null;
	let filterDeadEndsEnabled = true;
	let filterKnownExchangesEnabled = false;
	let filterUnconnectedNodesEnabled = true;
	let nodes: Node[] = [];
	let links: Link[] = [];

	let knownAddresses: Record<string, string> = {
		dd15f3040edab88d2e277f9d2fa5cc11616ebf1442279092e37924ab7cce8a74: '?',
		'00c3df112e62ad353b7cc7bf8ad8ce2fec8f5e633f1733834bf71e40b250c685': '?',
		'4dfa940def17f1427ae47378c440f10185867677109a02bc8374fc25b9dee8af': '?',
		'609d3e1e45103a82adc97d4f88c51f78dedb25701e8e51e8c4fec53448aadc29': 'Binance 1',
		'220c3a33f90601896e26f76fa619fe288742df1fa75426edfaf759d39f2455a5': 'Binance 2',
		'449ce7ad1298e2ed2781ed379aba25efc2748d14c60ede190ad7621724b9e8b2': 'Coinbase',
		'8fe706db7b08f957a15199e07761039a7718937aabcc0fe48bc380a4daf9afb0': 'Gate.io',
		e7a879ea563d273c46dd28c1584eaa132fad6f3e316615b3eb657d067f3519b5: 'OKX 1',
		d2c6135510eaf107bdc2128ef5962c7db2ae840efdf95b9395cdaf4983942978: 'OKX 2',
		'040834c30cdf5d7a13aae8b57d94ae2d07eefe2bc3edd8cf88298730857ac2eb': 'Kraken'
	};

	function getColor(account: string): string | undefined {
		if (knownAddresses[account]) {
			return 'blue';
		}
		return undefined;
	}

	let rootAddresses: Node[] = nodeProviders
		.filter((p) => p.rewards)
		.map(({ principal_id, rewards }) => {
			const n: Node = {
				id: principal_id!,
				account: rewards!.reward_account_formatted
			};
			return n;
		});

	interface Block {
		allowance: string;
		amount: string;
		block_hash: string;
		block_height: string;
		created_at: number;
		expected_allowance: string;
		expires_at: number;
		fee: string;
		from_account_identifier: string;
		icrc1_memo: string;
		memo: string;
		parent_hash: string;
		spender_account_identifier: string;
		to_account_identifier: string;
		transaction_hash: string;
		transfer_type: string;
	}

	interface BlocksResponse {
		blocks: Block[];
		total: number;
	}

	async function getBlocks(account: string): Promise<Block[]> {
		const url = `https://ledger-api.internetcomputer.org/accounts/${account}/transactions?limit=100`;
		const response = await fetch(url);
		const data: BlocksResponse = await response.json();
		let blocks = data.blocks;
		while (data.total > blocks.length) {
			const response = await fetch(`${url}&offset=${blocks.length}`);
			const data: BlocksResponse = await response.json();
			blocks = blocks.concat(data.blocks);
		}
		return blocks;
	}

	async function fetchTransactions() {
		if (processPercentage > 0) return;

		let txMap: Record<string, Block[]> = {};
		const totalAddresses = rootAddresses.length;

		for (let i = 0; i < totalAddresses; i++) {
			const { account } = rootAddresses[i];
			const blocks = (await getBlocks(account)).filter((b) => {
				const bigTx = Number(b.amount) / 1_00_000_000 > 0.1;
				const toSelf = b.from_account_identifier === b.to_account_identifier;
				return bigTx && !toSelf;
			}); // Only txs above 1 ICP.
			txMap[account] = blocks;

			processPercentage = Math.round(((i + 1) / totalAddresses) * 100);
			await new Promise((resolve) => setTimeout(resolve, 100)); // 1/4 second
		}

		const { nodes: ns, links: ls } = graphFromTxMap(txMap);
		nodes = ns; // Store for filtering.
		links = ls;

		applyFilter();
		loadingTransactions = false;
	}

	function updateSVG(nodes: Node[], links: Link[]) {
		const svg = newSimulation(nodes, links, 400, 200);
		document.getElementById('graph')?.replaceWith(svg);

		const svgNodes = document.querySelectorAll('.node');
		svgNodes.forEach((node) => {
			node.addEventListener('click', (e) => {
				node.childNodes.forEach((child) => {
					if (child.nodeName === 'circle') {
						const svgNodes = document.querySelectorAll('.node-circle');
						svgNodes.forEach((node) => {
							node.classList.remove('selected');
						});
						(child as SVGCircleElement).classList.add('selected');

						selectedAccount = (child as SVGCircleElement).id;
						nodeProviderData = nodeProviders.find(
							(p) => p.rewards?.reward_account_formatted === selectedAccount
						);
					}
				});
			});
		});
	}

	function graphFromTxMap(txMap: Record<string, Block[]>) {
		let nodes: Node[] = [];
		let links: Link[] = [];
		for (const account of Object.keys(txMap)) {
			if (!nodes.find((n) => n.account === account)) {
				nodes.push({ id: account, account, color: 'red' });
			}
		}
		for (const [account, blocks] of Object.entries(txMap)) {
			for (const block of blocks) {
				switch (block.transfer_type) {
					case 'send': {
						let fromNode = nodes.find((n) => n.account === block.from_account_identifier);
						if (!fromNode) {
							fromNode = {
								id: block.from_account_identifier,
								account: block.from_account_identifier,
								color: getColor(block.from_account_identifier)
							};
							nodes.push(fromNode);
						}
						let toNode = nodes.find((n) => n.account === block.to_account_identifier);
						if (!toNode) {
							toNode = {
								id: block.to_account_identifier,
								account: block.to_account_identifier,
								color: getColor(block.to_account_identifier)
							};
							nodes.push(toNode);
						}

						let link = links.find(
							(l) =>
								(l.source === fromNode && l.target === toNode) ||
								(l.source === toNode && l.target === fromNode)
						);

						// Links are always formulated from the perspective of the node provider account.
						if (!link) {
							const source = fromNode.account === account ? fromNode : toNode;
							const target = fromNode.account === account ? toNode : fromNode;

							const direction =
								block.from_account_identifier === account ? Direction.SEND : Direction.RECEIVE;
							links.push({
								direction,
								source,
								target
							});
						} else {
							const direction =
								block.from_account_identifier === account ? Direction.SEND : Direction.RECEIVE;
							if (direction !== link.direction) {
								link.direction = Direction.BOTH;
							}
						}
						break;
					}
					case 'mint': {
						break; // ignore
					}
					default: {
						console.log('Unknown transfer type:', block.transfer_type);
						break;
					}
				}
			}
		}
		return { nodes, links };
	}

	function filterDeadEnds(nodes: Node[], links: Link[]): { nodes: Node[]; links: Link[] } {
		const filteredNodes = nodes.filter((node) => {
			const linksCount = links.filter(
				(link) => link.source === node || link.target === node
			).length;
			return node.color === 'red' || node.color === 'blue' || 1 < linksCount;
		});
		const filteredLinks = links.filter((link) => {
			return (
				filteredNodes.includes(link.source as Node) && filteredNodes.includes(link.target as Node)
			);
		});
		return { nodes: filteredNodes, links: filteredLinks };
	}

	function selectNodeProvider(event: any) {
		const provider = nodeProviders.find((p) => p.name === event.target.value);
		if (provider && provider.rewards) {
			selectedAccount = provider.rewards?.reward_account_formatted;
			nodeProviderData = provider;

			const svgNodes = document.querySelectorAll('.node-circle');
			svgNodes.forEach((node) => {
				node.classList.remove('selected');
				if (node.id === selectedAccount) {
					node.classList.add('selected');
					const x = Number(node.getAttribute('cx'));
					const y = Number(node.getAttribute('cy'));
					const svg = document.getElementById('graph');
					const links = svg?.querySelector('.links');
					const nodes = svg?.querySelector('.nodes');
					if (links && nodes) {
						links.setAttribute('transform', `translate(${200 - x}, ${100 - y})`);
						nodes.setAttribute('transform', `translate(${200 - x}, ${100 - y})`);
					}
				}
			});
		}
	}

	function toggleFilterDeadEnds() {
		filterDeadEndsEnabled = !filterDeadEndsEnabled;
		applyFilter();
	}

	function toggleKnownExchanges() {
		filterKnownExchangesEnabled = !filterKnownExchangesEnabled;
		applyFilter();
	}

	function toggleFilterUnconnectedNodes() {
		filterUnconnectedNodesEnabled = !filterUnconnectedNodesEnabled;
		applyFilter();
	}

	function applyFilter() {
		let ns = nodes;
		let ls = links;

		if (filterKnownExchangesEnabled) {
			ns = ns.filter((node) => node.color !== 'blue');
			ls = ls.filter((link) => {
				return ns.includes(link.source as Node) && ns.includes(link.target as Node);
			});
		}

		if (filterDeadEndsEnabled) {
			const { nodes: filteredNodes, links: filteredLinks } = filterDeadEnds(ns, ls);
			ns = filteredNodes;
			ls = filteredLinks;
		}

		if (filterUnconnectedNodesEnabled) {
			ns = ns.filter((node) => {
				const linksCount = ls.filter((link) => link.source === node || link.target === node).length;
				return 0 < linksCount;
			});
			ls = ls.filter((link) => {
				return ns.includes(link.source as Node) && ns.includes(link.target as Node);
			});
		}

		updateSVG(ns, ls);
		resetSelection();
	}

	function resetSelection() {
		selectedAccount = null;
		nodeProviderData = null;
		const svgNodes = document.querySelectorAll('.node-circle');
		svgNodes.forEach((node) => {
			node.classList.remove('selected');
		});
		const select = document.querySelector('select');
		if (select) {
			select.value = '';
		}
	}

	onMount(() => {
		fetchTransactions();
	});
</script>

<div>
	<h1>Internet Computer Node Providers</h1>
	<a href="/">home</a>
	<p>
		NOTE: All transactions are fetched from the <a
			href="https://dashboard.internetcomputer.org"
			target="_blank">dashboard</a
		>, not live from on-chain.
	</p>
	<p>
		NOTE: The list of known exchange addresses is not exhaustive. If you know of an exchange address
		that is not listed, please let us know.
	</p>
	<br />

	{#if loadingTransactions}
		<p>
			Fetching transactions... {processPercentage}% complete. This can take a while... DO NOT
			REFRESH THE PAGE!
		</p>
	{:else}
		<select on:change={selectNodeProvider}>
			<option value="">Select a node provider</option>
			{#each nodeProviders as provider}
				<option value={provider.name}>{provider.name}</option>
			{/each}
		</select>
		<br />
		<button on:click={toggleFilterDeadEnds}>
			{#if filterDeadEndsEnabled}
				Show Dead Ends
			{:else}
				Filter Dead Ends
			{/if}
		</button>
		<button on:click={toggleKnownExchanges}>
			{#if filterKnownExchangesEnabled}
				Show Known Exchanges
			{:else}
				Filter Known Exchanges
			{/if}
		</button>
		<button on:click={toggleFilterUnconnectedNodes}>
			{#if filterUnconnectedNodesEnabled}
				Show Unconnected Nodes
			{:else}
				Filter Unconnected Nodes
			{/if}
		</button>
	{/if}
	<br />
	<div id="layout">
		<div id="svg-container">
			<svg id="graph"></svg>
		</div>

		<div id="details">
			{#if selectedAccount}
				<p>
					Selected Account: <a
						href="https://dashboard.internetcomputer.org/account/{selectedAccount}"
						target="_blank">{selectedAccount}</a
					>
				</p>
				{#if knownAddresses[selectedAccount]}
					<p>Name: {knownAddresses[selectedAccount]}</p>
				{/if}
				{#if nodeProviderData}
					<p>
						Principal ID: <a
							href="https://dashboard.internetcomputer.org/provider/{nodeProviderData?.principal_id}"
							>{nodeProviderData?.principal_id}</a
						>
					</p>
					<p>Provider: {nodeProviderData?.name}</p>
				{/if}
			{/if}
			<div id="color-legend">
				<h3>Color Legend</h3>
				<ul>
					<li>
						<span style="color: green;">○</span>: Selected Account
					</li>
					<li>
						<span style="color: red;">○</span>: Node Provider Reward Account
					</li>
					<li>
						<span style="color: blue;">○</span>: Known Exchange Account?<small
							style="font-size: .6em">* based on volume/txs</small
						>
					</li>
				</ul>
			</div>
		</div>
	</div>
</div>

<style>
	#layout {
		display: flex;
		gap: 1rem;
	}

	#svg-container {
		flex: 2;
		border: 1px solid black;
		min-width: 100vh;
	}

	#details {
		flex: 1;
		border: 1px solid #ccc;
		padding: 1rem;
		background-color: #f9f9f9;
	}

	#color-legend {
		margin-top: 1rem;
		font-size: 0.8rem;
	}

	* {
		margin: 0;
		padding: 0;
	}

	li {
		list-style-type: none;
	}

	:global(.selected) {
		stroke: green;
	}

	select {
		padding: 10px;
		border-radius: 5px;
		border: 1px solid #ccc;
		margin: 10px 0;
	}

	button {
		background-color: #f1f1f1;
		border: none;
		padding: 10px 20px;
		cursor: pointer;
		border-radius: 5px;
		margin: 10px 0;
	}

	a:visited {
		color: black;
		text-decoration: none;
	}
</style>
