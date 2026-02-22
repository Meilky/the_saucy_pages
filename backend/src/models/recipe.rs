use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Serialize, Deserialize)]
pub struct RecipeIngredient {
    pub uuid: Uuid,
    pub amount: f32,
}

#[derive(sqlx::FromRow, Serialize)]
pub struct Recipe {
    pub uuid: Uuid,
    pub name: String,
    pub description: String,
    #[sqlx(skip)]
    pub ingredients: Vec<RecipeIngredient>,
}

#[derive(Deserialize)]
pub struct CreateRecipe {
    pub name: String,
    pub description: String,
    pub ingredients: Vec<RecipeIngredient>,
}

impl From<CreateRecipe> for Recipe {
    fn from(value: CreateRecipe) -> Self {
        Self {
            uuid: Uuid::now_v7(),
            name: value.name,
            description: value.description,
            ingredients: value.ingredients,
        }
    }
}
