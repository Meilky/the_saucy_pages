use crate::models::recipe::{CreateRecipe, Recipe};
use crate::repositories::recipe::RecipeRepository;

pub struct RecipeService {
    repo: RecipeRepository,
}

impl RecipeService {
    pub fn new(repo: RecipeRepository) -> Self {
        Self { repo }
    }

    pub async fn create_recipe(&self, data: CreateRecipe) -> Result<Recipe, sqlx::Error> {
        if data.name.len() == 0 {
            return Err(sqlx::Error::Protocol("Invalid name".into()));
        } else if data.description.len() == 0 {
            return Err(sqlx::Error::Protocol("Invalid description".into()));
        }

        self.repo.create(data).await
    }

    pub async fn list_recipes(&self) -> Result<Vec<Recipe>, sqlx::Error> {
        self.repo.find_all().await
    }
}
