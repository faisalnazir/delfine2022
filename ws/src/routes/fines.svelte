<script>
	import { delfinedata } from '../stores/delfstore';
	import { afterUpdate, beforeUpdate, onMount, tick, onDestroy } from 'svelte';
	import { browser } from '$app/env';
	import {
		defaultEvmStores,
		web3,
		selectedAccount,
		connected,
		chainId,
		chainData
	} from 'svelte-web3';
	let nfts = [];
	let currentaddress = $selectedAccount;
	console.log('The current address is', currentaddress);

	const fetchNFTS = async (currentaddress) => {
		console.log('Is the wallet connected?', $connected);

		let nfturl =
			'https://deep-index.moralis.io/api/v2/' + currentaddress + '/nft?chain=mumbai&format=decimal';
		console.log('The URL for the NFT', nfturl);

		let res = await fetch(nfturl, {
			headers: {
				accept: 'application/json',
				'x-api-key': '685JUEmiN5WrZcHpqxlQ0qEsY99J4Ydqp0RjsL3pWuOU9nzXkuhDg6b9X5OWxX41',
				mode: 'no-cors'
			}
		});
		console.log(res);
		return await res.json();
	};

	onMount(async () => {
		if (browser) {
			defaultEvmStores.setBrowserProvider();
		}
	});
	$: {
		console.log('About to fetch NFTs');
		fetchNFTS($selectedAccount)
			.then(
				function (result) {
					nfts = result.result;
					console.log('The result is', result.result);
				},
				function (error) {
					console.log(error);
				}
			)
			.catch();
	}
	console.log($delfinedata);
</script>

<svelte:head>
	<title>Delfine</title>
</svelte:head>
<div class="badge badge-md badge-accent flex flex-auto absolute top-5 right-3 font-extrabold">
	{$selectedAccount}
</div>

{#each nfts as nft}
	{#if nft.token_id == 10000 && nft.amount > 0}<div
			class="badge badge-md flex flex-auto absolute top-12 right-3"
		>
			DELFINE PRIVE BLACK
		</div>{/if}
{/each}
<h1 class="text-5xl  text-center my-8 font-serif font-display">My Fine Collection</h1>
{#if !nfts}
	<p>Awaiting NFTs from the Blockchain</p>
{/if}
{#each nfts as nft}
	{#if nft.token_address == '0x94d9bfe5fff9effe9ff7f80ed2f4f793e419c4c2'}
		{#if nft.token_id != 10000}
			<a class="mt-16 m-8" href={`/delfine/${nft.token_id}`}>
				<div class="m-6 indicator">
					<div class="indicator-item badge badge-primary text-white">{nft.contract_type}</div>
					<div class="avatar">
						<div class="mb-8 rounded-box w-64 h-64 ">
							<img
								class="saturate-100 blur-0 brightness-100 mask mask-squircle"
								src={$delfinedata.find(function (element) {
									return element.id == nft.token_id;
								}).images.web.url}
							/>
						</div>
					</div>
				</div>
			</a>
		{/if}
	{/if}
{/each}
