use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(sqlx::FromRow, Serialize, Debug, Clone)]
pub struct Ingredient {
    pub uuid: Uuid,
    pub name: String,
    pub description: Option<String>,
}

#[derive(Deserialize, Debug, Clone)]
pub struct CreateIngredient {
    pub name: String,
    pub description: Option<String>,
}

impl From<CreateIngredient> for Ingredient {
    fn from(value: CreateIngredient) -> Self {
        Self {
            uuid: Uuid::now_v7(),
            name: value.name,
            description: value.description,
        }
    }
}

#[derive(Deserialize)]
pub struct UpdateIngredient {
    pub name: Option<String>,
    pub description: Option<Option<String>>,
}
