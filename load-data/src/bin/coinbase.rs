use futures::{stream::StreamExt, SinkExt};
use samsa::prelude::{ProduceMessage, ProducerBuilder};
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

    let bootstrap_addrs = vec!["localhost:9092".to_owned(), "localhost:9093".to_owned()];
    let topic = "crypto-raw";
    let url = "wss://ws-feed.exchange.coinbase.com";
    let product_id = "ETH-USD";
    let subscription_payload = format!(
        r#"{{
            "type": "subscribe",
            "product_ids": [
                "{}"
            ],
            "channels": [
                {{
                    "name": "ticker",
                    "product_ids": [
                        "{}"
                    ]
                }}
            ]
        }}"#,
        product_id,
        product_id
    );

    log::info!("Connecting");
    let (mut socket, _) = connect_async(url::Url::parse(&url).unwrap()).await.unwrap();
    log::info!("Connected to {}", url);
    log::info!("Starting subscription to {}", product_id);
    socket.send(subscription_payload.into()).await.unwrap();
    log::info!("Connected");

    let stream = socket.filter_map(|message| async {
        tracing::info!("Got message");
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

    ProducerBuilder::new(bootstrap_addrs.clone(), vec![topic.to_string()])
        .await
        .unwrap()
        .build_from_stream(stream)
        .await;

    tokio::time::sleep(tokio::time::Duration::MAX).await;

}

