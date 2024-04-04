use std::collections::HashMap;

use samsa::prelude::{create_topics, redpanda::adminapi::AdminAPI, BrokerConnection};

#[tokio::main]
async fn main() -> Result<(), ()> {
    println!("Hello, world!");

    let bootstrap_addrs = vec!["localhost:9092".to_owned(), "localhost:9093".to_owned()];

    let conn = BrokerConnection::new(bootstrap_addrs)
        .await
        .map_err(|err| println!("{:?}", err))
        .unwrap();

    let create_topics_res = samsa::prelude::create_topics(
        conn,
        1,
        "redpanda transform study",
        HashMap::from([("queue", 1), ("user-dlq", 1), ("post-dlq", 1)]),
    )
    .await
    .unwrap();

    let transform_metadata = TransformMetadataIn {
        name: name.to_string(),
        input_topic: topic.clone(),
        output_topics: vec![topic_2],
        ..Default::default()
    };
    let contents = std::fs::read("testdata/redpanda-identity.wasm")
        .map_err(|err| Error::ArgError(err.to_string()))?;
    client
        .deploy_wasm_transform(transform_metadata, contents)
        .await?;


    Ok(())
}
