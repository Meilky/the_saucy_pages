export type RecipeIngredient = {
	uuid: string;
	amount: number;
};

export type Recipe = {
	uuid: string;
	name: string;
	description: string;
	ingredients: RecipeIngredient[];
};

export type CreateRecipe = {
	name: string;
	description: string;
	ingredients: RecipeIngredient[];
};
