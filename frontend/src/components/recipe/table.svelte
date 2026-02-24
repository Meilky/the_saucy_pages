<script lang="ts">
	import type { Recipe } from "../../models/recipe";

	interface Props {
		recipes: Recipe[];
	}

	const props: Props = $props();
</script>

<table>
	<thead>
		<tr>
			<th>Name</th>
			<th>Actions</th>
		</tr>
	</thead>
	<tbody>
		{#each props.recipes as recipe}
			<tr>
				<td>{recipe.name}</td>
				<td><a href={`/recipes/${recipe.uuid}`}>View</a></td>
			</tr>
		{/each}
	</tbody>
</table>

<style>
	table {
		border-collapse: collapse;
		width: 100%;
	}

	td,
	th {
		border: 1px solid #dddddd;
		text-align: left;
		padding: 8px;
	}

	tr:nth-child(even) {
		background-color: #dddddd;
	}
</style>
