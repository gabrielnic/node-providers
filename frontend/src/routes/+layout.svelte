<script>
	import { onMount } from 'svelte';
</script>

<div class="page-container">
	<div class="content-container">
		<slot />
	</div>
</div>

<style>
	/* === Variables === */
	:root {
		--unselected-header-color: #5d6b77;
		--unselected-header-text-color: #c7c7c7;
		--toast-text-color: var(--main-button-text-color);
		--input-border: 1px solid var(--border-color);
		--box-shadow: black 0px 8px 16px;

		--border-size: 1px;

		--main-font: Manrope, sans-serif;
		--secondary-font: Manrope, sans-serif;

		--padding: 8px;
		--padding-0_25x: calc(var(--padding) / 4);
		--padding-0_5x: calc(var(--padding) / 2);
		--padding-1_5x: calc(1.5 * var(--padding));
		--padding-2x: calc(2 * var(--padding));
		--padding-3x: calc(3 * var(--padding));
		--padding-4x: calc(4 * var(--padding));
		--padding-6x: calc(6 * var(--padding));
		--padding-8x: calc(8 * var(--padding));
		--card-background-contrast: var(--main-color);
		--card-background: white;

		--main-color: #7ce7f2;
		--main-color-disabled: #4c66dcbd;
		--main-button-text-color: white;
		--main-button-text-color-disabled: #fefefeb5;
	}

	:global(.main-btn-style) {
		background-color: var(--main-button-color);
		border: none;
		cursor: pointer;
		border-radius: 0.3em;
		transition: all 0.3s ease;
		color: black;
		font-weight: bold;
		padding: 1em;
		display: flex;
		align-items: center;
		justify-content: center;
		border: var(--border-box);
	}

	:global(.main-btn-style:hover) {
		background-color: var(--main-button-color-hover);
		border: var(--main-button-border-hover);
	}

	/* === Animation === */

	:global(.spinner) {
		border: 3px solid;
		border-radius: 50%;
		animation: spin 1s linear infinite;
	}

	@keyframes spin {
		from {
			transform: rotate(0deg);
		}
		to {
			transform: rotate(360deg);
		}
	}

	:root[theme='light'] {
		--important-text-color: black;
		--stake-text-color: black;

		--logo-color: #505050;

		--title-color: black;
		--title-color-transparent: #8c8c8c;

		--border-color: #454545;
		--page-background: rgb(241, 241, 243);
		--background-color: #ffffff;
		--background-color-transparent: #fefefede;
		--switch-background-color: rgba(220, 220, 220, 0.49);

		--bridge-border-color: #a9aef1;
		--bridge-background-color: #171c2803;
		--bridge-hover-color: #42478c;

		--input-color: #ffffff40;

		--text-color: rgb(48, 53, 73);
		--faq-color: black;

		--unstake-selection-color: #283e9521;

		--svg-fill-color: #000000;
		--svg-opposite-color: #b3b3b3;
		--sns-selected-button-color: rgb(107 180 249 / 50%);

		--theme-background-asset-logo: none;
		--theme-border-asset-logo: none;
		--border-box: 1px solid rgb(234, 235, 239);

		--main-button-color: rgb(247, 247, 249);
		--main-button-border-hover: 1px solid black;
	}

	@font-face {
		font-family: 'FunnelDisplay';
		src: url('/FunnelDisplay.ttf') format('truetype');
		font-weight: normal;
		font-style: normal;
	}

	/* @font-face {
		font-family: 'Main';
		src: url('/Akrobat-Black.ttf') format('truetype');
		font-weight: normal;
		font-style: normal;
	}

	@font-face {
		font-family: 'Secondary';
		src: url('/DMSerifText.ttf') format('truetype');
		font-weight: normal;
		font-style: normal;
	} */

	/* === Layout === */
	.page-container {
		display: flex;
		flex-direction: column;
		height: inherit;
		width: 100vw;
		font-family: Manrope, sans-serif;
		background: var(--page-background);
	}

	.page-container::-webkit-scrollbar {
		width: 12px; /* Width of the scrollbar */
		background: radial-gradient(
			farthest-corner circle at 0% 0%,
			rgb(18, 69, 89),
			#0f0f4d
		); /* Match the background gradient */
	}

	.page-container::-webkit-scrollbar-track {
		background: radial-gradient(
			farthest-corner circle at 0% 0%,
			rgb(18, 69, 89),
			#0f0f4d
		); /* Match the background gradient */
	}

	.page-container::-webkit-scrollbar-thumb {
		background-color: rgba(255, 255, 255, 0.5); /* Thumb color with transparency */
		border-radius: 6px; /* Rounded corners for the thumb */
		background-clip: padding-box;
	}

	.page-container::-webkit-scrollbar-corner {
		background: radial-gradient(
			farthest-corner circle at 0% 0%,
			rgb(18, 69, 89),
			#0f0f4d
		); /* Match the background gradient */
	}

	.content-container {
		display: flex;
		flex-direction: column;
		padding: 1em;
	}
</style>
