use serde::Deserialize;
use reqwest::Client;

#[derive(Deserialize, Debug)]
pub struct Ticker {
    pub last_price: String,
}

pub struct BybitApi {
    client: Client,
}

impl BybitApi {
    pub fn new() -> Self {
        BybitApi {
            client: Client::new(),
        }
    }

    /// Get current price for a symbol via Bybit REST API
    pub async fn get_price(&self) -> Result<Ticker, reqwest::Error> {
        let url = format!("https://api.bybit.com/v2/public/tickers?symbol=BTCUSDT");

        let resp = self.client.get(&url).send().await?;
        let json_resp = resp.json::<serde_json::Value>().await?;

        let data = &json_resp["result"][0];

        Ok(Ticker {
            last_price: data["last_price"].as_str().unwrap_or("").to_string(),
        })
    }
}