<script>
	import '../app.css';
	import { onMount } from 'svelte';
	import { browser } from '$app/env';
	import Nav from '../components/nav.svelte';
	import { workSpace } from '../stores/solanaStore';

	// Wallet adapter (will be initialized client-side only)
	let walletAdapter;

	onMount(async () => {
		if (browser) {
			try {
				// Dynamically import wallet adapter modules
				const { PhantomWalletAdapter } = await import('@solana/wallet-adapter-wallets');

				// Initialize Phantom wallet adapter
				walletAdapter = new PhantomWalletAdapter();

				// Listen for wallet connection
				walletAdapter.on('connect', () => {
					console.log('Wallet connected:', walletAdapter.publicKey?.toString());
					workSpace.init(walletAdapter);
				});

				// Listen for wallet disconnection
				walletAdapter.on('disconnect', () => {
					console.log('Wallet disconnected');
					workSpace.disconnect();
				});

			} catch (error) {
				console.error('Error initializing wallet adapter:', error);
			}
		}
	});
</script>

<Nav {walletAdapter} />
<body />

<div class="p-4 max-w-6xl mx-auto">
	<slot />
</div>

<style>
</style>
