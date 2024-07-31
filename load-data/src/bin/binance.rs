use futures::{stream::StreamExt, SinkExt};
use samsa::prelude::*;
use tokio_tungstenite::connect_async;

#[tokio::main]
async fn main() {
    tracing_subscriber::fmt()
        // filter spans/events with level TRACE or higher.
        .with_max_level(tracing::Level::INFO)
        .compact()
        // Display source code file paths
        .with_file(true)
        // Display source code line numbers
        .with_line_number(true)
        // Display the thread ID an event was recorded on
        .with_thread_ids(true)
        // Don't display the event's target (module path)
        .with_target(false)
        // Build the subscriber
        .init();

    tracing::info!("Connecting");

    let bootstrap_addrs = vec![
        BrokerAddress {
            host: String::from("localhost"),
            port: 9092
        }];

    let topic = "crypto-raw";
    let url = "wss://testnet.binance.vision/ws-api/v3";

    let (mut socket, response) = connect_async(url::Url::parse(&url).unwrap()).await.unwrap();

    tracing::info!("Connected to {}", url);
    tracing::debug!("Response HTTP code: {}", response.status());
    tracing::debug!("Response headers:");
    for (ref header, _value) in response.headers() {
        tracing::debug!("* {}", header);
    }

    let producer = ProducerBuilder::<TcpConnection>::new(bootstrap_addrs.clone(), vec![topic.to_string()])
        .await
        .unwrap()
        .build()
        .await;

    loop {
        tracing::info!("sending message");
        socket.send(r#"{
            "id": "1dbbeb56-8eea-466a-8f6e-86bdcfa2fc0b",
            "method": "klines",
            "params": {
              "symbol": "BNBBTC",
              "interval": "1s",
              "limit": 1
            }
          }"#.into()).await.unwrap();
    
        tracing::info!("Connected");
    
        if let Some(message) = socket.next().await {
            tracing::info!("got message");
            match message {
                Ok(message) => {
                    let binary_data = message.into_data();
                    let m = ProduceMessage {
                        key: None,
                        topic: topic.to_owned(),
                        value: Some(bytes::Bytes::from(binary_data)),
                        headers: vec![],
                        partition_id: 0,
                    };
                    producer.sender.send(
                        m
                    ).await.unwrap();
                }
                Err(_) => break,
            }
        }

        tokio::time::sleep(tokio::time::Duration::from_secs(1)).await;
    }
    

}