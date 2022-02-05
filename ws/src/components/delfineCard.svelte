<script>
	export let delfine;
</script>

<a href={`/delfine/${delfine.id}`}>
	<div class="card card-bordered card-compact shadow-md">
		<figure>
			<img class="scale-100 rounded-sm" src={delfine.images.web.url} alt={delfine.title} />
		</figure>
		<div class="card-body">
			<h2 class="card-title text-bold font-display text-md trucate ...">
				{delfine.title}
				{#if delfine.creators != ''}
					<p class="text-lg text-primary-content font-display font-extralight">
						{delfine.creators[0].description}
					</p>
				{:else}
					<p class="text-sm text-primary-content">Creator unknown</p>
				{/if}
				<div class="badge mx-2 badge-secondary">ERC1511</div>
			</h2>
			<p>
				<!-- {delfine.tombstone} -->
			</p>
			<!-- <div class="justify-end card-actions">
			<a href={`/delfine/${delfine.id}`} class="btn btn-accent">More info</a>
		</div> -->
		</div>
	</div>
</a>
