<script lang="ts">
	import type { Recipe } from "../../models/recipe";

	interface Props {
		recipe: Recipe;
	}

	const { recipe }: Props = $props();
</script>

<h1>{recipe.name}</h1>
<ul>
	{#each recipe.ingredients as ingredient}
		<li>{ingredient.amount} - {ingredient.uuid}</li>
	{/each}
</ul>
<ol>
	{#each recipe.instructions.toSorted((a, b) => a.step_number - b.step_number) as instruction}
		<li>
			{instruction.step_number} - {instruction.description} - {instruction.uuid}
		</li>
	{/each}
</ol>
