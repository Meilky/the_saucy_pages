use std::sync::Arc;

use axum::{
    Router,
    routing::get,
};

use crate::controllers::recipe;
use crate::services::recipe::RecipeService;

pub struct AppState {
    pub recipe_service: RecipeService,
}

pub fn get_api(shared_state: Arc<AppState>) -> Router {
    Router::new()
        .route(
            "/api/recipes",
            get(recipe::get_recipes).post(recipe::post_recipe),
        )
        .with_state(shared_state)
}
