use std::sync::Arc;

use axum::{Router, routing::get};

use crate::controllers::{ingredient, recipe};
use crate::services::ingredient::IngredientService;
use crate::services::recipe::RecipeService;

pub struct AppState<RS>
where
    RS: RecipeService,
{
    pub recipe_service: RS,
    pub ingredient_service: IngredientService,
}

pub fn get_api<RS: RecipeService + Sync + Send>(shared_state: Arc<AppState<RS>>) -> Router {
    Router::new()
        .route(
            "/api/recipes",
            get(recipe::get_recipes).post(recipe::post_recipe),
        )
        .route(
            "/api/ingredients",
            get(ingredient::get_ingredients).post(ingredient::post_ingredient),
        )
        .with_state(shared_state)
}
