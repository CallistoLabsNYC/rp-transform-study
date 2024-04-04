use futures::stream::{iter, StreamExt};
use rp_transform_study::{now, PageEvent, PageView};
use samsa::prelude::{ProduceMessage, ProducerBuilder};

#[tokio::main]
async fn main() -> Result<(), ()> {
    println!("Hello, world!");

    let bootstrap_addrs = vec!["localhost:9092".to_owned(), "localhost:9093".to_owned()];
    let topic = "queue";

    let producer_stream = iter(vec![0].into_iter()).cycle().enumerate().map(|(i, _)| {
        let value = if i / 2 == 0 {
            serde_json::to_string(&PageView {
                user_id: 3,
                page_name: String::from("index.html"),
                created_at: now(),
            })
            .unwrap()
        } else {
            serde_json::to_string(&PageEvent {
                user_id: 3,
                event_name: String::from("ButtonHover:cta-main"),
                created_at: now(),
            })
            .unwrap()
        };

        ProduceMessage {
            key: None,
            topic: topic.to_owned(),
            value: Some(bytes::Bytes::from(value)),
            headers: vec![],
            partition_id: 0,
        }
    });

    ProducerBuilder::new(bootstrap_addrs.clone(), vec![topic.to_string()])
        .await
        .unwrap()
        .build_from_stream(producer_stream)
        .await;

    Ok(())
}
