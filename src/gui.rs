use iced::widget::{button, column, container, text_editor, row};
use iced::{Application, Command, Element, Length, Theme, executor};
use crate::bybit_client; 

pub struct MyWindow {
    content: text_editor::Content,
}

#[derive(Debug, Clone)]
pub enum Message {
    EditorAction(text_editor::Action),
    FetchData,              
    DataReceived(String),   
}

impl Application for MyWindow {
    type Executor = executor::Default;
    type Message = Message;
    type Theme = Theme;
    type Flags = ();

    fn new(_flags: ()) -> (Self, Command<Message>) {
        (
            Self { content: text_editor::Content::new() },
            Command::none(),
        )
    }

    fn title(&self) -> String { String::from("Bybit Bot v5") }

    fn update(&mut self, message: Message) -> Command<Message> {
        match message {
            Message::EditorAction(action) => {
                self.content.perform(action);
                Command::none()
            }
            Message::FetchData => {
                Command::perform(bybit_client::fetch_tickers(), Message::DataReceived)
            }
            Message::DataReceived(data) => {
                self.content = text_editor::Content::with_text(&data);
                Command::none()
            }
        }
    }

    fn view(&self) -> Element<'_, Message> {
        let editor = text_editor(&self.content).on_action(Message::EditorAction);
        let fetch_btn = button("Update Tickers").on_press(Message::FetchData);

        container(column![fetch_btn, editor].spacing(10))
            .padding(20).width(Length::Fill).height(Length::Fill).into()
    }
}


// use iced::widget::{column, text, container};
// use iced::{Element, Sandbox, Settings, Length};
// use crate::bybit_client;

// pub struct MyWindow;

// impl Sandbox for MyWindow {
//     type Message = ();

//     fn new() -> Self {
//         Self
//     }

//     fn title(&self) -> String {
//         String::from("Bybit trading bot")
//     }

//     // Event processing (button clicks, text input)
//     fn update(&mut self, _message: Self::Message) {
//         // There will be logic here later
//     }

//     // Interface rendering
//     fn view(&self) -> Element<'_, Self::Message> {
//         // Content: text inside a column
//         let content = column![
//             text("Hello world!.").size(30),
//             text("Сюда мы скоро добавим аналог QTextEdit."),
//         ]
//         .spacing(20);
//         // Centering the content in the window
//         container(content)
//             .width(Length::Fill)
//             .height(Length::Fill)
//             .center_x()
//             .center_y()
//             .into()
//     }
// }

