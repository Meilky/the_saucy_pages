<script lang="ts">
	import { getRandomDistinctIntArray } from "../libs/random";
	import type { Ingredient } from "../models/ingredient";
	import type { Recipe } from "../models/recipe";

	interface Props {
		ingredients: Ingredient[];
		recipes: Recipe[];
	}

	const { ingredients, recipes }: Props = $props();

	const randomPickNumber = $state(recipes.length > 5 ? 5 : recipes.length);

	const randomPicks = getRandomDistinctIntArray(
		randomPickNumber,
		0,
		recipes.length,
	);
</script>

<div class="dashboard">
	<h1>Dashboard</h1>

	<div class="stats-grid">
		<div class="stat">
			<h3>Ingredients</h3>
			<p class="count">{ingredients.length}</p>
		</div>

		<div class="stat">
			<h3>Recipes</h3>
			<p class="count">{recipes.length}</p>
		</div>
	</div>

	<div class="recipes">
		<h3>Random Recipes</h3>
		<ul class="recipe-list">
			{#each randomPicks as idx}
				<li class="recipe-item">
					<span class="recipe-name">{recipes[idx].name}</span>
				</li>
			{/each}
		</ul>
	</div>
</div>

<style>
	.dashboard {
		max-width: 1200px;
		margin: 0 auto;
		padding: 20px;
		color: #333;
	}

	h1 {
		text-align: center;
		margin-bottom: 30px;
		color: #2c3e50;
	}

	.stats-grid {
		display: grid;
		grid-template-columns: repeat(auto-fit, minmax(250px, 1fr));
		gap: 20px;
		margin-bottom: 30px;
	}

	.stat {
		background: white;
		padding: 20px;
		border-radius: 8px;
		box-shadow: 0 2px 4px rgba(0, 0, 0, 0.1);
	}

	.stat h3 {
		margin: 0 0 10px 0;
		color: #7f8c8d;
	}

	.count {
		font-size: 2.5em;
		font-weight: bold;
		color: #2c3e50;
	}

	.recipes {
		background: white;
		padding: 20px;
		border-radius: 8px;
		box-shadow: 0 2px 4px rgba(0, 0, 0, 0.1);
	}

	.recipe-list {
		list-style: none;
		padding: 0;
		margin: 0;
	}

	.recipe-item {
		padding: 10px;
		margin-bottom: 10px;
		background: #f8f9fa;
		border-radius: 4px;
		cursor: pointer;
		transition: all 0.2s ease;
	}

	.recipe-item:hover {
		background: #e9ecef;
	}

	.recipe-name {
		display: block;
		margin: 0;
		color: #2c3e50;
	}

	@media (max-width: 768px) {
		.stats-grid {
			grid-template-columns: 1fr;
		}
	}
</style>
