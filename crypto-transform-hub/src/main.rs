use redpanda_transform_sdk::*;
use serde::{Deserialize, Serialize};
use std::error::Error;
// use tokio_stream::StreamExt;

#[derive(Debug, Serialize)]
pub struct CryptoCandle {
    open: f64,
    high: f64,
    low: f64,
    close: f64,
    volume: f64,
    timestamp: f64,
    source: String,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(untagged)]
enum MultipleTypes {
    Str(String),
    F64(f64),
}

/*
COINBASE
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
    price: f64,
    open_24h: f64,
    volume_24h: f64,
    low_24h: f64,
    high_24h: f64,
}

impl TryFrom<CoinbaseMessage> for CryptoCandle {
    type Error = &'static str;
    fn try_from(m: CoinbaseMessage) -> Result<Self, Self::Error> {
        let timestamp = m.price;
        let open = m.open_24h;
        let high = m.high_24h;
        let low = m.low_24h;
        let close = 0_f64;
        let volume = m.volume_24h;

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

/*
OKX
{
> ts	String	Opening time of the candlestick, Unix timestamp format in milliseconds, e.g. 1597026383085
> o	String	Open price
> h	String	Highest price
> l	String	Lowest price
> c	String	Close price
}
"{
    \"arg\":{
        \"channel\":\"mark-price-candle1m\",
        \"instId\":\"BTC-USD-SWAP\"
    },
    \"data\":
        [[
            \"1712597400000\",
            \"71338.4\",
            \"71338.8\",
            \"71338.1\",
            \"71338.8\",
            \"0\"
        ]]
    }"
}
 */
#[derive(Deserialize)]
pub struct OkxMessage {
    data: Vec<Vec<String>>,
}

impl TryFrom<OkxMessage> for CryptoCandle {
    type Error = &'static str;
    fn try_from(m: OkxMessage) -> Result<Self, Self::Error> {
        if m.data.len() == 0 {
            return Err("missing data");
        }

        if m.data[0].len() == 0 {
            return Err("missing data");
        }

        let timestamp = m.data[0][0].parse().unwrap_or_default();
        let open = m.data[0][1].parse().unwrap_or_default();
        let high = m.data[0][2].parse().unwrap_or_default();
        let low = m.data[0][3].parse().unwrap_or_default();
        let close = m.data[0][4].parse().unwrap_or_default();
        let volume = 0_f64;

        Ok(CryptoCandle {
            timestamp,
            open,
            high,
            low,
            close,
            volume,
            source: "Okx".to_owned(),
        })
    }
}

/*
BINANCE
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

#[derive(Deserialize)]
pub struct BinanceMessage {
    result: Vec<Vec<MultipleTypes>>,
}

impl TryFrom<BinanceMessage> for CryptoCandle {
    type Error = &'static str;
    fn try_from(m: BinanceMessage) -> Result<Self, Self::Error> {
        if m.result.len() == 0 {
            return Err("missing data");
        }

        if m.result[0].len() == 0 {
            return Err("missing data");
        }

        let timestamp = match m.result[0][0].clone() {
            MultipleTypes::Str(s) => s.parse().unwrap_or_default(),
            MultipleTypes::F64(n) => n,
        };
        let open = match m.result[0][1].clone() {
            MultipleTypes::Str(s) => s.parse().unwrap_or_default(),
            MultipleTypes::F64(n) => n,
        };
        let high = match m.result[0][2].clone() {
            MultipleTypes::Str(s) => s.parse().unwrap_or_default(),
            MultipleTypes::F64(n) => n,
        };
        let low = match m.result[0][3].clone() {
            MultipleTypes::Str(s) => s.parse().unwrap_or_default(),
            MultipleTypes::F64(n) => n,
        };
        let close = match m.result[0][4].clone() {
            MultipleTypes::Str(s) => s.parse().unwrap_or_default(),
            MultipleTypes::F64(n) => n,
        };
        let volume = match m.result[0][5].clone() {
            MultipleTypes::Str(s) => s.parse().unwrap_or_default(),
            MultipleTypes::F64(n) => n,
        };

        Ok(CryptoCandle {
            timestamp,
            open,
            high,
            low,
            close,
            volume,
            source: "Binance".to_owned(),
        })
    }
}

// #[tokio::main]
fn main() {
    // Register your transform function.
    // This is a good place to perform other setup too.
    on_record_written(my_transform);

    // let bootstrap_addrs = vec!["localhost:9092".to_owned(), "localhost:9093".to_owned()];
    // let topic = "crypto-raw";
    // let assigned_topic_partitions = HashMap::from([(topic.to_owned(), vec![0])]);

    // let stream = samsa::prelude::ConsumerBuilder::new(bootstrap_addrs, assigned_topic_partitions)
    //     .await
    //     .unwrap()
    //     .build()
    //     .into_flat_stream();

    // tokio::pin!(stream);
    // while let Some(message) = stream.next().await {
    //     let b: Result<BinanceMessage, serde_json::Error> = serde_json::from_slice(&message.value);
    //     let c: Result<CoinbaseMessage, serde_json::Error> = serde_json::from_slice(&message.value);
    //     let o: Result<OkxMessage, serde_json::Error> = serde_json::from_slice(&message.value);
    //     let out: Result<CryptoCandle, &str> = match (b, c, o) {
    //         (Ok(m), _, _) => m.try_into(),
    //         (_, Ok(m), _) => m.try_into(),
    //         (_, _, Ok(m)) => m.try_into(),
    //         (Err(e), Err(e1), Err(e2)) => {
    //             println!("{:?}\n\n{:?}\n\n{:?}", e, e1, e2);
    //             Err("None of these worked!")
    //         }
    //     };

    //     let out = out.unwrap();
    //     println!("{:?}", out);
    // }
}

// my_transform is where you read the record that was written, and then you can
// return new records that will be written to the output topic
fn my_transform(event: WriteEvent, writer: &mut RecordWriter) -> Result<(), Box<dyn Error>> {
    let value = event.record.value().unwrap();
    let b: Result<BinanceMessage, serde_json::Error> = serde_json::from_slice(&value);
    let c: Result<CoinbaseMessage, serde_json::Error> = serde_json::from_slice(&value);
    let o: Result<OkxMessage, serde_json::Error> = serde_json::from_slice(&value);
    let out: Result<CryptoCandle, &str> = match (b, c, o) {
        (Ok(m), _, _) => m.try_into(),
        (_, Ok(m), _) => m.try_into(),
        (_, _, Ok(m)) => m.try_into(),
        (Err(e), Err(e1), Err(e2)) => {
            println!("{:?}\n\n{:?}\n\n{:?}", e, e1, e2);
            Err("None of these worked!")
        }
    };

    let out = match out {
        Ok(o) => o,
        Err(_) => return Ok(()),
    };

    let record = Record::new(
        Some(out.source.clone().into_bytes()),
        Some(serde_json::to_string(&out)?.into_bytes()),
    );

    writer.write(&record)?;
    Ok(())
}
