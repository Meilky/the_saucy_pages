<script lang="ts">
	import RecipesTable from "../components/recipe/table.svelte";
	import { getRecipes } from "../api/recipes";

	let data = $state(getRecipes());
</script>

{#await data}
	<p>waiting for the promise to resolve...</p>
{:then recipes}
	<a href="/recipes/new">New</a>

	<RecipesTable {recipes} />
{:catch error}
	<p>Something went wrong: {error.message}</p>
{/await}
