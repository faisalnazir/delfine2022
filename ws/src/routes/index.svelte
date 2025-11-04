<script context="module">
	export const prerender = false;
</script>

<script>
	import { afterUpdate, beforeUpdate, onMount, tick } from 'svelte';
	import { browser } from '$app/env';
	import Stats from '../components/stats.svelte';
	import { BigNumber } from 'bignumber.js';

	// Solana imports
	import { Connection, PublicKey, LAMPORTS_PER_SOL, Transaction, SystemProgram } from '@solana/web3.js';
	import { AnchorProvider, Program, web3 } from '@project-serum/anchor';
	import { getAssociatedTokenAddress, getAccount, TOKEN_PROGRAM_ID, ASSOCIATED_TOKEN_PROGRAM_ID } from '@solana/spl-token';

	// Wallet adapter imports - will be initialized in layout
	import { workSpace } from '../stores/solanaStore';

	let address;
	let solBalance = 0;
	let delfBalance = 0;
	let remainingTokens = 0;
	let tokenamount = 0;
	let buyTokensmessage = '';
	let tokenRateMessage = '';
	let exchangeRate = 1000; // 1000 DELF per 1 SOL
	let myDelfs = 0;
	let connected = false;
	let wallet;
	let connection;
	let provider;
	let program;

	// Solana Configuration
	const NETWORK = 'devnet'; // Change to 'mainnet-beta' for production
	const RPC_ENDPOINT = 'https://api.devnet.solana.com';

	// Program IDs - Update these after deployment
	const PROGRAM_ID = new PublicKey('De1FiNexxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxx');
	const DELF_MINT = new PublicKey('YOUR_DELF_MINT_ADDRESS_HERE');

	// Initialize Solana connection
	if (browser) {
		connection = new Connection(RPC_ENDPOINT, 'confirmed');

		// Subscribe to wallet changes
		workSpace.subscribe(value => {
			if (value?.wallet && value?.connection) {
				wallet = value.wallet;
				connection = value.connection;
				provider = value.provider;
				program = value.program;
				connected = !!wallet.publicKey;

				if (connected) {
					address = wallet.publicKey.toString();
					updateBalances();
				}
			}
		});
	}

	// Get SOL balance
	const getSolBalance = async () => {
		if (!wallet?.publicKey) return 0;

		try {
			const balance = await connection.getBalance(wallet.publicKey);
			return balance / LAMPORTS_PER_SOL;
		} catch (error) {
			console.error('Error getting SOL balance:', error);
			return 0;
		}
	};

	// Get DELF token balance
	const getDelfBalance = async () => {
		if (!wallet?.publicKey) return 0;

		try {
			const associatedTokenAddress = await getAssociatedTokenAddress(
				DELF_MINT,
				wallet.publicKey
			);

			const tokenAccount = await getAccount(connection, associatedTokenAddress);
			return Number(tokenAccount.amount) / Math.pow(10, 9); // 9 decimals
		} catch (error) {
			console.error('Error getting DELF balance:', error);
			return 0;
		}
	};

	// Get remaining tokens in crowdsale
	const getRemainingTokens = async () => {
		if (!program) return 0;

		try {
			// Get config account
			const [configPda] = await PublicKey.findProgramAddress(
				[Buffer.from('config')],
				PROGRAM_ID
			);

			const config = await program.account.config.fetch(configPda);

			// Get crowdsale vault balance
			const vaultAddress = await getAssociatedTokenAddress(
				DELF_MINT,
				configPda,
				true
			);

			const vaultAccount = await getAccount(connection, vaultAddress);
			return Number(vaultAccount.amount) / Math.pow(10, 9);
		} catch (error) {
			console.error('Error getting remaining tokens:', error);
			return 0;
		}
	};

	// Update all balances
	const updateBalances = async () => {
		if (!connected) return;

		solBalance = await getSolBalance();
		delfBalance = await getDelfBalance();
		myDelfs = delfBalance;
		remainingTokens = await getRemainingTokens();
	};

	// Buy DELF tokens with SOL
	const buyTokens = async () => {
		buyTokensmessage = '';

		if (!connected) {
			buyTokensmessage = 'Please connect your wallet first';
			return;
		}

		if (!tokenamount || tokenamount <= 0) {
			buyTokensmessage = 'Please enter a valid amount';
			return;
		}

		if (parseFloat(tokenamount) > parseFloat(solBalance)) {
			buyTokensmessage = "You don't have enough SOL - Your transaction will fail";
			return;
		}

		try {
			buyTokensmessage = 'Processing transaction...';

			// Convert SOL amount to lamports
			const solAmountLamports = tokenamount * LAMPORTS_PER_SOL;

			// Get PDAs
			const [configPda] = await PublicKey.findProgramAddress(
				[Buffer.from('config')],
				PROGRAM_ID
			);

			// Get associated token addresses
			const crowdsaleVault = await getAssociatedTokenAddress(
				DELF_MINT,
				configPda,
				true
			);

			const buyerTokenAccount = await getAssociatedTokenAddress(
				DELF_MINT,
				wallet.publicKey
			);

			const config = await program.account.config.fetch(configPda);

			// Call the buy_delf_tokens instruction
			const tx = await program.methods
				.buyDelfTokens(new web3.BN(solAmountLamports))
				.accounts({
					config: configPda,
					delfMint: DELF_MINT,
					crowdsaleVault: crowdsaleVault,
					buyerTokenAccount: buyerTokenAccount,
					buyer: wallet.publicKey,
					authority: config.authority,
					systemProgram: SystemProgram.programId,
					tokenProgram: TOKEN_PROGRAM_ID,
					associatedTokenProgram: ASSOCIATED_TOKEN_PROGRAM_ID,
					rent: web3.SYSVAR_RENT_PUBKEY,
				})
				.rpc();

			buyTokensmessage = `Success! Transaction: ${tx}`;

			// Update balances after purchase
			await updateBalances();

			// Clear the input
			tokenamount = 0;

		} catch (error) {
			console.error('Error buying tokens:', error);
			buyTokensmessage = `Error: ${error.message}`;
		}
	};

	// Calculate token rate message
	$: {
		if (tokenamount && !isNaN(tokenamount) && tokenamount > 0) {
			tokenRateMessage = `equals ${(tokenamount * exchangeRate).toLocaleString()} DELF`;
		} else {
			tokenRateMessage = '';
		}
	}

	// Initialize on mount
	onMount(async () => {
		if (connected) {
			await updateBalances();
		}
	});

	// Update after changes
	afterUpdate(async () => {
		if (connected) {
			await updateBalances();
		}
	});
</script>

<svelte:head>
	<title>Delfine - Home</title>
</svelte:head>

<section>
	<div class="hero min-h-screen bg-base-100">
		<div class="text-center hero-content">
			<div class="max-w-md">
				<img
					src="https://images.unsplash.com/photo-1559102877-4a2cc0e37fce?ixlib=rb-1.2.1&ixid=MnwxMjA3fDB8MHxwaG90by1wYWdlfHx8fGVufDB8fHx8&auto=format&fit=crop&w=1057&q=80"
					class="rounded-lg shadow-xl mb-10"
					alt="art"
				/>
				<h1 class="mt-15 mb-5 text-6xl text-bold font-display">Invest in the Future of Art</h1>
				<p class="mb-5 text-xl">
					Buy Delfine tokens using this invitation only website. Invest and be at the front of the
					queue for the next movement to shake the world of art.
				</p>
				<a class="btn btn-primary text-white" href="/#buyDelf">Get Started</a>
			</div>
		</div>
	</div>

	<div id="buyDelf" class="modal min-h-screen min-w bg-base-300 flex flex-col">
		<div
			class="mt-12 font-extrabold flex flex-col justify-center items-center prose lg:prose-xl text-xl"
		>
			<h2 class="badge badge-outline badge-lg badge-success absolute top-3 right-3">
				{connected ? `Solana ${NETWORK}` : 'Not Connected'}
			</h2>
			<div class="badge badge-lg flex flex-auto absolute top-3 left-3">
				DELFINE PRIVE TOKEN SALE
			</div>
			{#if connected}
				<p class="badge-outline badge absolute bottom-3 left-3 badge-xl">
					Address: {address?.slice(0, 4)}...{address?.slice(-4)}
				</p>
			{:else}
				<p class="badge-outline badge absolute bottom-3 left-3 badge-xl">
					Please connect your Solana wallet
				</p>
			{/if}
		</div>

		<Stats tokenFloat={remainingTokens} account_balance={solBalance} myDelfs={myDelfs} />

		<p class="flex mt-12 justify-center text-primary-focus text-xl">
			Purchase Delfine Tokens using your Solana wallet with the form below.
		</p>

		{#if connected}
			<form on:submit|preventDefault={buyTokens} class="m-2 p-2 text-xl">
				<input
					bind:value={tokenamount}
					type="number"
					step="0.01"
					min="0"
					class="w-full input input-primary input-bordered"
					placeholder="Amount of SOL to spend"
				/>

				<button class="mt-1 btn btn-accent w-full justify-center text-xl" type="submit">
					BUY DELF Tokens with SOL
				</button>

				{#if buyTokensmessage !== ''}
					<div
						class={`alert ${buyTokensmessage.includes('Success') ? 'alert-success' : 'alert-error'} mt-2`}
					>
						{buyTokensmessage}
					</div>
				{/if}
			</form>

			{#if tokenRateMessage !== ''}
				<div class="alert alert-info text-2xl">
					{tokenRateMessage}
				</div>
			{/if}
		{:else}
			<div class="alert alert-warning m-4 text-xl">
				Please connect your Solana wallet using the button in the navigation bar.
			</div>
		{/if}
	</div>
</section>

<style>
	.modal {
		display: block;
	}
</style>
