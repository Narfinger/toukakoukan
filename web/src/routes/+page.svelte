<script>
	import '../app.css';
	let expenses = [
		{ people: ['christian', 'emilia'], payed: 0, amount: 500 },
		{ people: ['christian', 'emilia'], payed: 1, amount: 1000 },
		{ people: ['christian', 'emilia'], payed: 1, amount: 300 },
		{ people: ['christian', 'emilia'], payed: 0, amount: 200 }
	];
	let group_id = 0;
</script>

<div class="container mx-auto">
	<h1 class="text-3xl font-bold underline">Main Expense Overview</h1>

	<button class="btn"> Button </button>
	<h2><a href="/add/{group_id}">ADDD</a></h2>

	<table class="border-collapse table-auto">
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
</div>
