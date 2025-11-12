# Rust Bybit Trading Bot Simulator

![Rust](https://img.shields.io/badge/Rust-000000?style=for-the-badge&logo=rust&logoColor=white)
![Bybit](https://img.shields.io/badge/Bybit-FF6A00?style=for-the-badge&logo=data:image/png;base64)

## Overview

This project is a **Trading Bot Simulator** written in Rust that connects to **Bybit** to fetch live market data and simulates trading strategies such as **SMA (Simple Moving Average)**.  
All trades are **simulated**, no real orders are placed — safe for testing strategies.

The project supports:
- Live market data via **WebSocket**
- Backtesting on historical CSV data
- Multiple strategies (SMA implemented, extensible for others)
- Logging and detailed trade reports
- CLI interface for flexible use

---

## Features

- ✅ Fetch live prices from Bybit via REST and WebSocket
- ✅ Simulate trading strategies
- ✅ Backtest strategies on historical data
- ✅ CLI interface (`--mode live|backtest`)
- ✅ Logging and trade analytics
- ✅ Configurable parameters via `config.toml`

---

## Getting Started

### Prerequisites

- Rust >= 1.70
- Cargo
- Internet connection for live mode

### Installation

```bash
git clone https://github.com/bobrovskij-vadim/bybit_trading_bot
cd bybit_trading_bot
cargo build --release
```

### Configuration

Edit `config.toml` to configure:

```toml
[general]
mode = "live" # or "backtest"
pair = "BTCUSDT"
timeframe = "1h"

[strategy.sma]
short_period = 10
long_period = 30
```

---

### Usage

#### Live mode
```bash
cargo run -- --mode live
```

#### Backtesting
```bash
cargo run -- --mode backtest --file data/btcusdt_1h.csv
```

---

## Project Structure

```
src/
├── main.rs
├── api/           # Bybit REST and WS API modules
├── strategy/      # Strategies (SMA, RSI, etc.)
├── simulator/     # Trading engine and backtesting
├── utils.rs       # Helpers

data/               # Historical CSV files
config.toml        # Bot configuration
Cargo.toml
```

---

## Learning Outcomes

By building this project you will learn:
- Asynchronous Rust programming (`tokio`, `async/await`)
- Working with WebSocket and REST APIs
- Implementing trading strategies
- Backtesting and trade simulation
- CLI applications in Rust
- Logging, configuration, and modular architecture

---

## Future Improvements

- Multiple trading pairs support
- Additional trading strategies (RSI, MACD, etc.)
- Web dashboard / visualization
- Docker deployment
- Unit testing for strategies and engine

---

## License

MIT License

