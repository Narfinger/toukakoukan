<script>
	let expenses = [
		{ people: ['christian', 'emilia'], payed: 0, amount: 500 },
		{ people: ['christian', 'emilia'], payed: 1, amount: 1000 },
		{ people: ['christian', 'emilia'], payed: 1, amount: 300 },
		{ people: ['christian', 'emilia'], payed: 0, amount: 200 }
	];
	let group_id = 0;
</script>

<h1>Main Expense Overview</h1>

<h2><a href="/add/{group_id}">ADDD</a></h2>

<table>
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
