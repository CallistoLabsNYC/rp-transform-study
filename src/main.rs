use postgres::{Client, NoTls};
use std::env;
use redpanda_transform_sdk::*;
use anyhow::Result;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
struct PageView {
    page_name: String,
    user_id: i32,
}

fn main() {
    on_record_written(my_transform);
}

// This will be called for each record in the source topic.
//
// The output records returned will be written to the destination topic.
fn my_transform(event: WriteEvent, writer: &mut RecordWriter) -> Result<()> {
    let event: PageView = serde_json::from_str(event.record.value())?;

    let mut client = Client::connect(&env::var("DATABASE_URL").unwrap(), NoTls)?;
    client.execute(
        r#"
INSERT INTO page_view ( page_name, user_id )
VALUES ( $1, $2 )
        "#,
        &[&event.page_name,
        &event.user_id]
    )?;

    // writer.write(event.record)?;
    Ok(())
}