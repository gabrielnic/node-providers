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
	let filterFoundationAccountsEnabled = false;
	let filterUnconnectedNodesEnabled = true;
	let nodes: Node[] = [];
	let links: Link[] = [];

	// Thanks @borovan and others for the list of addresses:
	// https://forum.dfinity.org/t/node-provider-icp-address-graph/42655
	const foundationAccounts = [
		'1a481b7aa86fa89029117f0487479f52fe3ee27ad63d48bac78fa60b3ebf9237',
		'12b7c1c6269b2021758ed5da65448a4ac3ac9fa0cf015caa4bb4c3e3dc7ca1c1',
		'21b3cb5fccbbb1b4d92c03ace6f16f836e3425cf61ac3b2a9823499a60d5c7b5',
		'36039b216d234b39bfc993df251deba6d7481d974f20a7aa4ea8a2aff8c7606e',
		'406ae771636e9e6501166f71edb0b61b80a325640048b11b23b3eaf43a5048ed',
		'5495612bb400e0dbd51ca4ae28835b3c47f6968127fe54d4aadc8704cc363057',
		'54f4a6d3bc831c5217e4e313bca7f2549f6b51b9dc25d77ae045bbb59c8eafaf',
		'57c9e0f1111d0aba921adf0056a16771e9a4fd84a6daee21267511b2b3410731',
		'581ebcfa72dbed72cb0d18240d30879ac915df69dba3d1a8cfbb5508bb973df1',
		'600bc2e6fa9dcf3543acc22bd8181ca7e11ef3f0a9ce662480fe61524c9bb8c1',
		'61529f442b6fc6a2db07f4dc446b255e6702aa95ed8bb2fa5c38cb04358eea65',
		'65675acf9e0752bbc58893b284e2c7558db61bfdbefe63c055bb69305f0da17e',
		'6a6fe5e9936747faadb472f3d3790830c372eb4ab7039d207059bf34eacf4de9',
		'78a6c47cc1e153e1b63eafcd471d5e6de1365592e73d0c41931461fd97271d03',
		'860c10fd2e96106edccb48e657b265517fda354a4588a52a13384dd58ffcaab2',
		'91ee71b84b7603de21bcf6cc1857a7cb91d12062d91f95013d300e0eb35e6d52',
		'ad2d4c5c3b70fa1289c7b45ef252d608f8d70f0d9b8198d6a6d0c4f5bdfc9c65',
		'b0c25df9be777bb84a2b8ddec02bbf42bfef588c44bc7fd483a515725cd68830',
		'b30d4f02181bd81dbe5ef9f22b33a7d7a5fe61884964fc697721ca259cfd3c41',
		'b93164c6ae75984345a3d47fa9877749acb19fee032ec6aac67cb3ee6100f302',
		'b9cdb3a04e388736de3eea9426f4f83e79c90a6602b0805746ff7991e546df9f',
		'd5336412e2107f4b0502234128dbc11ccf53221ae67bae5109eb4be11bb9babb',
		'd539266d9e7784304ceb7b72a729794004646e4a39a56b51c3e911a698bbdd8d',
		'f7d23ad118bab9eae59055a98addd2a1738cc281fa1dc7ca4568e8e661e21283',
		'62dd39780d34f2dc21eb680f99962659a6a0c2ccd9c68ec365962ae0eaf293f0'
	];
	const knownCEXAccounts: Record<string, string> = {
		'609d3e1e45103a82adc97d4f88c51f78dedb25701e8e51e8c4fec53448aadc29': 'Binance 1',
		'220c3a33f90601896e26f76fa619fe288742df1fa75426edfaf759d39f2455a5': 'Binance 2',
		d3e13d4777e22367532053190b6c6ccf57444a61337e996242b1abfb52cf92c8: 'Binance 3',
		'449ce7ad1298e2ed2781ed379aba25efc2748d14c60ede190ad7621724b9e8b2': 'Coinbase 1',
		'4dfa940def17f1427ae47378c440f10185867677109a02bc8374fc25b9dee8af': 'Coinbase 2',
		a6ed987d89796f921c8a49d275ec7c9aa04e75a8fc8cd2dbaa5da799f0215ab0: 'Coinbase 3',
		'660b1680dafeedaa68c1f1f4cf8af42ed1dfb8564646efe935a2b9a48528b605': 'Coinbase 4',
		dd15f3040edab88d2e277f9d2fa5cc11616ebf1442279092e37924ab7cce8a74: 'Coinbase 5',
		'4878d23a09b554157b31323004e1cc053567671426ca4eec7b7e835db607b965': 'Coinbase 6',
		'8fe706db7b08f957a15199e07761039a7718937aabcc0fe48bc380a4daf9afb0': 'Gate.io',
		e7a879ea563d273c46dd28c1584eaa132fad6f3e316615b3eb657d067f3519b5: 'OKX 1',
		d2c6135510eaf107bdc2128ef5962c7db2ae840efdf95b9395cdaf4983942978: 'OKX 2',
		'040834c30cdf5d7a13aae8b57d94ae2d07eefe2bc3edd8cf88298730857ac2eb': 'Kraken',
		'935b1a3adc28fd68cacc95afcdec62e985244ce0cfbbb12cdc7d0b8d198b416d': 'Houbi',
		efa01544f509c56dd85449edf2381244a48fad1ede5183836229c00ab00d52df: 'KuCoin',
		'00c3df112e62ad353b7cc7bf8ad8ce2fec8f5e633f1733834bf71e40b250c685': 'KuCoin 2',
		acd76fff0536f863d9dd4b326a1435466f82305758b4b1b4f62ff9fa81c14073: 'Bybit',
		bad030b417484232fd2019cb89096feea3fdd3d9eb39e1d07bcb9a13c7673464: 'Bitget',
		'9e62737aab36f0baffc1faac9edd92a99279723eb3feb2e916fa99bb7fe54b59': 'MEXC'
	};

	function getColor(account: string): string | undefined {
		if (knownCEXAccounts[account]) {
			return 'blue';
		}
		if (foundationAccounts.includes(account)) {
			return 'orange';
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

	function toggleFoundationAccounts() {
		filterFoundationAccountsEnabled = !filterFoundationAccountsEnabled;
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

		if (filterFoundationAccountsEnabled) {
			ns = ns.filter((node) => node.color !== 'orange');
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
		<button on:click={toggleFoundationAccounts}>
			{#if filterFoundationAccountsEnabled}
				Show Foundation Accounts
			{:else}
				Filter Foundation Accounts
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
				{#if knownCEXAccounts[selectedAccount]}
					<p>CEX Name: {knownCEXAccounts[selectedAccount]}</p>
				{/if}
				{#if foundationAccounts.includes(selectedAccount)}
					<p>Foundation Account</p>
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
		border: 1px solid #ccc;
		border-radius: 5px;
		padding: 1rem;
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
