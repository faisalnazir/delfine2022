<script>
	import { browser } from '$app/env';
	import { workSpace } from '../stores/solanaStore';

	export let walletAdapter;

	let connected = false;
	let walletAddress = '';

	// Subscribe to workspace changes
	if (browser) {
		workSpace.subscribe((value) => {
			if (value?.wallet?.publicKey) {
				connected = true;
				const addr = value.wallet.publicKey.toString();
				walletAddress = `${addr.slice(0, 4)}...${addr.slice(-4)}`;
			} else {
				connected = false;
				walletAddress = '';
			}
		});
	}

	async function connectWallet() {
		if (!walletAdapter) {
			alert('Wallet adapter not initialized. Please refresh the page.');
			return;
		}

		try {
			await walletAdapter.connect();
		} catch (error) {
			console.error('Error connecting wallet:', error);
			alert('Failed to connect wallet. Make sure you have Phantom wallet installed.');
		}
	}

	async function disconnectWallet() {
		if (!walletAdapter) return;

		try {
			await walletAdapter.disconnect();
		} catch (error) {
			console.error('Error disconnecting wallet:', error);
		}
	}
</script>

<nav class="flex justify-between w-full items-center">
	<div class="flex">
		<a class="mx-4 mt-4 text-lg font-display" href="/"> Home</a>
		<a class="mx-4 mt-4 text-lg font-display" href="/dapp">Delfine</a>
		<a class="mx-4 mt-4 text-lg font-display" href="/fines">My Fines</a>
	</div>

	<div class="flex items-center mr-4 mt-4">
		{#if connected}
			<span class="mr-2 badge badge-success">{walletAddress}</span>
			<button class="btn btn-sm btn-outline" on:click={disconnectWallet}>
				Disconnect
			</button>
		{:else}
			<button class="btn btn-sm btn-primary" on:click={connectWallet}>
				Connect Wallet
			</button>
		{/if}
	</div>
</nav>
