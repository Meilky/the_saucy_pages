<script lang="ts">
	import { createRecipe } from "$api/recipes";
	import type { Ingredient } from "../../models/ingredient";
	import type {
		CreateRecipeInstruction,
		RecipeIngredient,
	} from "../../models/recipe";

	interface Props {
		onSubmited?: () => void;
		ingredients: Ingredient[];
	}

	const props: Props = $props();

	let name = $state("");
	let description = $state("");
	let ingredients: RecipeIngredient[] = $state([]);
	let instructions: CreateRecipeInstruction[] = $state([]);

	function reset() {
		name = "";
		description = "";
		ingredients = [];
		instructions = [];
	}

	async function submit() {
		await createRecipe({
			name,
			description,
			ingredients,
			instructions,
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
			uuid: "",
			amount: 0,
		});
	}

	function addInstruction() {
		instructions.push({
			step_number: instructions.length,
			description: "",
		});
	}

	function removeIngredient(uuid: string) {
		const idx = ingredients.findIndex((v) => v.uuid === uuid);

		if (idx === -1) return;

		ingredients.splice(idx, 1);
	}

	function removeInstruction(step_number: number) {
		const idx = instructions.findIndex(
			(v) => v.step_number === step_number,
		);

		if (idx === -1) return;

		ingredients.splice(idx, 1);
	}

	function createRemoveIngredientCallback(uuid: string) {
		return () => {
			return removeIngredient(uuid);
		};
	}

	function createRemoveInstructionCallback(step_number: number) {
		return () => {
			return removeInstruction(step_number);
		};
	}

	function onStepChanged(
		instruction: CreateRecipeInstruction,
		step_number: number,
	) {
		// decrement
		if (instruction.step_number > step_number) {
			for (let i = step_number; i < instruction.step_number; i++) {
				let ins = instructions.find((v) => v.step_number === i);
				ins!.step_number++;
			}
		}

		// increment
		if (instruction.step_number < step_number) {
			for (let i = step_number; i > instruction.step_number; i--) {
				let ins = instructions.find((v) => v.step_number === i);
				ins!.step_number++;
			}
		}

		instruction.step_number = step_number;
	}

	const datalistId = $props.id();
</script>

<form onsubmit={onFormSubmit} onreset={onFormReset}>
	<datalist id={datalistId}>
		{#each props.ingredients as ingredient}
			<option value={ingredient.uuid} label={ingredient.name}></option>
		{/each}
	</datalist>
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
						<td
							><input
								list={datalistId}
								bind:value={ingredient.uuid}
							/></td
						>
						<td
							><input
								type="number"
								min="0"
								step="0.1"
								bind:value={ingredient.amount}
							/></td
						>
						<td>
							<button
								type="button"
								onclick={createRemoveIngredientCallback(
									ingredient.uuid,
								)}
							>
								Remove
							</button>
						</td>
					</tr>
				{/each}
			</tbody>
		</table>
		<table>
			<thead>
				<tr>
					<th>Instructions (Step number)</th>
					<th>Description</th>
					<th>
						<button type="button" onclick={addInstruction}>
							Add
						</button>
					</th>
				</tr>
			</thead>
			<tbody>
				{#each instructions.toSorted((a, b) => a.step_number - b.step_number) as instruction}
					<tr>
						<td
							><input
								type="number"
								min="0"
								max={instructions.length - 1}
								step="1"
								bind:value={
									() => instruction.step_number,
									(step_number) =>
										onStepChanged(instruction, step_number)
								}
							/></td
						>
						<td><input bind:value={instruction.description} /></td>
						<td>
							<button
								type="button"
								onclick={createRemoveInstructionCallback(
									instruction.step_number,
								)}
							>
								Remove
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

	td > input {
		width: 100%;
	}
</style>
