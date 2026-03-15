use iced::{
    Element, Task,
    widget::{self, button, column},
};

use crate::views::recipes;
use crate::{init, services::recipe, views::ingredients};

enum Screen {
    Recipes(recipes::Recipes),
    Ingrendients(ingredients::Ingrendients),
}

pub struct App {
    current_screen: Option<Screen>,
    recipe_service: Option<init::RecipeService>,
    ingredient_service: Option<init::IngredientService>,
}

#[derive(Debug, Clone)]
pub enum Message {
    Booted((init::RecipeService, init::IngredientService)),
    ChangeScreen,
    RecipesMSG(recipes::Message),
    IngredientsMSG(ingredients::Message),
}

impl App {
    pub fn boot() -> (Self, Task<Message>) {
        (
            Self {
                current_screen: None,
                recipe_service: None,
                ingredient_service: None,
            },
            iced::Task::perform(init::init(), Message::Booted),
        )
    }

    pub fn update(&mut self, message: Message) -> Task<Message> {
        match message {
            Message::RecipesMSG(message) => {
                if let Some(Screen::Recipes(recipes)) = &mut self.current_screen {
                    recipes.update(message);
                }
                Task::none()
            }
            Message::IngredientsMSG(message) => {
                if let Some(Screen::Ingrendients(ingredients)) = &mut self.current_screen {
                    ingredients.update(message);
                }

                Task::none()
            }
            Message::ChangeScreen => match &self.current_screen {
                Some(Screen::Ingrendients(_ingredients)) => {
                    let recipe_service = self.recipe_service.clone().unwrap();

                    let (recipes, t) = recipes::Recipes::boot(recipe_service);

                    self.current_screen = Some(Screen::Recipes(recipes));

                    t.map(Message::RecipesMSG)
                }
                Some(Screen::Recipes(_recipes)) => {
                    self.current_screen =
                        Some(Screen::Ingrendients(ingredients::Ingrendients::new()));

                    Task::none()
                }
                None => Task::none(),
            },
            Message::Booted((recipe_service, ingredient_service)) => {
                self.recipe_service = Some(recipe_service.clone());
                self.ingredient_service = Some(ingredient_service);

                let (recipes, t) = recipes::Recipes::boot(recipe_service);

                self.current_screen = Some(Screen::Recipes(recipes));

                t.map(Message::RecipesMSG)
            }
        }
    }

    pub fn view(&self) -> Element<'_, Message> {
        column![
            button("Change").on_press(Message::ChangeScreen),
            match &self.current_screen {
                Some(Screen::Ingrendients(ingredients)) =>
                    ingredients.view().map(Message::IngredientsMSG),
                Some(Screen::Recipes(recipes)) => recipes.view().map(Message::RecipesMSG),
                None => widget::text!("Booting...").into(),
            }
        ]
        .into()
    }
}
