<script lang="ts">
	import type { Ingredient } from "../../models/ingredient";
	import type { Recipe } from "../../models/recipe";

	interface Props {
		recipe: Recipe;
		ingredients: Ingredient[];
	}

	const { recipe, ingredients }: Props = $props();

	const mapIngredients = new Map<string, Ingredient>(
		ingredients.map((v) => [v.uuid, v]),
	);
</script>

<h1>{recipe.name}</h1>
<ul>
	{#each recipe.ingredients as ingredient}
		<li>
			{ingredient.amount} - {mapIngredients.get(ingredient.uuid)!.name}
		</li>
	{/each}
</ul>
<ol>
	{#each recipe.instructions.toSorted((a, b) => a.step_number - b.step_number) as instruction}
		<li>
			{instruction.step_number} - {instruction.description} - {instruction.uuid}
		</li>
	{/each}
</ol>
