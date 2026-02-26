<script lang="ts">
	import type { Ingredient } from "../models/ingredient";
	import type { Recipe } from "../models/recipe";

	import RecipesDetails from "../components/recipe/details.svelte";

	import { getRecipeByUUID } from "../api/recipes";
	import { getIngredientsForRecipe } from "../api/ingredients";

	const pathName = window.location.pathname;

	const match =
		/^\/recipes\/(?<uuid>[0-9a-f]{8}(?:\-[0-9a-f]{4}){3}-[0-9a-f]{12})\/?$/.exec(
			pathName,
		);

	async function getData(): Promise<{
		recipe: Recipe;
		ingredients: Ingredient[];
	}> {
		const [recipe, ingredients] = await Promise.all([
			getRecipeByUUID(match!.groups!.uuid),
			getIngredientsForRecipe(match!.groups!.uuid),
		]);

		return { recipe, ingredients };
	}
</script>

{#await getData()}
	<p>waiting for the promise to resolve...</p>
{:then data}
	<RecipesDetails {...data} />
{:catch error}
	<p>Something went wrong: {error.message}</p>
{/await}
