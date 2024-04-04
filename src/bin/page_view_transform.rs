use postgres::{Client, NoTls};
use std::env;
use redpanda_transform_sdk::*;
use anyhow::Result;
use rp_transform_study::PageView;

fn main() {
    on_record_written(my_transform);
}

// This will be called for each record in the source topic.
//
// The output records returned will be written to the destination topic.
fn my_transform(event: WriteEvent, writer: &mut RecordWriter) -> Result<()> {
    let event: PageView = serde_json::from_slice(event.record.value().unwrap_or(b""))?;

    let mut client = Client::connect(&env::var("DATABASE_URL").unwrap(), NoTls)?;
    client.execute(
        r#"
INSERT INTO page_view ( page_name, user_id, created_at )
VALUES ( $1, $2, $3 )
        "#,
        &[&event.page_name,
        &event.user_id, &event.created_at]
    )?;

    // writer.write(event.record)?;
    Ok(())
}