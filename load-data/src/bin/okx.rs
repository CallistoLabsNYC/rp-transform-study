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

    let bootstrap_addrs = vec![BrokerAddress {
        host: String::from("localhost"),
        port: 9092,
    }];
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

    tracing::info!("Connecting");
    let (mut socket, _) = connect_async(url::Url::parse(&url).unwrap()).await.unwrap();
    tracing::info!("Connected to {}", url);
    tracing::info!("Starting subscription to {}", instrument_id);
    socket.send(subscription_payload.into()).await.unwrap();
    tracing::info!("Connected");

    let stream = socket
        .enumerate()
        .filter_map(|(_, message)| async {
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
        })
        .chunks(3);

    let output_stream =
        ProducerBuilder::<TcpConnection>::new(bootstrap_addrs.clone(), vec![topic.to_string()])
            .await
            .unwrap()
            .build_from_stream(stream)
            .await;

    tokio::pin!(output_stream);
    while (output_stream.next().await).is_some() {
        tracing::info!("Batch sent");
    }
}
