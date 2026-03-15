use std::sync::Arc;

use crate::error::{AppError, IngredientError};
use crate::models::ingredient::{CreateIngredient, Ingredient};
use crate::repositories::ingredient::IngredientRepository;

pub trait IngredientService: Send + Sync {
    async fn list_ingredients(&self) -> Result<Vec<Ingredient>, AppError>;
    async fn create_ingredient(&self, data: CreateIngredient) -> Result<Ingredient, AppError>;
}

pub struct ImplIngredientService<IR: IngredientRepository> {
    ingredient_repo: Arc<IR>,
}

impl<IR: IngredientRepository> ImplIngredientService<IR> {
    pub fn new(ingredient_repo: Arc<IR>) -> Self {
        Self { ingredient_repo }
    }
}

impl<Repo: IngredientRepository> IngredientService for ImplIngredientService<Repo> {
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
