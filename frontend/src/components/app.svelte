<script lang="ts">
	import { getRecipes } from "$api/recipes";
    import CreateForm from "./recipes/create-form.svelte";

	let data = $state(getRecipes());
</script>

{#await data}
	<p>waiting for the promise to resolve...</p>
{:then recipes}
	<ul>
		{#each recipes as recipe}
			<li>{recipe.name}</li>
		{/each}
	</ul>
{:catch error}
	<p>Something went wrong: {error.message}</p>
{/await}

<CreateForm />
