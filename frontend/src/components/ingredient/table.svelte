<script lang="ts">
	import type { Ingredient } from "../../models/ingredient";

	interface Props {
		ingredients: Ingredient[];
	}

	const props: Props = $props();
</script>

<ul>
	{#each props.ingredients as ingredient}
		<li>{ingredient.name}</li>
	{/each}
</ul>
