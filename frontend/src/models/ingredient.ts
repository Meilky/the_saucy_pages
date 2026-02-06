export type Ingredient = {
	uuid: string;
	name: string;
	description: string | undefined;
};

export type CreateIngredient = {
	name: string;
	description: string | undefined;
};
