use iced::Settings;
use iced::Application;

mod app;
use crate::app::*;

fn main() {
    Calculator::run(Settings::default());
}
