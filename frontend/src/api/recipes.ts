import type { CreateRecipe, Recipe } from "../models/recipe";

export async function getRecipes(): Promise<Recipe[]> {
	const response = await fetch("/api/recipes");

	if (!response.ok) {
		throw new Error(response.statusText);
	}

	const data: Recipe[] = await response.json();

	return data;
}

export async function createRecipe(recipeToCreate: CreateRecipe): Promise<Recipe> {
	const response = await fetch("/api/recipes", {
		method: "POST",
		headers: {
			"Content-Type": "application/json"
		},
		body: JSON.stringify(recipeToCreate)
	});

	if (!response.ok) {
		throw new Error(response.statusText);
	}

	const data: Recipe = await response.json();

	return data;
}
