<script lang="ts">
	import { createRecipe, getRecipes } from "./api/recipes";

	let name = $state("");
	let description = $state("");

	let data = $state(getRecipes());

	async function onFormSubmit(e: SubmitEvent) {
		e.preventDefault();

		await createRecipe({
			name,
			description,
		});

		name = "";
		description = "";

		data = getRecipes();
	}
</script>

<main class="container">
	<h1>The Saucy Pages</h1>
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
	<form onsubmit={onFormSubmit}>
		<fieldset>
			<label>
				Name
				<input type="text" bind:value={name} />
			</label>
			<label>
				Description
				<input type="text" bind:value={description} />
			</label>
		</fieldset>
		<input type="submit" value="Submit" />
	</form>
</main>
