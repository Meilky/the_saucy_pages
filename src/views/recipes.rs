use iced::{
    Element, Task,
    widget::{column, text},
};

use crate::{
    controllers::recipe::RecipeController as _, init::RecipeController, models::recipe::Recipe,
};

pub struct Recipes {
    recipes: Option<Vec<Recipe>>,
}

#[derive(Debug, Clone)]
pub enum Message {
    Update(Vec<Recipe>),
}

impl Recipes {
    pub fn boot(recipe_controller: RecipeController) -> (Self, Task<Message>) {
        (
            Self { recipes: None },
            Task::perform(Recipes::load(recipe_controller), Message::Update),
        )
    }

    pub async fn load(recipe_controller: RecipeController) -> Vec<Recipe> {
        recipe_controller.list_recipes().await.unwrap()
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
