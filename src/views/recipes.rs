use iced::{
    Element, Task,
    widget::{column, text},
};

use crate::{
    controllers::recipe::RecipeController as _,
    init::RecipeController,
    models::recipe::{CreateRecipe, Recipe},
};

enum State {
    Loading,
    Loaded,
}

pub struct Recipes {
    current_state: State,
    recipe_controller: RecipeController,
    recipes: Vec<Recipe>,
}

#[derive(Debug, Clone)]
pub enum Message {
    Load,
    Loaded(Vec<Recipe>),
    Create(CreateRecipe),
}

async fn load_recipes(controller: RecipeController) -> Vec<Recipe> {
    controller.list_recipes().await.unwrap()
}

async fn create_recipe(controller: RecipeController, recipe_to_create: CreateRecipe) {
    let _ = controller.create_recipe(recipe_to_create).await;
}

impl Recipes {
    pub fn boot(recipe_controller: RecipeController) -> (Self, Task<Message>) {
        (
            Self {
                current_state: State::Loading,
                recipes: vec![],
                recipe_controller: recipe_controller.clone(),
            },
            Task::done(Message::Load),
        )
    }

    pub fn update(&mut self, message: Message) -> Task<Message> {
        match message {
            Message::Load => {
                let controller = self.recipe_controller.clone();

                Task::perform(load_recipes(controller), Message::Loaded)
            }
            Message::Loaded(recipes) => {
                self.recipes = recipes;
                self.current_state = State::Loaded;

                Task::none()
            }
            Message::Create(recipe_to_create) => {
                let controller = self.recipe_controller.clone();

                Task::perform(create_recipe(controller, recipe_to_create), |_| {
                    Message::Load
                })
            }
        }
    }

    pub fn view(&self) -> Element<'_, Message> {
        let recipes = column(self.recipes.iter().map(|r| text(r.name.clone()).into()));

        let s = match self.current_state {
            State::Loading => text("Loading..."),
            State::Loaded => text("Loaded"),
        };

        column![s, recipes].into()
    }
}
