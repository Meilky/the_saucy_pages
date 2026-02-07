<script lang="ts">
	import type { Recipe } from "../../models/recipe";

	interface Props {
		recipes: Recipe[];
	}

	const props: Props = $props();
</script>

<ul>
	{#each props.recipes as recipe}
		<li>{recipe.name}</li>
	{/each}
</ul>
