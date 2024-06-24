use serde::Deserialize;
use crate::{CryptoCandle, now};

/*
BINANCE payload
{
  "id": "1dbbeb56-8eea-466a-8f6e-86bdcfa2fc0b",
  "status": 200,
  "result": [
    [
      1655971200000,      // Kline open time
      "0.01086000",       // Open price
      "0.01086600",       // High price
      "0.01083600",       // Low price
      "0.01083800",       // Close price
      "2290.53800000",    // Volume
      1655974799999,      // Kline close time
      "24.85074442",      // Quote asset volume
      2283,               // Number of trades
      "1171.64000000",    // Taker buy base asset volume
      "12.71225884",      // Taker buy quote asset volume
      "0"                 // Unused field, ignore
    ]
  ],
  "rateLimits": [
    {
      "rateLimitType": "REQUEST_WEIGHT",
      "interval": "MINUTE",
      "intervalNum": 1,
      "limit": 6000,
      "count": 2
    }
  ]
}
*/

#[derive(Deserialize, Clone)]
#[serde(untagged)]
enum MultipleTypes {
    Str(String),
    F64(f64),
}

#[derive(Deserialize)]
pub struct BinanceMessage {
    result: Vec<Vec<MultipleTypes>>,
}

fn convert_types(mt: MultipleTypes) -> f64 {
    match mt {
        MultipleTypes::Str(s) => s.parse().unwrap_or_default(),
        MultipleTypes::F64(n) => n,
    }
}

impl TryFrom<BinanceMessage> for CryptoCandle {
    type Error = &'static str;
    fn try_from(m: BinanceMessage) -> Result<Self, Self::Error> {
        if m.result.len() == 0 || m.result[0].len() == 0 {
            return Err("missing data");
        }

        let timestamp = now() as f64;
        let open = convert_types(m.result[0][1].clone());
        let high = convert_types(m.result[0][2].clone());
        let low = convert_types(m.result[0][3].clone());
        let close = convert_types(m.result[0][4].clone());
        let volume = convert_types(m.result[0][5].clone());

        Ok(CryptoCandle {
            timestamp,
            open,
            high,
            low,
            close,
            volume,
            symbol: "BNBBTC".to_owned(),
            source: "Binance".to_owned(),
        })
    }
}