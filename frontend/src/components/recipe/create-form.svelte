<script lang="ts">
	import { createRecipe } from "$api/recipes";

	interface Props {
		onSubmited?: () => void;
	}

	const props: Props = $props();

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

		props.onSubmited?.();
	}
</script>

<form onsubmit={onFormSubmit}>
	<fieldset>
		<legend>New recipe</legend>
		<label for="name">Name</label>
		<input type="text" name="name" bind:value={name} />
		<label for="description">Description</label>
		<input type="text" name="description" bind:value={description} />
	</fieldset>
	<input type="submit" value="Submit" />
	<input type="reset" value="Reset" />
</form>

<style>
	fieldset {
		display: grid;
		grid-template-columns: auto auto;
	}
</style>
