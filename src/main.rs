mod gui;
// mod bybit_client;

use gui::MyWindow;
use iced::{Settings, Sandbox};

// use bybit_client::run_client;
// use reqwest::Error;

#[tokio::main]
// async fn main() -> Result<(), Error> {
async fn main() -> iced::Result {    

    MyWindow::run(Settings::default())

    // Ok(())
}



// #[tokio::main]
// async fn main() -> Result<(), Error> {

//     MyWindow::run(Settings::default())
//     //run_client().await?;

//     Ok(())
// }
 