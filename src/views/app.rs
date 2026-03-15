use iced::{
    Element, Task,
    widget::{button, column},
};

use crate::views::ingredients;
use crate::views::recipes;

enum Screen {
    Recipes(recipes::Recipes),
    Ingrendients(ingredients::Ingrendients),
}

pub struct App {
    current_screen: Screen,
}

#[derive(Debug, Clone, Copy)]
pub enum Message {
    ChangeScreen,
    CounterA(recipes::Message),
    CounterB(ingredients::Message),
}

impl App {
    pub fn new() -> Self {
        Self {
            current_screen: Screen::Recipes(recipes::Recipes::new()),
        }
    }

    pub fn update(&mut self, message: Message) -> Task<Message> {
        match message {
            Message::CounterA(message) => {
                if let Screen::Recipes(recipes) = &mut self.current_screen {
                    recipes.update(message);
                }
            }
            Message::CounterB(message) => {
                if let Screen::Ingrendients(ingredients) = &mut self.current_screen {
                    ingredients.update(message);
                }
            }
            Message::ChangeScreen => match &self.current_screen {
                Screen::Ingrendients(_ingredients) => {
                    self.current_screen = Screen::Recipes(recipes::Recipes::new())
                }
                Screen::Recipes(_recipes) => {
                    self.current_screen = Screen::Ingrendients(ingredients::Ingrendients::new())
                }
            },
        };

        Task::none()
    }

    pub fn view(&self) -> Element<'_, Message> {
        column![
            button("Change").on_press(Message::ChangeScreen),
            match &self.current_screen {
                Screen::Ingrendients(ingredients) => ingredients.view().map(Message::CounterB),
                Screen::Recipes(recipes) => recipes.view().map(Message::CounterA),
            }
        ]
        .into()
    }
}
