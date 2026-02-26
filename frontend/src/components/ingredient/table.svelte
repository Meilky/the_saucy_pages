<script lang="ts">
	import type { Ingredient } from "../../models/ingredient";

	interface Props {
		ingredients: Ingredient[];
	}

	const props: Props = $props();
</script>

<table>
	<thead>
		<tr>
			<th>
				<input type="checkbox" />
			</th>
			<th>Name</th>
			<th>Actions</th>
		</tr>
	</thead>
	<tbody>
		{#each props.ingredients as ingredient}
			<tr>
				<td><input type="checkbox" /></td>
				<td>{ingredient.name}</td>
				<td><a href={`/ingredients/${ingredient.uuid}`}>View</a></td>
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
		border: 1px solid darkgray;
		text-align: left;
		padding: 8px;
	}

	/* Remove TOP outer border (first row's top) */
	tr:first-child th,
	tr:first-child td {
		border-top: none;
	}

	/* Remove BOTTOM outer border (last row's bottom) */
	tr:last-child td {
		border-bottom: none;
	}

	/* Remove RIGHT outer border (last column's right) */
	td:last-child,
	th:last-child {
		border-right: none;
	}

	tr:nth-child(even) {
		background-color: lightgray;
	}
</style>
