use crate::error::{AppError, IngredientError};
use crate::models::ingredient::{CreateIngredient, Ingredient, UpdateIngredient};
use crate::repositories::ingredient::IngredientRepository;

pub struct IngredientService {
    repo: IngredientRepository,
}

impl IngredientService {
    pub fn new(repo: IngredientRepository) -> Self {
        Self { repo }
    }

    pub async fn list_ingredients(&self) -> Result<Vec<Ingredient>, AppError> {
        self.repo.find_all().await
    }

    pub async fn create_ingredient(&self, data: CreateIngredient) -> Result<Ingredient, AppError> {
        if data.name.len() == 0 {
            return Err(IngredientError::NameTooShort.into());
        } else if let Some(description) = data.description.clone()
            && description.len() == 0
        {
            return Err(IngredientError::DescriptionTooShort.into());
        }

        self.repo.create(data).await
    }

    pub async fn update_ingredient(&self, data: UpdateIngredient) -> Result<Ingredient, AppError> {
        if let Some(name) = data.name
            && name.len() == 0
        {
            return Err(IngredientError::NameTooShort.into());
        }

        if let Some(description_v) = data.description
            && let Some(description) = description_v
            && description.len() == 0
        {
            return Err(IngredientError::DescriptionTooShort.into());
        }

        Err(AppError::InternalServerError)
    }
}
