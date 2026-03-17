use iced::{
    Element, Task,
    widget::{button, column, text, text_input},
};

use crate::{
    controllers::ingredient::IngredientController as _,
    init::IngredientController,
    models::ingredient::{CreateIngredient, Ingredient},
};

enum State {
    Loading,
    Loaded,
}

pub struct IngredientsView {
    name_input_content: String,
    description_input_content: String,
    current_state: State,
    ingredient_controller: IngredientController,
    ingredients: Vec<Ingredient>,
}

#[derive(Debug, Clone)]
pub enum Message {
    Load,
    Loaded(Vec<Ingredient>),
    Create(CreateIngredient),
    NameInputUpdated(String),
    DescriptionInputUpdated(String),
    FormSubmited,
}

async fn load_ingredients(controller: IngredientController) -> Vec<Ingredient> {
    controller.list_ingredients().await.unwrap()
}

async fn create_ingredient(
    controller: IngredientController,
    ingredient_to_create: CreateIngredient,
) {
    let _ = controller.create_ingredient(ingredient_to_create).await;
}

impl IngredientsView {
    pub fn boot(ingredient_controller: IngredientController) -> (Self, Task<Message>) {
        (
            Self {
                name_input_content: String::new(),
                description_input_content: String::new(),
                current_state: State::Loading,
                ingredient_controller: ingredient_controller,
                ingredients: vec![],
            },
            Task::done(Message::Load),
        )
    }

    pub fn update(&mut self, message: Message) -> Task<Message> {
        match message {
            Message::Load => {
                let controller = self.ingredient_controller.clone();

                Task::perform(load_ingredients(controller), Message::Loaded)
            }
            Message::Loaded(ingredients) => {
                self.ingredients = ingredients;
                self.current_state = State::Loaded;

                Task::none()
            }
            Message::Create(ingredient_to_create) => {
                let controller = self.ingredient_controller.clone();

                Task::perform(create_ingredient(controller, ingredient_to_create), |_| {
                    Message::Load
                })
            }
            Message::NameInputUpdated(content) => {
                self.name_input_content = content;
                Task::none()
            }
            Message::DescriptionInputUpdated(content) => {
                self.description_input_content = content;
                Task::none()
            }
            Message::FormSubmited => {
                let name = self.name_input_content.clone();
                let mut description: Option<String> = None;

                if !self.description_input_content.is_empty() {
                    description = Some(self.description_input_content.clone());
                }

                let ingredient_to_create = CreateIngredient { name, description };

                Task::done(Message::Create(ingredient_to_create))
            }
        }
    }

    pub fn view(&self) -> Element<'_, Message> {
        let ingredients = column(self.ingredients.iter().map(|r| text(r.name.clone()).into()));

        let s = match self.current_state {
            State::Loading => text("Loading..."),
            State::Loaded => text("Loaded"),
        };

        let create_form = column![
            text_input("Name", &self.name_input_content).on_input(Message::NameInputUpdated),
            text_input("Description", &self.description_input_content)
                .on_input(Message::DescriptionInputUpdated),
            button("Submit").on_press(Message::FormSubmited)
        ];

        column![s, ingredients, create_form].into()
    }
}
