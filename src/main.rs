mod gui;
mod bybit_client;

use gui::MyWindow;
use iced::{Settings, Application};

#[tokio::main]
async fn main() -> iced::Result {
    MyWindow::run(Settings::default())
}
