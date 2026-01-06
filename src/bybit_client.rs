use serde::Deserialize;
use std::collections::HashMap;


pub const BYBIT_WS_URL: &str = "https://api.bybit.com/v5/market/tickers?category=spot";

pub const SYMBOLS: [&str; 10] = [
    "BTCUSDT",  "ETHUSDT",   "SOLUSDT", "XRPUSDT",  "TRXUSDT",
    "ENAUSDT",  "ETHFIUSDT", "APTUSDT", "STRKUSDT", "ASTERUSDT"  
];

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct BybitResponseV5<T> {
    ret_code: i32,
    ret_msg: String,
    result: V5Result<T>,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct V5Result<T> {
    list: T, 
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct TickerInfo {
    symbol: String,
    last_price: String, 
    bid1_price: String,
    ask1_price: String,
}

pub async fn fetch_tickers() -> String {
    let response = reqwest::get(BYBIT_WS_URL).await;

    match response {
        Ok(resp) => {
            let data = resp.json::<BybitResponseV5<Vec<TickerInfo>>>().await;
            
            match data {
                Ok(res) if res.ret_code == 0 => {
                    let ticker_map: HashMap<String, TickerInfo> = res.result.list
                        .into_iter()
                        .map(|t| (t.symbol.clone(), t))
                        .collect();
                    
                    let mut output = String::from(format!(
                        "{:<14} | {:<18} | {:<18} | {:<18}\n", 
                        "Symbol", "Price", "Bid", "Ask"
                    ));
                    
                    output.push_str("----------------------------------------------------------------------------\n");
                    
                    for symbol in SYMBOLS.iter() {
                        if let Some(t) = ticker_map.get(*symbol) {
                            output.push_str(&format!(
                                "{:<12} | {:<18} | {:<18} | {:<18}\n", 
                                t.symbol, t.last_price, t.bid1_price, t.ask1_price
                            ));
                        }
                    }
                    output
                }
                Ok(res) => format!("API Error: {}", res.ret_msg),
                Err(e) => format!("JSON Error: {}", e),
            }
        }
        Err(e) => format!("Connection error: {}", e),
    }
}