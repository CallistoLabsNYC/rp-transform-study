# Redpanda Transform Study

The intention of this study is to demonstrate the utility of Rust + Redpanda Transforms. 

We will use Rust's powerful pattern matching, type system, and `serde` to crush through integrating with multiple APIs. For this, we use live Crypto prices from 3 different exchanges.

The goal is to take 3 different Websocket APIs which give different message type. The Websocket consumers will just dump the message as is into a raw topic. The Transform will automatically do the conversion into a common type and send it to a cleaned Candle topic.

## Setup
1. [Install `rpk` on your machine](https://docs.redpanda.com/current/get-started/rpk-install/).
1. Run `docker-compose up`.
1. In the Redpanda Console, create 2 topics named `crypto-raw` and `crypto-candles`.
1. In your terminal, `cd crypto-transform-hub`
1. Run `rpk transform build && rpk transform deploy` and answer the questions with the correct topic names.
1. `cd ../load-data` and run the following commands in different terminal windows:
```
RUST_LOG=DEBUG cargo run --bin binance

RUST_LOG=DEBUG cargo run --bin coinbase

RUST_LOG=DEBUG cargo run --bin okx
```

With your topic filling up, watch your destination topic to see the normalized messages arrive.
