# Ideal Network Indexer

Event indexer for the Ideal Network.

## Architecture

![Hybrid Architecture](https://raw.githubusercontent.com/ethernomad/hybrid-diagram/main/hybrid.png)

IDN Indexer uses the [Hybrid Indexer](https://github.com/hybrid-explorer/hybrid-indexer) Rust library. It can be accessed using [Hybrid Dapp](https://github.com/hybrid-explorer/hybrid-dapp).

## Configuration

The indexer requires two key configuration values that can be set via environment variables:

- `IDN_GENESIS_HASH`: The genesis hash of your Ideal Network chain (hex format without 0x prefix)
- `IDN_WS_URL`: WebSocket URL of your Ideal Network node

Example configuration:
```sh
export IDN_GENESIS_HASH="af97825bf72091072a08b9dbff88d6664e2061bcb4e28a90f17bd85572d8f8ae"
export IDN_WS_URL="ws://127.0.0.1:1234"
```

If not set, the indexer will use default values suitable for local development.

## Building

Ideal Network Indexer can be built using `cargo build`, however it is necessary to use the nightly `rustc`.

```sh
rustup default nightly
cargo build --release
```

Compiling `metadata` can take a very long time.

## Running

```
Usage: ideal-indexer [OPTIONS]

Options:
  -c, --chain <CHAIN>                Chain to index [default: ideal]
  -d, --db-path <DB_PATH>            Database path
  -u, --url <URL>                    URL of Substrate node to connect to
  -b, --block-number <BLOCK_NUMBER>  Block number to start indexing from
      --db-mode <DB_MODE>            Database mode [default: lowspace] [possible values: lowspace, highthroughput]
      --db-cache-capacity <SIZE>      Maximum size in bytes for the system page cache [default: 1024.00 MiB]
      --queue-depth <QUEUE_DEPTH>    Maximum number of concurrent requests to the chain [default: 64]
  -i, --index-variant               Index event variants
  -p, --port <PORT>                  Port to open for WebSocket queries [default: 8172]
  -v, --verbose...                   More output per occurrence
  -q, --quiet...                     Less output per occurrence
  -h, --help                         Print help
  -V, --version                      Print version
```

## Docker

First build the docker image:

```sh
docker build .
```

Run the docker image:

```sh
docker run --rm -p 8172:8172 \
  -e IDN_GENESIS_HASH="your_genesis_hash_here" \
  -e IDN_WS_URL="ws://your.node.url:port" \
  [image_hash] -p 8172
