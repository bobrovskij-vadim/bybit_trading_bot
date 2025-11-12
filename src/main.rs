use clap::{Parser, ValueEnum};
use tracing::info;
use tracing_subscriber;

#[derive(Parser)]
#[command(author, version, about, long_about = None)]
struct Cli {
    #[arg(short, long, value_enum, default_value_t = Mode::Live)]
    mode: Mode,
}

#[derive(Copy, Clone, PartialEq, Eq, PartialOrd, Ord, ValueEnum, Debug)]
enum Mode {
    Live,
    Backtest,
}

#[tokio::main]
async fn main() {
    // Initialize logging subscriber
    tracing_subscriber::fmt::init();

    // Parse CLI arguments
    let cli = Cli::parse();

    info!("Rust Bybit Trading Bot Simulator started!");
    info!("Selected mode: {:?}", cli.mode);

    match cli.mode {
        Mode::Live => {
            info!("Running in live mode (connecting to Bybit WebSocket)...");
            // TODO: Implement live mode logic
        }
        Mode::Backtest => {
            info!("Running in backtest mode...");
            // TODO: Implement backtesting logic
        }
    }

    info!("Application finished.");
}