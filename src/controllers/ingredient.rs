use std::sync::Arc;

use crate::error::{AppError, IngredientError};
use crate::models::ingredient::{CreateIngredient, Ingredient};
use crate::repositories::ingredient::IngredientRepository;

pub trait IngredientController: Send + Sync {
    async fn list_ingredients(&self) -> Result<Vec<Ingredient>, AppError>;
    async fn create_ingredient(&self, data: CreateIngredient) -> Result<Ingredient, AppError>;
}

#[derive(Debug, Clone)]
pub struct ImplIngredientController<IR: IngredientRepository> {
    ingredient_repo: Arc<IR>,
}

impl<IR: IngredientRepository> ImplIngredientController<IR> {
    pub fn new(ingredient_repo: Arc<IR>) -> Self {
        Self { ingredient_repo }
    }
}

impl<Repo: IngredientRepository> IngredientController for ImplIngredientController<Repo> {
    async fn list_ingredients(&self) -> Result<Vec<Ingredient>, AppError> {
        self.ingredient_repo.find_all().await
    }

    async fn create_ingredient(&self, data: CreateIngredient) -> Result<Ingredient, AppError> {
        if data.name.is_empty() {
            return Err(IngredientError::NameTooShort.into());
        } else if let Some(description) = data.description.clone()
            && description.is_empty()
        {
            return Err(IngredientError::DescriptionTooShort.into());
        }

        self.ingredient_repo.create(data).await
    }
}
