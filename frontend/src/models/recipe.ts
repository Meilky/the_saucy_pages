export type RecipeInstruction = {
	uuid: string;
	step_number: number;
	description: string;
};

export type RecipeIngredient = {
	uuid: string;
	amount: number;
};

export type Recipe = {
	uuid: string;
	name: string;
	description: string;
	ingredients: RecipeIngredient[];
	instructions: RecipeInstruction[];
};

export type CreateRecipeInstruction = {
	step_number: number;
	description: string;
};

export type CreateRecipe = {
	name: string;
	description: string;
	ingredients: RecipeIngredient[];
	instructions: CreateRecipeInstruction[];
};
