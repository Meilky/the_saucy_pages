use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(sqlx::FromRow, Serialize)]
pub struct Recipe {
    pub uuid: Uuid,
    pub name: String,
    pub description: String,
}

#[derive(Deserialize)]
pub struct CreateRecipe {
    pub name: String,
    pub description: String,
}

impl From<CreateRecipe> for Recipe {
    fn from(value: CreateRecipe) -> Self {
        Self {
            uuid: Uuid::now_v7(),
            name: value.name,
            description: value.description,
        }
    }
}

#[derive(Deserialize)]
pub struct UpdateRecipe {
    pub name: Option<String>,
    pub description: Option<String>,
}
