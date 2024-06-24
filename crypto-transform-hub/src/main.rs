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

    let out: Result<CryptoCandle, &str> = match serde_json::from_slice(&value) {
        Ok(CryptoMessage::Binance(b)) => b.try_into(),
        Ok(CryptoMessage::Coinbase(c)) => c.try_into(),
        Ok(CryptoMessage::Okx(o)) => o.try_into(),
        Err(_) => {
            return Ok(())
        }
    };

    let out = match out {
        Ok(o) => o,
        Err(_) => {
            return Ok(())
        }
    };

    let record = Record::new(
        Some(out.symbol.clone().into_bytes()),
        Some(serde_json::to_string(&out)?.into_bytes()),
    );

    writer.write(&record)?;
    Ok(())
}
