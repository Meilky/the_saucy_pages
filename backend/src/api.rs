use std::sync::Arc;

use axum::{Router, routing::get};

use crate::controllers::{ingredient, recipe};
use crate::repositories::ingredient::ImplIngredientRepository;
use crate::repositories::recipe::ImplRecipeRepository;
use crate::services::ingredient::ImplIngredientService;
use crate::services::recipe::ImplRecipeService;

pub struct HttpAppState {
    pub recipe_service: ImplRecipeService<ImplRecipeRepository>,
    pub ingredient_service: ImplIngredientService<ImplIngredientRepository>,
}

pub fn get_api(shared_state: Arc<HttpAppState>) -> Router {
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
