use iced::{
    Element, Task,
    widget::{self, button, column, row},
};

use crate::views::recipes;
use crate::{init, views::ingredients};

enum Screen {
    Recipes(recipes::Recipes),
    Ingrendients(ingredients::Ingredients),
}

pub struct App {
    current_screen: Option<Screen>,
    recipe_controller: Option<init::RecipeController>,
    ingredient_controller: Option<init::IngredientController>,
}

#[derive(Debug, Clone, PartialEq)]
enum Module {
    Recipe,
    Ingredient,
    System,
}

#[derive(Debug, Clone)]
pub enum Message {
    Booted((init::RecipeController, init::IngredientController)),
    ChangeModule(Module),
    RecipesMSG(recipes::Message),
    IngredientsMSG(ingredients::Message),
}

impl App {
    pub fn boot() -> (Self, Task<Message>) {
        (
            Self {
                current_screen: None,
                recipe_controller: None,
                ingredient_controller: None,
            },
            iced::Task::perform(init::init(), Message::Booted),
        )
    }

    pub fn update(&mut self, message: Message) -> Task<Message> {
        match message {
            Message::RecipesMSG(message) => {
                if let Some(Screen::Recipes(recipes)) = &mut self.current_screen {
                    return recipes.update(message).map(Message::RecipesMSG);
                }

                Task::none()
            }
            Message::IngredientsMSG(message) => {
                if let Some(Screen::Ingrendients(ingredients)) = &mut self.current_screen {
                    ingredients.update(message)
                }

                Task::none()
            }
            Message::ChangeModule(module) => self.change_module(module),
            Message::Booted((recipe_controller, ingredient_controller)) => {
                self.recipe_controller = Some(recipe_controller.clone());
                self.ingredient_controller = Some(ingredient_controller);

                let (recipes, t) = recipes::Recipes::boot(recipe_controller);

                self.current_screen = Some(Screen::Recipes(recipes));

                t.map(Message::RecipesMSG)
            }
        }
    }

    pub fn view(&self) -> Element<'_, Message> {
        let current_screen = match &self.current_screen {
            Some(Screen::Ingrendients(ingredients)) => {
                ingredients.view().map(Message::IngredientsMSG)
            }
            Some(Screen::Recipes(recipes)) => recipes.view().map(Message::RecipesMSG),
            None => widget::text!("Booting...").into(),
        };

        let navbar = column![
            button("Recipes").on_press(Message::ChangeModule(Module::Recipe)),
            button("Ingrendients").on_press(Message::ChangeModule(Module::Ingredient)),
            button("System").on_press(Message::ChangeModule(Module::System)),
        ];

        row![navbar, current_screen].into()
    }

    fn change_module(&mut self, module: Module) -> Task<Message> {
        let current_module: Option<Module> = match &self.current_screen {
            Some(Screen::Ingrendients(_)) => Some(Module::Ingredient),
            Some(Screen::Recipes(_)) => Some(Module::Recipe),
            None => None,
        };

        if let Some(m) = current_module
            && m == module
        {
            return Task::none();
        }

        match module {
            Module::Recipe => {
                let recipe_controller = self.recipe_controller.clone().unwrap();

                let (recipes, t) = recipes::Recipes::boot(recipe_controller);

                self.current_screen = Some(Screen::Recipes(recipes));

                t.map(Message::RecipesMSG)
            }
            Module::Ingredient => {
                let ingredient_controller = self.ingredient_controller.clone().unwrap();

                let (ingredients, t) = ingredients::Ingredients::boot(ingredient_controller);

                self.current_screen = Some(Screen::Ingrendients(ingredients));

                t.map(Message::IngredientsMSG)
            }
            Module::System => Task::none(),
        }
    }
}
