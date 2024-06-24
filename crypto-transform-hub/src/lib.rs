pub mod binance;
pub mod coinbase;
pub mod okx;

use std::time::{SystemTime, UNIX_EPOCH};
use serde::{Deserialize, Serialize};

#[derive(Deserialize)]
#[serde(untagged)]
pub enum CryptoMessage {
    Binance(binance::BinanceMessage),
    Coinbase(coinbase::CoinbaseMessage),
    Okx(okx::OkxMessage),
}

#[derive(Debug, Serialize)]
pub struct CryptoCandle {
    pub open: f64,
    pub high: f64,
    pub low: f64,
    pub close: f64,
    pub volume: f64,
    pub timestamp: f64,
    pub source: String,
    pub symbol: String
}

pub fn now() -> i64 {
    SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .expect("Time went backwards")
        .as_millis() as i64
}