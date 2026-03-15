use iced::{
    Element,
    widget::{button, column, text},
};

pub struct Recipes {
    value: i32,
}

#[derive(Debug, Clone, Copy)]
pub enum Message {
    Increment,
    Decrement,
}

impl Recipes {
    pub fn new() -> Self {
        Self { value: 0 }
    }

    pub fn view(&self) -> Element<'_, Message> {
        column![
            button("+ r").on_press(Message::Increment),
            text(self.value).size(50),
            button("- r").on_press(Message::Decrement),
        ]
        .into()
    }

    pub fn update(&mut self, message: Message) {
        match message {
            Message::Increment => {
                self.value += 1;
            }
            Message::Decrement => {
                self.value -= 1;
            }
        }
    }
}
