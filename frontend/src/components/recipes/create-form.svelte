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
	}
</script>

<form onsubmit={onFormSubmit}>
	<fieldset class="space-y-4">
		<label>
			Name
			<input type="text" bind:value={name} />
		</label>
		<label class="label">
			Description
			<input type="text" bind:value={description} />
		</label>
	</fieldset>
	<fieldset>
		<button type="button"> Submit </button>
	</fieldset>
</form>
