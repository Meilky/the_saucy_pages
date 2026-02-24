<script lang="ts">
    import type { Ingredient } from "../models/ingredient";

    import { getIngredients } from "$api/ingredients";
	import CreateRecipeForm from "../components/recipe/create-form.svelte";

	function onSubmited() {
		navigation.navigate("/recipes");
	}

	async function getData(): Promise<Ingredient[]> {
		return getIngredients();
	}

	let data = $state(getData());
</script>

{#await data}
	<p>waiting for the promise to resolve...</p>
{:then ingredients}
	<CreateRecipeForm {onSubmited} {ingredients} />
{:catch error}
	<p>Something went wrong: {error.message}</p>
{/await}
