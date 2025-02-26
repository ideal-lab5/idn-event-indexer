FROM rust:slim

WORKDIR /usr/src/ideal-network-indexer

COPY . .

RUN cargo build --release -j 1

EXPOSE 8172
