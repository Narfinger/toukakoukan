<script>
	import '../app.css';
	import { goto } from '$app/navigation';
	let group_id = 0;
	async function getExpenses() {
		let response = await fetch('/expense/' + group_id + '/');
		let expenses = await response.json();
		return expenses;
	}
	const promise = getExpenses();
	function goTo() {
		goto('/add/{group_id}/');
	}
</script>

<div class="container mx-auto px-4">
	<h1 class="underline">Main Expense Overview</h1>

	<button class="btn btn-primary" on:click={goTo}>Button</button>
	<h2><a href="/add/{group_id}">ADDD</a></h2>

	{#await promise}
		<p>Loading expenses</p>
	{:then expenses}
		<table class="table">
			{#each expenses as exp}
				<tr>
					<td
						>{#if exp['payed'] == 0}
							"You are owed"
						{:else}
							"You owe"
						{/if}
					</td>
					<td>{exp['amount'] / exp['people'].length}YEN</td>
				</tr>
			{/each}
		</table>
	{/await}
</div>
