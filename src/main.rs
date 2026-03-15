use std::sync::Arc;

use dotenv::dotenv;
use sqlx::sqlite::SqlitePoolOptions;

use iced::widget::{Column, button, column, text};

use crate::{
    repositories::{ingredient::ImplIngredientRepository, recipe::ImplRecipeRepository},
    services::{
        ingredient::{ImplIngredientService, IngredientService},
        recipe::{ImplRecipeService, RecipeService},
    },
    views::app::App,
};

mod error;
mod models;
mod repositories;
mod services;
mod views;

async fn init() -> (impl RecipeService, impl IngredientService) {
    let database_url =
        std::env::var("DATABASE_URL").expect("DATABASE_URL must be set in .env or shell");

    let sqlite_pool = SqlitePoolOptions::new()
        .max_connections(5)
        .connect(&database_url)
        .await
        .unwrap();

    sqlx::migrate!().run(&sqlite_pool).await.unwrap();

    let arc_pool = Arc::new(sqlite_pool);

    let ingredient_repository = Arc::new(ImplIngredientRepository::new(Arc::clone(&arc_pool)));
    let recipe_repository = Arc::new(ImplRecipeRepository::new(Arc::clone(&arc_pool)));

    let ingredient_service = ImplIngredientService::new(ingredient_repository.clone());
    let recipe_service = ImplRecipeService::new(recipe_repository, ingredient_repository);

    (recipe_service, ingredient_service)
}

fn main() -> iced::Result {
    dotenv().ok();

    tracing_subscriber::fmt::init();

    iced::application(App::new, App::update, App::view)
        .title("The Saucy Pages")
        .run()
}
