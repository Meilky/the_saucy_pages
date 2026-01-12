<script lang="ts">
	import { getRecipes } from "$api/recipes";
	import { AppBar } from "@skeletonlabs/skeleton-svelte";
    import CreateForm from "./recipes/create-form.svelte";

	let data = $state(getRecipes());
</script>

<AppBar>
	<AppBar.Toolbar class="grid-cols-[auto_1fr_auto]">
		<AppBar.Lead>
			<h1 class="h1">The Saucy Pages</h1>
		</AppBar.Lead>
	</AppBar.Toolbar>
</AppBar>

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
