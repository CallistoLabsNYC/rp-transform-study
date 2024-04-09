use serde::Deserialize;
use crate::CryptoCandle;

/*
COINBASE payload
"{
    \"type\":\"ticker\",
    \"sequence\":58303904263,
    \"product_id\":\"ETH-USD\",
    \"price\":\"3659.37\",
    \"open_24h\":\"3395.8\",
    \"volume_24h\":\"94855.97317284\",
    \"low_24h\":\"3369.06\",
    \"high_24h\":\"3671.07\",
    \"volume_30d\":\"3689477.67843382\",
    \"best_bid\":\"3659.36\",
    \"best_bid_size\":\"1.29804582\",
    \"best_ask\":\"3659.38\",
    \"best_ask_size\":\"0.43569488\",
    \"side\":\"buy\",
    \"time\":\"2024-04-08T16:58:47.908116Z\",
    \"trade_id\":512218036,
    \"last_size\":\"0.05642456\"
}"
*/

#[derive(Deserialize)]
pub struct CoinbaseMessage {
    price: String,
    open_24h: String,
    volume_24h: String,
    low_24h: String,
    high_24h: String,
}

impl TryFrom<CoinbaseMessage> for CryptoCandle {
    type Error = &'static str;
    fn try_from(m: CoinbaseMessage) -> Result<Self, Self::Error> {
        let timestamp = m.price.parse().unwrap_or_default();
        let open = m.open_24h.parse().unwrap_or_default();
        let high = m.high_24h.parse().unwrap_or_default();
        let low = m.low_24h.parse().unwrap_or_default();
        let close = 0_f64;
        let volume = m.volume_24h.parse().unwrap_or_default();

        Ok(CryptoCandle {
            timestamp,
            open,
            high,
            low,
            close,
            volume,
            source: "Coinbase".to_owned(),
        })
    }
}
