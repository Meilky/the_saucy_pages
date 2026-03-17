use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct RecipeIngredient {
    pub uuid: Uuid,
    pub amount: f32,
}

#[derive(Serialize, Clone, Debug)]
pub struct RecipeInstruction {
    pub uuid: Uuid,
    pub step_number: u16,
    pub description: String,
}

#[derive(Deserialize, Clone, Debug)]
pub struct CreateRecipeInstruction {
    pub step_number: u16,
    pub description: String,
}

impl From<CreateRecipeInstruction> for RecipeInstruction {
    fn from(value: CreateRecipeInstruction) -> Self {
        Self {
            uuid: Uuid::now_v7(),
            step_number: value.step_number,
            description: value.description,
        }
    }
}

#[derive(sqlx::FromRow, Serialize, Clone, Debug)]
pub struct Recipe {
    pub uuid: Uuid,
    pub name: String,
    pub description: String,
    #[sqlx(skip)]
    pub ingredients: Vec<RecipeIngredient>,
    #[sqlx(skip)]
    pub instructions: Vec<RecipeInstruction>,
}

#[derive(Deserialize, Clone, Debug)]
pub struct CreateRecipe {
    pub name: String,
    pub description: String,
    pub ingredients: Vec<RecipeIngredient>,
    pub instructions: Vec<CreateRecipeInstruction>,
}

impl From<CreateRecipe> for Recipe {
    fn from(value: CreateRecipe) -> Self {
        Self {
            uuid: Uuid::now_v7(),
            name: value.name,
            description: value.description,
            ingredients: value.ingredients,
            instructions: value
                .instructions
                .iter()
                .map(|v| v.clone().into())
                .collect(),
        }
    }
}
