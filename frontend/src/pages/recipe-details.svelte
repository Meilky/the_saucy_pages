<script lang="ts">
	import RecipesDetails from "../components/recipe/details.svelte";
	import { getRecipeByUUID } from "../api/recipes";

	const pathName = window.location.pathname;

	const match = /^\/recipes\/(?<uuid>[0-9a-f]{8}(?:\-[0-9a-f]{4}){3}-[0-9a-f]{12})\/?$/.exec(pathName);

	let data = $state(getRecipeByUUID(match!.groups!.uuid));
</script>

{#await data}
	<p>waiting for the promise to resolve...</p>
{:then recipe}
	<RecipesDetails {recipe} />
{:catch error}
	<p>Something went wrong: {error.message}</p>
{/await}
