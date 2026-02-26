<script lang="ts">
	import IconPlus from "@lucide/svelte/icons/plus";

	import IngredientsTable from "../components/ingredient/table.svelte";

	import { getIngredients } from "../api/ingredients";

	let data = $state(getIngredients());
</script>

{#await data}
	<p>waiting for the promise to resolve...</p>
{:then ingredients}
	<div id="topbar">
		<a href="/ingredients/new" class="btn"><IconPlus /></a>
	</div>

	<div id="page-container">
		<div id="filters">Filters</div>
	<IngredientsTable {ingredients} />
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
