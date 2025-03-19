<script lang="ts">
	import nodeProvidersData from './combined_providers.json';
	import nodeStakeData from './stake.json';

	let nodeProviders = nodeProvidersData;
	const nodeStake = new Map(nodeStakeData.map((provider) => [provider.toml_id, provider.stake]));

	let sortKey: 'name' | 'latest' | 'node_count' | 'total' | 'start' = 'name';
	let sortOrder = 1; // 1 for ascending, -1 for descending

	function sortBy(key: 'name' | 'latest' | 'node_count' | 'total' | 'start') {
		if (sortKey === key) {
			sortOrder *= -1;
		} else {
			sortKey = key;
			sortOrder = 1;
		}

		nodeProviders = [...nodeProviders].sort((a, b) => {
			let valA: number | string = 0;
			let valB: number | string = 0;

			if (key === 'name') {
				valA = a[key] ?? '';
				valB = b[key] ?? '';
				return (valA as string).localeCompare(valB as string) * sortOrder;
			}

			if (key === 'latest') {
				valA = a.rewards?.most_recent_reward_e8s ?? 0;
				valB = b.rewards?.most_recent_reward_e8s ?? 0;
			} else if (key === 'node_count') {
				valA = a.total_rewardable_nodes ?? 0;
				valB = b.total_rewardable_nodes ?? 0;
			} else if (key === 'total') {
				valA = a.rewards?.total_mint_rewards_icp ?? 0;
				valB = b.rewards?.total_mint_rewards_icp ?? 0;
			} else if (key === 'start') {
				valA = a.rewards?.first_mint_timestamp ?? 0;
				valB = b.rewards?.first_mint_timestamp ?? 0;
			}

			return (((valA as number) - valB) as number) * sortOrder;
		});
	}

	$: nodeProviders;
</script>

<h1>Internet Computer Node Provider Verification Status</h1>

<table>
	<thead>
		<tr>
			<th><a on:click={() => sortBy('name')}>Provider Name</a></th>
			<th>Wiki Link</th>
			<th>Documents (✔️ = hash match | ❌ = hash don't match)</th>
			<th><a on:click={() => sortBy('node_count')}>Total Rewardable Nodes</a></th>
			<th><a on:click={() => sortBy('latest')}>Last Rewards (ICP)</a></th>
			<th><a on:click={() => sortBy('total')}>Total Rewards (ICP)</a></th>
			<th><a on:click={() => sortBy('start')}>First Rewards Date</a></th>
			<th>Last Rewards Date</th>
			<th>Network Stake</th>
		</tr>
	</thead>
	<tbody>
		{#each nodeProviders as nodeProvider}
			<tr>
				<td><a href={nodeProvider.dashboard_link} target="_blank">{nodeProvider.name}</a></td>
				<td><a href={nodeProvider.wiki_link} target="_blank">{nodeProvider.name}</a></td>
				<td style="display: flex; gap: 1em;">
					{#each nodeProvider.document_validations as document}
						<a href={document.file_path} target="_blank">
							{document.document_type}
							{document.matches ? '✔️' : '❌'}
						</a>
					{/each}
				</td>
				<td>{nodeProvider.total_rewardable_nodes}</td>
				{#if nodeProvider.rewards}
					<td>
						<a href={nodeProvider.rewards.reward_account_dashboard_link} target="_blank">
							{Math.floor(
								nodeProvider.rewards.most_recent_reward_e8s / 100_000_000
							).toLocaleString()}
						</a>
					</td>
					<td>
						<a href={nodeProvider.rewards.reward_account_dashboard_link} target="_blank">
							{Math.floor(nodeProvider.rewards.total_mint_rewards_icp).toLocaleString()}
						</a>
					</td>
					<td>{new Date(nodeProvider.rewards['first_mint_timestamp'] * 1000).toLocaleString()}</td>
					<td>{new Date(nodeProvider.rewards['last_mint_timestamp'] * 1000).toLocaleString()}</td>
				{/if}
				<td>
					{#if nodeProvider.toml_id && nodeStake.get(nodeProvider.toml_id)}
						{@html nodeStake.get(nodeProvider.toml_id)}
					{:else}
						?
					{/if}
				</td>
			</tr>
		{/each}
	</tbody>
</table>

<a href="https://github.com/internet-computer/node-providers" target="_blank">contribute ❤️</a>

<style>
	a {
		text-decoration: underline;
	}
	h1 {
		font-family: Manrope, sans-serif;
		margin: 0;
	}

	table {
		width: 100%;
		border-collapse: collapse;
		margin-top: 20px;
		font-size: 16px;
		text-align: left;
	}

	th,
	td {
		padding: 12px;
		border-bottom: 1px solid #ddd;
	}

	th {
		background-color: #f8f9fa;
		color: #333;
		font-weight: bold;
	}

	td {
		color: #333;
	}

	tr:nth-child(even) {
		background-color: #f2f2f2;
	}

	tr:hover {
		background-color: #e9ecef;
	}
</style>
