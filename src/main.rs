use dotenv::dotenv;

use crate::views::app::App;

mod error;
mod init;
mod models;
mod repositories;
mod services;
mod views;

fn main() -> iced::Result {
    dotenv().ok();

    tracing_subscriber::fmt::init();

    iced::application(App::boot, App::update, App::view)
        .title("The Saucy Pages")
        .run()
}
