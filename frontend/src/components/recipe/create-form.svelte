<script lang="ts">
	import { createRecipe } from "$api/recipes";
	import type { RecipeIngredient } from "../../models/recipe";

	interface Props {
		onSubmited?: () => void;
	}

	const props: Props = $props();

	let name = $state("");
	let description = $state("");
	let ingredients: RecipeIngredient[] = $state([]);

	function reset() {
		name = "";
		description = "";
		ingredients = [];
	}

	async function submit() {
		await createRecipe({
			name,
			description,
			ingredients,
		});

		reset();

		props.onSubmited?.();
	}

	async function onFormSubmit(e: SubmitEvent) {
		e.preventDefault();

		await submit();
	}

	function onFormReset(e: any) {
		e.preventDefault();

		reset();
	}

	function addIngredient() {
		ingredients.push({
			uuid: "123",
			amount: 14,
		});
	}

	function removeIngredient(uuid: string) {
		const idx = ingredients.findIndex((v) => v.uuid === uuid);

		if (idx === -1) return;

		ingredients.splice(idx, 1);
	}

	function createRemoveRecipeCallback(uuid: string) {
		return () => {
			return removeIngredient(uuid);
		};
	}

	function createEditRecipeCallback(uuid: string) {
		return () => {
			console.log(`Edit: ${uuid}`);
		};
	}
</script>

<form onsubmit={onFormSubmit} onreset={onFormReset}>
	<fieldset>
		<legend>New recipe</legend>
		<label for="name">Name</label>
		<input type="text" name="name" bind:value={name} />
		<label for="description">Description</label>
		<input type="text" name="description" bind:value={description} />
		<table>
			<thead>
				<tr>
					<th>Ingredient</th>
					<th>Amount</th>
					<th>
						<button type="button" onclick={addIngredient}>
							Add
						</button>
					</th>
				</tr>
			</thead>
			<tbody>
				{#each ingredients as ingredient}
					<tr>
						<td>{ingredient.uuid}</td>
						<td>{ingredient.amount}</td>
						<td>
							<button
								type="button"
								onclick={createRemoveRecipeCallback(
									ingredient.uuid,
								)}
							>
								Remove
							</button>
							<button
								type="button"
								onclick={createEditRecipeCallback(
									ingredient.uuid,
								)}
							>
								Edit
							</button>
						</td>
					</tr>
				{/each}
			</tbody>
		</table>
	</fieldset>
	<input type="submit" value="Submit" />
	<input type="reset" value="Reset" />
</form>

<style>
	fieldset {
		display: grid;
		grid-template-columns: auto auto;
	}

	table {
		grid-column-start: 1;
		grid-column-end: 3;
	}
</style>
