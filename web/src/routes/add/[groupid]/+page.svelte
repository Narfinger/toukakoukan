<script>
	import { enhance } from '$app/forms';
	let data = {
		people: ['Christian', 'Emilia']
	};

	export const actions = {
		default: async ({ cookies, request }) => {
			const data = await request.formData();
		}
	};
</script>

<h1>ADDD</h1>
<h2>Adding for {data['people']}</h2>
<form method="POST">
	<label>
		Expense: <input name="description" autocomplete="off" />
	</label>
	<label>
		Total: <input name="total" autocomplete="off" />
	</label>
	<label>
		Paye:
		<select name="pay_type" id="cars">
			<option value="volvo">{data['people'][0]} is paying 50/50</option>
			<option value="saab">{data['people'][1]} is paying 50/50</option>
		</select>
	</label>
</form>
