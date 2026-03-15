use std::sync::Arc;

use sqlx::sqlite::SqlitePoolOptions;

use crate::{
    repositories::{ingredient::SQLiteIngredientRepository, recipe::SQLiteRecipeRepository},
    services::{ingredient::ImplIngredientService, recipe::ImplRecipeService},
};

pub type RecipeService = ImplRecipeService<SQLiteRecipeRepository, SQLiteIngredientRepository>;
pub type IngredientService = ImplIngredientService<SQLiteIngredientRepository>;

pub async fn init() -> (RecipeService, IngredientService) {
    let database_url =
        std::env::var("DATABASE_URL").expect("DATABASE_URL must be set in .env or shell");

    let sqlite_pool = SqlitePoolOptions::new()
        .max_connections(5)
        .connect(&database_url)
        .await
        .unwrap();

    sqlx::migrate!().run(&sqlite_pool).await.unwrap();

    let arc_pool = Arc::new(sqlite_pool);

    let ingredient_repository = Arc::new(SQLiteIngredientRepository::new(Arc::clone(&arc_pool)));
    let recipe_repository = Arc::new(SQLiteRecipeRepository::new(Arc::clone(&arc_pool)));

    let ingredient_service = ImplIngredientService::new(ingredient_repository.clone());
    let recipe_service = ImplRecipeService::new(recipe_repository, ingredient_repository);

    (recipe_service, ingredient_service)
}
