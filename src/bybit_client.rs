use serde::Deserialize;

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
                    let mut output = String::from("Symbol | Price | Bid | Ask\n");
                    output.push_str("----------------------------------\n");
                    
                    for t in res.result.list.iter().filter(|t| SYMBOLS.contains(&t.symbol.as_str())) {
                        output.push_str(&format!(
                            "{:<10} | {:<10} | {:<10} | {:<10}\n", 
                            t.symbol, t.last_price, t.bid1_price, t.ask1_price
                        ));
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

// pub async fn run_client() -> Result<(), Error> {

//     let response = reqwest::get(BYBIT_WS_URL)
//         .await?
//         .json::<BybitResponseV5<Vec<TickerInfo>>>() 
//         .await?;

//     match response.ret_code {
//         0 => {
//             println!("\n✅ Successfully retrieved the list of Tickers V5:");
            
//             let all_tickets = response.result.list;

//             let filtered_tickers: Vec<TickerInfo> = all_tickets
//                 .into_iter()
//                 .filter(|ticker_info| {
//                     SYMBOLS.contains(&ticker_info.symbol.as_str())
//                 })
//                 .collect();
            

//             println!("{:<5} {:<10} {:<20} {:<20} {:<20}", "ID", "Symbol", "Last Price", "Bid (Buy)", "Ask (Sell)");
//             println!("---------------------------------------------------------------------------------");

//             for (i, ticker) in filtered_tickers.iter().enumerate() {
//                 println!("{:<5} {:<10} {:<20} {:<20} {:<20}",
//                     i + 1,
//                     ticker.symbol,
//                     ticker.last_price,
//                     ticker.bid1_price,
//                     ticker.ask1_price
//                 );
//             }
//         }
//         _ => {
//             eprintln!("❌ API V5 Error (Code: {}): {}", response.ret_code, response.ret_msg);
//         }
//     }

//     Ok(())
// }