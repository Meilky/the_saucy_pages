use dotenv::dotenv;

use crate::views::app::App;

mod controllers;
mod error;
mod init;
mod models;
mod repositories;
mod views;

fn main() -> iced::Result {
    dotenv().ok();

    tracing_subscriber::fmt::init();

    iced::application(App::boot, App::update, App::view)
        .title("The Saucy Pages")
        .run()
}
