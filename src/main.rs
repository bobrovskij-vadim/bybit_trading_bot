mod bybit_client;

use bybit_client::run_client;
use reqwest::Error;

#[tokio::main]
async fn main() -> Result<(), Error> {

    run_client().await?;

    Ok(())
}
 