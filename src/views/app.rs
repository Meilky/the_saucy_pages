use std::sync::Arc;

use iced::{
    Element, Task,
    widget::{self, button, column},
};

use crate::{
    init,
    services::{
        ingredient::ImplIngredientService, ingredient::IngredientService,
        recipe::ImplRecipeService, recipe::RecipeService,
    },
    views::ingredients,
};
use crate::{repositories::recipe, views::recipes};

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
    CounterA(recipes::Message),
    CounterB(ingredients::Message),
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
            Message::CounterA(message) => {
                if let Some(Screen::Recipes(recipes)) = &mut self.current_screen {
                    recipes.update(message);
                }
                Task::none()
            }
            Message::CounterB(message) => {
                if let Some(Screen::Ingrendients(ingredients)) = &mut self.current_screen {
                    ingredients.update(message);
                }

                Task::none()
            }
            Message::ChangeScreen => match &self.current_screen {

                Some(Screen::Ingrendients(_ingredients)) => {
                    self.current_screen = Some(Screen::Recipes(recipes::Recipes::new()));

                    Task::none()
                }
                Some(Screen::Recipes(_recipes)) => {
                    self.current_screen =
                        Some(Screen::Ingrendients(ingredients::Ingrendients::new()));

                    Task::none()
                }
                None => Task::none(),
            },
            Message::Booted((recipe_service, ingredient_service)) => {
                self.recipe_service = Some(recipe_service);
                self.ingredient_service = Some(ingredient_service);
                self.current_screen = Some(Screen::Recipes(recipes::Recipes::new()));

                Task::none()
            }
        }
    }

    pub fn view(&self) -> Element<'_, Message> {
        column![
            button("Change").on_press(Message::ChangeScreen),
            match &self.current_screen {
                Some(Screen::Ingrendients(ingredients)) =>
                    ingredients.view().map(Message::CounterB),
                Some(Screen::Recipes(recipes)) => recipes.view().map(Message::CounterA),
                None => widget::text!("Booting...").into(),
            }
        ]
        .into()
    }
}
