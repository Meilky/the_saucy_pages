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
};

mod error;
mod models;
mod repositories;
mod services;

#[derive(Default)]
struct Counter {
    value: i32,
}

#[derive(Debug, Clone, Copy)]
pub enum Message {
    Increment,
    Decrement,
}

impl Counter {
    pub fn view(&self) -> Column<'_, Message> {
        // We use a column: a simple vertical layout
        column![
            // The increment button. We tell it to produce an
            // `Increment` message when pressed
            button("+").on_press(Message::Increment),
            // We show the value of the counter here
            text(self.value).size(50),
            // The decrement button. We tell it to produce a
            // `Decrement` message when pressed
            button("-").on_press(Message::Decrement),
        ]
    }

    pub fn update(&mut self, message: Message) {
        match message {
            Message::Increment => {
                self.value += 1;
            }
            Message::Decrement => {
                self.value -= 1;
            }
        }
    }
}

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

    iced::run(Counter::update, Counter::view)
}
