<script>
	export let tokenFloat;
	export let account_balance;
	export let myDelfs;
</script>

<div class="flex flex-col w-200 shadow stats">
	<div class="stat">
		<div class="stat-figure text-secondary" />
		<div class="stat-title">Tokens Remaining</div>
		<div class="stat-value">{tokenFloat}</div>
		<div class="stat-desc">DELFS</div>
	</div>
	<div class="stat">
		<div class="stat-figure text-secondary" />
		<div class="stat-title">Your Balance</div>
		<div class="stat-value">{account_balance}</div>
		<div class="stat-desc ">Tokens to purchase DELF</div>
	</div>
	<div class="stat">
		<div class="stat-figure text-secondary" />
		<div class="stat-title">Your Delfs</div>
		<div class="stat-value">{myDelfs} DELFs</div>
		<div class="stat-desc">Delfine Utlity Token</div>
	</div>
</div>
