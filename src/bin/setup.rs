use std::collections::HashMap;

use samsa::prelude::{
    create_topics,
    redpanda::adminapi::{AdminAPI, TransformMetadataIn},
    BrokerConnection,
};

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
        HashMap::from([("queue", 1), ("page-view-dlq", 1), ("page-event-dlq", 1)]),
    )
    .await
    .unwrap();

    // let transform_metadata = TransformMetadataIn {
    //     name: "transformer".to_string(),
    //     input_topic: "queue".to_owned(),
    //     output_topics: vec!["page-view-dlq".to_owned()],
    //     ..Default::default()
    // };
    // let contents = std::fs::read("testdata/redpanda-identity.wasm")
    //     .map_err(|err| samsa::prelude::Error::ArgError(err.to_string()))?;

    // client
    //     .deploy_wasm_transform(transform_metadata, contents)
    //     .await?;

    Ok(())
}
