<script lang="ts">
	import Dashboard from "../components/dashboard.svelte";

	import { getRecipes } from "../api/recipes";
	import { getIngredients } from "../api/ingredients";

	async function getData() {
		const [recipes, ingredients] = await Promise.all([
			getRecipes(),
			getIngredients(),
		]);

		return { recipes, ingredients };
	}

	let data = $state(getData());
</script>

{#await data}
	<p>waiting for the promise to resolve...</p>
{:then data}
	<Dashboard {...data} />
{:catch error}
	<p>Something went wrong: {error.message}</p>
{/await}
