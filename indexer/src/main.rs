#![feature(more_qualified_paths)]
use acuity_index_substrate::{shared::*, websockets::*};
use byte_unit::Byte;
use clap::{Parser, ValueEnum};
use clap_verbosity_flag::{InfoLevel, Verbosity};
use serde::{Deserialize, Serialize};
use sled::{Db, Tree};
use tracing_log::AsTrace;
use zerocopy::AsBytes;

#[cfg(test)]
mod tests;

#[derive(Copy, Clone, PartialEq, Eq, PartialOrd, Ord, ValueEnum, Debug)]
pub enum Chain {
    Polkadot,
    Kusama,
    Rococo,
    Westend,
    Ideal,
}

#[derive(Clone, ValueEnum, Debug)]
pub enum DbMode {
    LowSpace,
    HighThroughput,
}

impl From<DbMode> for sled::Mode {
    fn from(val: DbMode) -> Self {
        match val {
            DbMode::LowSpace => sled::Mode::LowSpace,
            DbMode::HighThroughput => sled::Mode::HighThroughput,
        }
    }
}

#[derive(Clone, Debug)]
pub struct ChainTrees {
    pub auction_index: Tree,
    pub candidate_hash: Tree,
    pub para_id: Tree,
}

impl IndexTrees for ChainTrees {
    fn open(db: &Db) -> Result<Self, sled::Error> {
        Ok(ChainTrees {
            auction_index: db.open_tree(b"auction_index")?,
            candidate_hash: db.open_tree(b"candiate_hash")?,
            para_id: db.open_tree(b"para_id")?,
        })
    }

    fn flush(&self) -> Result<(), sled::Error> {
        self.auction_index.flush()?;
        self.candidate_hash.flush()?;
        self.para_id.flush()?;
        Ok(())
    }
}

#[derive(Serialize, Deserialize, Clone, Debug, Eq, PartialEq, Hash)]
#[serde(tag = "type", content = "value")]
pub enum ChainKey {
    AuctionIndex(u32),
    CandidateHash(Bytes32),
    ParaId(u32),
}

impl IndexKey for ChainKey {
    type ChainTrees = ChainTrees;

    fn write_db_key(
        &self,
        trees: &ChainTrees,
        block_number: u32,
        event_index: u16,
    ) -> Result<(), sled::Error> {
        let block_number = block_number.into();
        let event_index = event_index.into();
        match self {
            ChainKey::AuctionIndex(auction_index) => {
                let key = U32Key {
                    key: (*auction_index).into(),
                    block_number,
                    event_index,
                };
                trees.auction_index.insert(key.as_bytes(), &[])?
            }
            ChainKey::CandidateHash(candidate_hash) => {
                let key = Bytes32Key {
                    key: candidate_hash.0,
                    block_number,
                    event_index,
                };
                trees.candidate_hash.insert(key.as_bytes(), &[])?
            }
            ChainKey::ParaId(para_id) => {
                let key = U32Key {
                    key: (*para_id).into(),
                    block_number,
                    event_index,
                };
                trees.para_id.insert(key.as_bytes(), &[])?
            }
        };
        Ok(())
    }

    fn get_key_events(&self, trees: &ChainTrees) -> Vec<Event> {
        match self {
            ChainKey::AuctionIndex(auction_index) => {
                get_events_u32(&trees.auction_index, *auction_index)
            }
            ChainKey::CandidateHash(candidate_hash) => {
                get_events_bytes32(&trees.candidate_hash, candidate_hash)
            }
            ChainKey::ParaId(para_id) => get_events_u32(&trees.para_id, *para_id),
        }
    }
}

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
pub struct Args {
    /// Chain to index
    #[arg(short, long, value_enum, default_value_t = Chain::Ideal)]
    pub chain: Chain,
    /// Database path
    #[arg(short, long)]
    pub db_path: Option<String>,
    /// Database mode
    #[arg(long, value_enum, default_value_t = DbMode::LowSpace)]
    pub db_mode: DbMode,
    /// Maximum size in bytes for the system page cache
    #[arg(long, default_value = "1024.00 MiB")]
    pub db_cache_capacity: String,
    /// URL of Substrate node to connect to
    #[arg(short, long)]
    pub url: Option<String>,
    /// Maximum number of concurrent requests to the chain
    #[arg(long, default_value_t = 1)]
    pub queue_depth: u8,
    /// Index event variants
    #[arg(short, long, default_value_t = false)]
    pub index_variant: bool,
    /// Port to open for WebSocket queries
    #[arg(short, long, default_value_t = 8172)]
    pub port: u16,
    #[command(flatten)]
    verbose: Verbosity<InfoLevel>,
}

mod polkadot;
use polkadot::PolkadotIndexer;
mod kusama;
use kusama::KusamaIndexer;
mod ideal;
use ideal::IdealIndexer;
mod rococo;
use rococo::RococoIndexer;
mod westend;
use westend::WestendIndexer;
mod pallets;

#[tokio::main]
async fn main() {
    // Check command line parameters.
    let args = Args::parse();
    let db_cache_capacity = Byte::from_str(args.db_cache_capacity)
        .unwrap()
        .get_bytes()
        .try_into()
        .unwrap();
    let log_level = args.verbose.log_level_filter().as_trace();
    // Start the indexer.
    match args.chain {
        Chain::Polkadot => {
            acuity_index_substrate::start::<PolkadotIndexer>(
                args.db_path,
                args.db_mode.into(),
                db_cache_capacity,
                args.url,
                args.queue_depth,
                args.index_variant,
                args.port,
                log_level,
            )
            .await
        }
        Chain::Kusama => {
            acuity_index_substrate::start::<KusamaIndexer>(
                args.db_path,
                args.db_mode.into(),
                db_cache_capacity,
                args.url,
                args.queue_depth,
                args.index_variant,
                args.port,
                log_level,
            )
            .await
        }
        Chain::Rococo => {
            acuity_index_substrate::start::<RococoIndexer>(
                args.db_path,
                args.db_mode.into(),
                db_cache_capacity,
                args.url,
                args.queue_depth,
                args.index_variant,
                args.port,
                log_level,
            )
            .await
        }
        Chain::Westend => {
            acuity_index_substrate::start::<WestendIndexer>(
                args.db_path,
                args.db_mode.into(),
                db_cache_capacity,
                args.url,
                args.queue_depth,
                args.index_variant,
                args.port,
                log_level,
            )
            .await
        }
        Chain::Ideal => {
            acuity_index_substrate::start::<IdealIndexer>(
                args.db_path,
                args.db_mode.into(),
                db_cache_capacity,
                args.url,
                args.queue_depth,
                args.index_variant,
                args.port,
                log_level,
            )
            .await
        }
    };
}
