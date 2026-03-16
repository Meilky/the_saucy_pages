use iced::{
    Element, Task,
    widget::{column, text},
};

use crate::{
    controllers::ingredient::IngredientService as _, init::IngredientController,
    models::ingredient::Ingredient,
};

pub struct Ingredients {
    ingredient_controller: IngredientController,
    ingredients: Option<Vec<Ingredient>>,
}

#[derive(Debug, Clone)]
pub enum Message {
    Update(Vec<Ingredient>),
}

impl Ingredients {
    pub fn boot(ingredient_controller: IngredientController) -> (Self, Task<Message>) {
        (
            Self {
                ingredient_controller: ingredient_controller.clone(),
                ingredients: None,
            },
            Task::perform(Ingredients::load(ingredient_controller), Message::Update),
        )
    }

    pub async fn load(ingredient_controller: IngredientController) -> Vec<Ingredient> {
        ingredient_controller.list_ingredients().await.unwrap()
    }

    pub fn view(&self) -> Element<'_, Message> {
        if let Some(ingredients) = &self.ingredients {
            let col = column(ingredients.iter().map(|r| text(r.name.clone()).into()));

            return col.into();
        }

        text("Loading...").into()
    }

    pub fn update(&mut self, message: Message) {
        match message {
            Message::Update(recipes) => {
                self.ingredients = Some(recipes);
            }
        }
    }
}
