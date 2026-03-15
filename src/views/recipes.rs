use iced::{
    Element, Task,
    widget::{column, text},
};

use crate::{init::RecipeService, models::recipe::Recipe, services::recipe::RecipeService as _};

pub struct Recipes {
    recipe_service: RecipeService,
    recipes: Option<Vec<Recipe>>,
}

#[derive(Debug, Clone)]
pub enum Message {
    Update(Vec<Recipe>),
}

impl Recipes {
    pub fn boot(recipe_service: RecipeService) -> (Self, Task<Message>) {
        (
            Self {
                recipe_service: recipe_service.clone(),
                recipes: None,
            },
            Task::perform(Recipes::load(recipe_service), Message::Update),
        )
    }

    pub async fn load(recipe_service: RecipeService) -> Vec<Recipe> {
        recipe_service.list_recipes().await.unwrap()
    }

    pub fn view(&self) -> Element<'_, Message> {
        if let Some(recipes) = &self.recipes {
            let col = column(recipes.iter().map(|r| text(r.name.clone()).into()));

            return col.into();
        }

        text("Loading...").into()
    }

    pub fn update(&mut self, message: Message) {
        match message {
            Message::Update(recipes) => {
                self.recipes = Some(recipes);
            }
        }
    }
}
