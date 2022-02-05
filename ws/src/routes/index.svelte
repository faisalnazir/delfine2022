<script context="module">
	export const prerender = false;
</script>

<script>
	// IMPORT THESE
	import { afterUpdate, beforeUpdate, onMount, tick } from 'svelte';
	import { browser } from '$app/env';
	import DELF from '../../../build/contracts/DELF.json';
	import CrowdSale from '../../../build/contracts/CrowdSale.json';
	import Stats from '../components/stats.svelte';
	import {
		defaultEvmStores,
		web3,
		selectedAccount,
		connected,
		chainId,
		chainData
	} from 'svelte-web3';
	import { BigNumber } from 'bignumber.js';
	// This will only render client-side if the browser is available.
	if (browser) {
		defaultEvmStores.setBrowserProvider();
	}

	let address;
	let myNativeBalance;
	let result;
	let delfbalance;
	let remainingTokens;
	let delfc;
	let crowdsalec;
	let crowdsalecd;
	let tokens;
	let tokenamount = 0;
	let tokenwei;
	let tokenFloat;
	let account_balance;
	let buyTokensmessage;
	let tokenRateMessage = '';
	let bgTokens;
	let myDelfs;
	// Make sure this is the same exchange rate as per the contract
	const exchangeRate = 1000;

	// MUMBAI MATIC SMART CONTRACTS
	const delfContractAddress = '0x4bFd45feDb16Dc16f4a9073dA904Ad9A28Dd3510';
	const crowdsaleContractAddress = '0x42745e0c8E2A39023516B0f99c00A61804537250';

	// Global contracts don't seem to work for some reason
	// crowdsalec = new $web3.eth.Contract(CrowdSale.abi, crowdsaleContractAddress);
	// delfc = new $web3.eth.Contract(DELF.abi, delfContractAddress);

	let buyTokens = async () => {
		if (parseFloat(tokenamount) > parseFloat(myNativeBalance)) {
			buyTokensmessage = "You don't have enougth - Your transaction will fail";
		} else {
			delfc = new $web3.eth.Contract(DELF.abi, delfContractAddress);
			crowdsalec = new $web3.eth.Contract(CrowdSale.abi, crowdsaleContractAddress);
			tokenwei = $web3.utils.toWei(tokenamount, 'ether');

			await crowdsalec.methods
				.buyTokens($selectedAccount)
				.send({ value: tokenwei, from: $selectedAccount })
				.then(function (result) {
					console.log(result);
				});
		}
	};

	const getDelf = async () => {
		crowdsalecd = new $web3.eth.Contract(CrowdSale.abi, crowdsaleContractAddress);
		await crowdsalecd.methods
			.getRemainingTokens()
			.call()
			.then(function (result) {
				remainingTokens = result;
			});
		tokenFloat = parseFloat(remainingTokens) / 10 ** 18;
		bgTokens = new BigNumber(tokenFloat);
		return bgTokens.toFormat(0);
	};

	const getMyDelfs = async () => {
		let delfcon = new $web3.eth.Contract(DELF.abi, delfContractAddress);
		await delfcon.methods
			.balanceOf($selectedAccount)
			.call({ from: $selectedAccount })
			.then(function (result) {
				result = result / 10 ** 18;
				myDelfs = result;
			});
	};
	// getMyDelfs();
	const query_balance = async (address) => {
		if ($web3.utils.isAddress(address)) {
			result = $web3.utils.fromWei(await $web3.eth.getBalance(address));
			return result;
		} else {
			result = 'Not a valid address. Please add in your address';
		}
	};

	const get_balance = async (_address) => {
		if ($web3.utils.isAddress(_address)) {
			return (
				$web3.utils
					.fromWei(await $web3.eth.getBalance(_address))
					.toString()
					.substring(0, 6) +
				' ' +
				$chainData?.nativeCurrency?.symbol
			);
		} else {
			return '...';
		}
	};

	$: tokenFloat;
	tokenRateMessage = '';
	$: {
		tokenamount;

		if (tokenamount != NaN) {
			tokenRateMessage = 'equals ' + tokenamount * exchangeRate + ' DELFS';
		}
	}

	onMount(async () => {
		tick();
		account_balance = await get_balance($selectedAccount).then(function (result) {
			return result;
		});
		tick();
		tokenFloat = await getDelf().then(function (result) {
			// console.log(result);
			return result;
		});
		tick();
		await getMyDelfs().then(function (result) {
			console.log(result);
			// return result;
		});
		tick();
		myNativeBalance = await query_balance($selectedAccount).then(function (result) {
			return result;
		});
		tick();
		console.log('My Native Balance', myNativeBalance);
	});

	afterUpdate(async () => {
		account_balance = await get_balance($selectedAccount).then(function (result) {
			return result;
		});
		tokenFloat = await getDelf().then(function (result) {
			// console.log(result);
			return result;
		});
		await getMyDelfs().then(function (result) {
			return result;
		});
		myNativeBalance = await query_balance($selectedAccount).then(function (result) {
			return result;
		});
	});

	$: tokenFloat;
	$: remainingTokens;
	$: bgTokens;
	$: myDelfs;
</script>

<svelte:head>
	<title>Home</title>
</svelte:head>

<section>
	<div class="hero min-h-screen bg-base-100">
		<div class="text-center hero-content">
			<div class="max-w-md">
				<img
					src="https://images.unsplash.com/photo-1559102877-4a2cc0e37fce?ixlib=rb-1.2.1&ixid=MnwxMjA3fDB8MHxwaG90by1wYWdlfHx8fGVufDB8fHx8&auto=format&fit=crop&w=1057&q=80"
					class=" rounded-lg shadow-xl mb-10"
					alt="art"
				/>
				<h1 class="mt-15 mb-5 text-6xl text-bold font-display">Invest in the Future of Art</h1>
				<p class="mb-5 text-xl">
					Buy Delfine tokens using this invation only website. Invest and be at the front of the
					queue for the next movement to shake the world of art.
				</p>
				<a class="btn btn-primary text-white" href="/#buyDelf">Get Started</a>
			</div>
		</div>
	</div>
	<!-- NOTE: this codeblock includes Tailwind CSS classes -->
	<div id="buyDelf" class="modal min-h-screen min-w bg-base-300 flex flex-col">
		<div
			class="mt-12 font-extrabold flex flex-col justify-center items-center prose lg:prose-xl text-xl"
		>
			<h2 class="badge badge-outline badge-lg badge-success absolute top-3 right-3">
				{$chainData?.name || 'Ganache Development'}
			</h2>
			<div class="badge badge-lg flex flex-auto absolute top-3 left-3">
				DELFINE PRIVE TOKEN SALE
			</div>
			<!-- <div class="badge badge-lg badge-primary">
				Native Currency: {$chainData?.nativeCurrency?.name || 'Dev ETH'} ({$chainData
					?.nativeCurrency?.symbol || 'ETH'})
			</div> -->
			<p class="badge-outline badge absolute bottom-3 left-3 badge-xl">
				Address:{$selectedAccount}
			</p>
		</div>
		<Stats {tokenFloat} {account_balance} {myDelfs} />
		<p class="flex mt-12 justify-center text-primary-focus text-xl">
			Purchase Delfine Tokens using MetaMask with the form below.
		</p>
		<form on:submit|preventDefault={buyTokens} class="m-2 p-2 text-xl">
			<input
				bind:value={tokenamount}
				type="text"
				class="w-full input input-primary input-bordered"
				placeholder="Add number of {$chainData?.nativeCurrency?.name} here"
			/>

			<button class="mt-1 btn btn-accent w-full justify-center text-xl" type="submit">
				BUY DELF Tokens with {$chainData?.nativeCurrency?.name || 'Dev ETH'}</button
			>

			{#if buyTokensmessage != ''}
				<div
					class="alert alert-error"
					bind:textContent={buyTokensmessage}
					contenteditable="false"
				/>
			{/if}
		</form>
		{#if tokenRateMessage != NaN}
			<div
				class="alert alert-information text-2xl"
				bind:textContent={tokenRateMessage}
				contenteditable="false"
			/>
		{/if}
	</div>
</section>
