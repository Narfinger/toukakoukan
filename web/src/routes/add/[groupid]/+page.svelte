<script>
	import { superForm, superValidateSync } from 'sveltekit-superforms/client';
	import { redirect } from '@sveltejs/kit';
	import { z } from 'zod';
	const schema = z.object({
		description: z.string().default('Hello world!'),
		total: z.number().positive(),
		select: z.string().default('0')
	});
	const { form, errors, enhance } = superForm(superValidateSync(schema), {
		SPA: true,
		validators: schema,
		onUpdate({ form }) {
			if (form.valid) {
				// TODO: Do something with the validated form.data
				console.log('redirecting');
				throw redirect(303, '/');
			} else {
				console.log('INVALID FORM');
				console.log(form.errors);
			}
		}
	});
	let data = {
		people: ['Christian', 'Emilia']
	};
</script>

<h1>ADDD</h1>
<h2>Adding for {data['people']}</h2>
<form method="POST" action="default" use:enhance>
	<label>
		Expense: <input name="description" autocomplete="off" bind:value={$form.description} />
	</label>
	<label>
		Total: <input name="total" autocomplete="off" type="number" bind:value={$form.total} />
	</label>
	<label>
		Paye:
		<select name="pay_type" id="cars" bind:value={$form.select}>
			<option value="0">{data['people'][0]} is paying 50/50</option>
			<option value="1">{data['people'][1]} is paying 50/50</option>
		</select>
	</label>
	<button type="submit">Submit</button>
</form>
