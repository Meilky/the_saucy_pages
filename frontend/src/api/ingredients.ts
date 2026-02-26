import type { CreateIngredient, Ingredient } from "../models/ingredient";

export async function getIngredients(): Promise<Ingredient[]> {
	const response = await fetch("/api/ingredients");

	if (!response.ok) {
		throw new Error(response.statusText);
	}

	const data: Ingredient[] = await response.json();

	return data;
}

export async function getIngredientsForRecipe(uuid: string): Promise<Ingredient[]> {
	const response = await fetch(`/api/recipes/${uuid}/ingredients`);

	if (!response.ok) {
		throw new Error(response.statusText);
	}

	const data: Ingredient[] = await response.json();

	return data;
}

export async function createIngredient(ingredientToCreate: CreateIngredient): Promise<Ingredient> {
	const response = await fetch("/api/ingredients", {
		method: "POST",
		headers: {
			"Content-Type": "application/json",
		},
		body: JSON.stringify(ingredientToCreate),
	});

	if (!response.ok) {
		throw new Error(response.statusText);
	}

	const data: Ingredient = await response.json();

	return data;
}
