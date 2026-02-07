<script lang="ts">
	import IngredientsTable from "../components/ingredient/table.svelte";
	import { getIngredients } from "../api/ingredients";

	let data = $state(getIngredients());
</script>

{#await data}
	<p>waiting for the promise to resolve...</p>
{:then ingredients}
	<IngredientsTable {ingredients} />
{:catch error}
	<p>Something went wrong: {error.message}</p>
{/await}
