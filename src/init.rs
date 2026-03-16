use std::sync::Arc;

use sqlx::sqlite::SqlitePoolOptions;

use crate::{
    controllers::{ingredient::ImplIngredientController, recipe::ImplRecipeController},
    repositories::{ingredient::SQLiteIngredientRepository, recipe::SQLiteRecipeRepository},
};

pub type RecipeController = ImplRecipeController<SQLiteRecipeRepository, SQLiteIngredientRepository>;
pub type IngredientController = ImplIngredientController<SQLiteIngredientRepository>;

pub async fn init() -> (RecipeController, IngredientController) {
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

    let ingredient_service = ImplIngredientController::new(ingredient_repository.clone());
    let recipe_service = ImplRecipeController::new(recipe_repository, ingredient_repository);

    (recipe_service, ingredient_service)
}
