# Redpanda Transform: Build a Crypto Data Hub with Rust, Redpanda, and Samsa

As the industry continues to focus on challenges surrounding streaming and real-time data sources, simplicity should be at top of mind. Event driven systems have their benefits as well as downsides. This is true for most system design, but there are things we can do to address these challenges. With large-scale, distributed applications, the network becomes a bottleneck. Messages in a Redpanda cluster could hop between Consumers more than a few times, causing a slowdown that is not always necessary. Redpanda Transforms are a perfect solution to cut out the network overhead and do our logic *directly on the brokers*.

In this article, we will use a Redpanda Transform to build a Crypto data hub that recieves prices from different exchanges and stores them in a consistent format.

## The Problem

Event driven systems benefit from decoupling different parts of an application. Redpanda is central to these types of systems because it's the platform where the message passing takes place. Most of the time, messages are Consumed from the Redpanda cluster and Produced back to a different location in the cluster. This comes with an overhead of the network. With certain operations, such as mapping or transforming messages, the network exchange is wasteful. Schema Mapping is one of these operations and we will deploy our logic to run directly on the broker with a Redpanda Transform.

## The Technologies

For this solution we choose our tools carefully. We will do our programming in Rust for two reasons. The langauge has a strong type system which lends itself to easily casting between types. Most importantly, though, the [Redpanda Transform SDK](https://docs.rs/redpanda-transform-sdk/latest/redpanda_transform_sdk/) allows us to hook into the transform features.

To communicate with Redpanda in Rust, we use a homegrown client called [Samsa](https://github.com/CallistoLabsNYC/samsa). This will allow us to Produce messages to the cluster in a lightweight and asyncronous manner.

Other tools are decided for us by the nature of the solution. We need a Redpanda cluster running; Docker will suffice for those who need to spin one up quickly. To compile and deploy our transform, we will use the `rpk` tool.

Our [repository](https://github.com/CallistoLabsNYC/rp-transform-study) holds the code and will help get everything set up.

## The Solution

Development comes in two parts: first the data sources then the transform itself.

### Data

On the data sources, we picked Binance, Okx, and Coinbase as our three Crypto exchanges. They each provide a Websocket API that reports live prices updates for assets. Seeing as they are free and do not require any sort of authentication, we recommend taking the prices with a grain of salt and to put off the High Frequency Trading dream for another day.

Here is the connection for Okx; each of the connectors follow this same pattern.

```rust
use futures::{stream::StreamExt, SinkExt};
use samsa::prelude::{ProduceMessage, ProducerBuilder};
use tokio_tungstenite::connect_async;

#[tokio::main]
async fn main() {
    //
    // 1. Set up variables
    //
    let bootstrap_addrs = vec!["localhost:9092".to_owned(), "localhost:9093".to_owned()];
    let topic = "crypto-raw";
    let url = "wss://ws.okx.com:8443/ws/v5/business";
    let instrument_id = "BTC-USD-SWAP";
    let subscription_payload = format!(
        r#"{{
        "op": "subscribe",
        "args": [
          {{
            "channel": "mark-price-candle1m",
            "instId": "{}"
          }}
        ]
      }}"#,
        instrument_id
    );

    //
    // 2. Connect to OKX & build source stream
    //
    let (mut socket, _) = connect_async(url::Url::parse(&url).unwrap()).await.unwrap();
    socket.send(subscription_payload.into()).await.unwrap();
    
    let stream = socket.enumerate().filter_map(|(i, message)| async {
        if let Ok(message) = message {
            let binary_data = message.into_data();
            Some(ProduceMessage {
                key: None,
                topic: topic.to_owned(),
                value: Some(bytes::Bytes::from(binary_data)),
                headers: vec![],
                partition_id: 0,
            })
        } else {
            None
        }
    });

    //
    // 3. Set up samsa producer
    //
    ProducerBuilder::new(bootstrap_addrs.clone(), vec![topic.to_string()])
        .await
        .unwrap()
        .build_from_stream(stream)
        .await;

    tokio::time::sleep(tokio::time::Duration::MAX).await;
}
```

First we set up our variables. The cluster URLs and our destination topic are required. We also define our Okx url and the payload to start our subscription. Next we create the websocket subscription using Tungstenite to give us a stream from the socket. Lastly, we build our Samsa Producer using the Tungstenite stream.

### Transform

On the Redpanda Transform side, we define our target data structure to be a typical Candle.

```rust
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
```

The next step is to define types for the individual crypto messages as well as how to cast them into our target type above.

```rust
//
// Define our Price type that our network message is parsed into
//
#[derive(Deserialize)]
pub struct OkxMessage {
    data: Vec<Vec<String>>,
}

//
// Define our conversion trait to the common type
//
impl TryFrom<OkxMessage> for CryptoCandle {
    type Error = &'static str;
    fn try_from(m: OkxMessage) -> Result<Self, Self::Error> {
        if m.data.len() == 0 || m.data[0].len() == 0 {
            return Err("missing data");
        }

        let timestamp = now() as f64;
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
            symbol: "BTC-USD-SWAP".to_owned(),
            source: "Okx".to_owned(),
        })
    }
}

//
// Define enum variant for each potential message type
//
#[derive(Deserialize)]
#[serde(untagged)]
pub enum CryptoMessage {
    Binance(binance::BinanceMessage),
    Coinbase(coinbase::CoinbaseMessage),
    Okx(okx::OkxMessage),
}
```

This is where Rust comes in handy. Luckily for us, we can use [serde](https://serde.rs/) to simply parse the message into something usable. This is done with `#[derive(Deserialize)]` on our `OkxMessage` type. The `TryFrom` trait in the standard library also allows you to define a mapping from one type to another that could potentially fail. And lastly we create a single enum to abstract all potential message types.

Using Rust Redpanda Transform SDK is simple, just follow the example and inject your own logic.
```rust
use redpanda_transform_sdk::*;
use std::error::Error;

use crypto_transform_hub::{CryptoCandle, CryptoMessage};

fn main() {
    // Register your transform function.
    // This is a good place to perform other setup too.
    on_record_written(my_transform);
}

// my_transform is where you read the record that was written, and then you can
// return new records that will be written to the output topic
fn my_transform(event: WriteEvent, writer: &mut RecordWriter) -> Result<(), Box<dyn Error>> {
    let value = event.record.value().unwrap();

    // 
    // Try to parse each type of message
    //
    let out: Result<CryptoCandle, &str> = match serde_json::from_slice(&value) {
        Ok(CryptoMessage::Binance(b)) => b.try_into(),
        Ok(CryptoMessage::Coinbase(c)) => c.try_into(),
        Ok(CryptoMessage::Okx(o)) => o.try_into(),
        Err(_) => {
            return Ok(())
        }
    };

    //
    // Handle errors as you wish
    //
    let out = match out {
        Ok(o) => o,
        Err(_) => {
            return Ok(())
        }
    };

    //
    // Define and write new record
    //
    let record = Record::new(
        Some(out.symbol.clone().into_bytes()),
        Some(serde_json::to_string(&out)?.into_bytes()),
    );
    writer.write(&record)?;
    Ok(())
}
```

The SDK requires you to simply call `on_record_written` on a transform function. In our transform function, we break the value out of the message and try to cast it to each type. When a type-casting succeeds, it will continue on to create the new record and write it to the output topic. If at any point an error arises, the code will just quietly pause and wait for the next message.

## Conclusion

Our tool selection results in the beginnings of a nicely designed application. The ingestion code using Samsa is powerful and repeatable for many different financial exchanges. Using the transform cuts out a network hop and simplifies our application. Since all the types and conversion code is in one place, it helps cut down on complexity. The Rust code to deserialize and cast our messages into one type is clean and concise. When it comes to large-scale, distributed applications, efficiency at every step is important.