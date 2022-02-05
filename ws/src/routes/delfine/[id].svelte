<script context="module">
	export async function load({ params }) {
		const url = `https://openaccess-api.clevelandart.org/api/artworks/${params.id}`;
		const res = await fetch(url);
		const delfine = await res.json();
		console.log(delfine);
		return { props: { delfine } };
	}
</script>

<script>
	import { nftdata } from '../../stores/nftstore';
	import { browser } from '$app/env';
	import {
		defaultEvmStores,
		web3,
		selectedAccount,
		connected,
		chainId,
		chainData
	} from 'svelte-web3';
	import FINE from '../../../../build/contracts/FINE.json';
	export let delfine;
	let receiverAddress;
	let buyTokensmessage;
	const fineContractAddress = '0x94D9bFE5Fff9EfFe9ff7F80ED2F4f793e419c4c2';
	let finec;
	let fined = new $web3.eth.Contract(FINE.abi, fineContractAddress);

	if (browser) {
		defaultEvmStores.setBrowserProvider();
	}

	const sendFine = async () => {
		console.log('I am in sendFine');
		if ($web3.utils.isAddress(receiverAddress)) {
			finec = new $web3.eth.Contract(FINE.abi, fineContractAddress);
			console.log('Before Call to FINE');
			await finec.methods
				.safeTransferFrom($selectedAccount, receiverAddress, parseInt(delfine.data.id), 1, 0x786)
				.send({ from: $selectedAccount })
				.then(function (result) {
					console.log(result);
				});
		} else {
			buyTokensmessage = 'This is not a valid address';
		}
	};

	const getOwner = async () => {
		console.log('I am about to find the owner');

		console.log('about to get owner', fined);
		await fined.methods
			.balanceOf($selectedAccount, delfine.data.id)
			.call({ from: $selectedAccount })
			.then(function (result) {
				isOwner = result;
			});
	};
	let isOwner;
	$: {
		getOwner().then(function (result) {});
	}
</script>

<!-- 
<h1>{delfine.data.title}</h1>
<h2>{delfine.data.tombstone}</h2>

<img src={delfine.data.images.web.url} alt="" /> -->
<div class="badge badge-md badge-accent flex flex-auto absolute top-5 right-3 font-extrabold">
	{$selectedAccount}
</div>
{#if isOwner == '1'}
	<div class="badge badge-md badge-secondary flex flex-auto absolute top-12 right-3 font-extrabold">
		Owner
	</div>
{/if}
<div class="hero min-h-screen bg-base-200">
	<div class="flex-col hero-content lg:flex-col">
		<img
			src={delfine.data.images.web.url}
			class="max-w-4xl rounded-lg shadow-2xl scale-60"
			alt={delfine.data.title}
		/>
		<div>
			<h1 class="mb-5 text-3xl font-bold font-display">{delfine.data.title}</h1>
			{#if isOwner == '1'}
				<div class="badge badge-lg badge-accent mb-6 flex flex-auto font-extrabold">Owner</div>
			{/if}
			<p class="text-lg font-display">Measurements: {delfine.data.measurements}</p>
			<p class="text-lg font-display">
				Creation Date: {delfine.data.creation_date} <br />
				Culture: {delfine.data.culture[0]}
			</p>
			<p class="text-lg text-primary-content font-display">
				{#if delfine.data.creators != ''}
					Creator: {delfine.data.creators[0].description}
				{:else}
					Creator: Unknown
				{/if}
			</p>
			<p class="text-lg font-display">List of Provenances</p>
			<ol class="list-disc list-outside p-3">
				{#each delfine.data.provenance as provenance}
					<li class="ml-9 text-md font-display">{provenance.description}</li>
				{/each}
			</ol>
			<a href="/dapp" class="btn btn-primary">Back</a>
			{#if isOwner == 1}
				<form on:submit|preventDefault={sendFine} class="m-2 p-2 text-xl">
					<input
						bind:value={receiverAddress}
						type="text"
						class="w-full input input-primary input-bordered"
						placeholder="ETH address of receiver"
					/>
					<button class="mt-1 btn btn-accent w-full justify-center text-xl" type="submit">
						TRANSFER FINE
					</button>
					{#if buyTokensmessage}
						<div
							class="alert alert-error"
							bind:textContent={buyTokensmessage}
							contenteditable="false"
						/>
					{/if}
				</form>
			{/if}
		</div>
	</div>
</div>
