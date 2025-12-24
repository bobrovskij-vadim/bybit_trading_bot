use iced::widget::{column, text, container};
use iced::{Element, Sandbox, Settings, Length};

pub struct MyWindow;

impl Sandbox for MyWindow {
    type Message = ();

    fn new() -> Self {
        Self
    }

    fn title(&self) -> String {
        String::from("Bybit trading bot")
    }

    // Event processing (button clicks, text input)
    fn update(&mut self, _message: Self::Message) {
        // There will be logic here later
    }

    // Interface rendering
    fn view(&self) -> Element<'_, Self::Message> {
        // Content: text inside a column
        let content = column![
            text("Hello world!.").size(30),
            text("Сюда мы скоро добавим аналог QTextEdit."),
        ]
        .spacing(20);
        // Centering the content in the window
        container(content)
            .width(Length::Fill)
            .height(Length::Fill)
            .center_x()
            .center_y()
            .into()
    }
}

