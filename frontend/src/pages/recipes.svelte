<script lang="ts">
	import IconPlus from "@lucide/svelte/icons/plus";

	import RecipesTable from "../components/recipe/table.svelte";
	import { getRecipes } from "../api/recipes";

	let data = $state(getRecipes());
</script>

{#await data}
	<p>waiting for the promise to resolve...</p>
{:then recipes}
	<div id="topbar">
		<a href="/recipes/new" class="btn"><IconPlus /></a>
	</div>

	<div id="page-container">
		<div id="filters">Filters</div>
		<RecipesTable {recipes} />
	</div>
{:catch error}
	<p>Something went wrong: {error.message}</p>
{/await}

<style>
	.btn {
		display: flex;
		border: 1px darkgray solid;
		align-items: center;
		border-radius: 2px;
		width: max-content;
		padding: 2px;
		color: black;
		background-color: lightgray;
		text-decoration: none;
	}

	#topbar {
		padding-left: 1ch;
		padding-right: 1ch;

		padding-top: 0.5em;
		padding-bottom: 0.5em;

		border-bottom: 1px darkgray solid;
	}

	#page-container {
		display: grid;
		grid-template-columns: 15vw auto;
	}
</style>
