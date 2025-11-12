use tokio_tungstenite::{connect_async, tungstenite::Message};
use futures_util::{SinkExt, StreamExt};
use tracing::info;


pub async fn subscribe_ticker(symbol: &str) {
    let url = "wss://stream.bybit.com/realtime_public";


    loop {
        match connect_async(url).await {
            Ok((ws_stream, _)) => {
                info!("Connected to Bybit WebSocket");
                let (mut write, mut read) = ws_stream.split();

                // Subscribe to ticker
                let sub_msg = format!(
                    r#"{{"op":"subscribe","args":["instrument_info.100ms.{}"]}}"#,
                    symbol
                );

                // Convert String into Utf8Bytes for tokio-tungstenite 0.28
                if let Err(e) = write.send(Message::Text(sub_msg.into())).await {
                    info!("Failed to send subscribe message: {}", e);
                    continue;
                }


                while let Some(msg) = read.next().await {
                    match msg {
                        Ok(Message::Text(txt)) => {
                            info!("Received: {}", txt);
                        }
                        Ok(_) => {}
                        Err(e) => {
                            info!("WebSocket error: {}. Reconnecting in 5s...", e);
                            tokio::time::sleep(std::time::Duration::from_secs(5)).await;
                            break;
                        }
                    }
                }
            }
            Err(e) => {
                info!("WebSocket connection failed: {}. Retrying in 5s...", e);
                tokio::time::sleep(std::time::Duration::from_secs(5)).await;
            }
        }
    }
}