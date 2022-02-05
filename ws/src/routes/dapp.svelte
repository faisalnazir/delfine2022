<script>
	import { delfinedata } from '../stores/delfstore';
	import { nftdata } from '../stores/nftstore';
	import DelfineCard from '../components/delfineCard.svelte';
	// console.log($delfinedata);
	// console.log($nftdata);
	import { afterUpdate, beforeUpdate, onMount, tick } from 'svelte';
	//Todo - filter $NFTDATA for DeflinePRive Black
	import { browser } from '$app/env';
	import {
		defaultEvmStores,
		web3,
		selectedAccount,
		connected,
		chainId,
		chainData
	} from 'svelte-web3';
	// defaultEvmStores.setProvider();

	let currentaddress = $selectedAccount;
	tick();
	console.log('The current address is', currentaddress);
	let nftldata;

	const fetchNFTS = async (currentaddress) => {
		console.log('IS the wallet connected?', $connected);
		let nfturl =
			'https://deep-index.moralis.io/api/v2/' + currentaddress + '/nft?chain=mumbai&format=decimal';

		const res = await fetch(nfturl, {
			method: 'GET',
			headers: {
				accept: 'application/json',
				'x-api-key': '685JUEmiN5WrZcHpqxlQ0qEsY99J4Ydqp0RjsL3pWuOU9nzXkuhDg6b9X5OWxX41'
			}
		});
		return await res.json();
	};

	onMount(async () => {
		if (browser) {
			defaultEvmStores.setBrowserProvider();
		}
		// await tick();
		// lnftdata = fetchNFTS().then(function (result) {
		// 	return result;
		// });
		// console.log('onMount the data is ', lnftdata);
	});

	// beforeUpdate(async () => {
	// 	fetchNFTS().then(function (result) {
	// 		lnftdata = result;
	// 		console.log('The result after the update is', result.result);
	// 	});
	// 	console.log('After the update', lnftdata);
	// });
	let nfts = [];
	// let fines = [];
	// $: {
	// 	console.log('About to fetch NFTs');
	// 	fetchNFTS($selectedAccount)
	// 		.then(
	// 			function (result) {
	// 				nfts = result.result;
	// 				// console.log('The result is', result.result);
	// 			},
	// 			function (error) {
	// 				console.log(error);
	// 			}
	// 		)
	// 		.catch();
	// 	console.log('NFT::', nfts);
	// }

	// let fines = nfts.filter(() => {
	// 	token_address == '0x94D9bFE5Fff9EfFe9ff7F80ED2F4f793e419c4c2';
	// });
	// console.log('Fines', fines);
</script>

<svelte:head>
	<title>Delfine</title>
</svelte:head>
<div class="badge badge-md badge-accent flex flex-auto absolute top-5 right-3 font-extrabold">
	{$selectedAccount}
</div>

{#each $nftdata as NFT}
	<!-- {#if NFT.token_id == 10000 && NFT.amount > 0}<div
			class="badge badge-md flex flex-auto absolute top-12 right-3"
		>
			DELFINE PRIVE BLACK
		</div>{/if} -->
{/each}
<h1 class="text-5xl  text-center my-8 font-serif font-display">Delfine Network</h1>

<div class="grid gap-4 md:grid-cols-2 lg:grid-cols-3 sm:grid-cols-1">
	{#each $delfinedata as delfine}
		<DelfineCard {delfine} />
	{/each}
</div>
