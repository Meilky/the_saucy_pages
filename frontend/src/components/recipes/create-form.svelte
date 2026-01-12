<script lang="ts">
	import { createRecipe } from "$api/recipes";

	let name = $state("");
	let description = $state("");

	async function onFormSubmit(e: SubmitEvent) {
		e.preventDefault();

		await createRecipe({
			name,
			description,
		});

		name = "";
		description = "";

		data = getRecipes();
	}
</script>

<form onsubmit={onFormSubmit}>
	<fieldset class="space-y-4">
		<label class="label">
			<label class="label-text">Name</label>
			<input class="input" type="text" bind:value={name} />
		</label>
		<label class="label">
			<label class="label-text">Description</label>
			<input class="input" type="text" bind:value={description} />
		</label>
	</fieldset>
	<fieldset class="flex justify-end">
		<button type="button" class="btn preset-outlined-surface-300-700">Submit</button>
	</fieldset>
</form>
