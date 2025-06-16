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

mod config;
mod ideal;
mod pallets;

use ideal::IdealIndexer;

// Let's add a test function to see what's available
#[allow(dead_code)]
fn print_runtime_info() {
	use ideal_metadata::ideal_metadata::Event;
	println!("Available Event variants:");

	// We can't enumerate all variants at runtime, but we can try to see what compiles
	// Let's check if the IDN variants exist by trying to compile them

	// Test if these compile (they will be dead code but help us see if the types exist)
	#[allow(unreachable_code)]
	{
		return;
		// These would only compile if the variants exist:
		let _test: Event = panic!(); // This will never run, just for type checking
		match _test {
			Event::System(_) => {},
			Event::Balances(_) => {},
			// Test if these exist:
			// Event::IdnManager(_) => {},
			// Event::RandomnessBeacon(_) => {},
			_ => {},
		}
	}
}

#[derive(Copy, Clone, PartialEq, Eq, PartialOrd, Ord, ValueEnum, Debug)]
pub enum Chain {
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
	pub account_id: Tree,
	pub block_hash: Tree,
	pub extrinsic_hash: Tree,
}

impl IndexTrees for ChainTrees {
	fn open(db: &Db) -> Result<Self, sled::Error> {
		Ok(ChainTrees {
			account_id: db.open_tree(b"account_id")?,
			block_hash: db.open_tree(b"block_hash")?,
			extrinsic_hash: db.open_tree(b"extrinsic_hash")?,
		})
	}

	fn flush(&self) -> Result<(), sled::Error> {
		self.account_id.flush()?;
		self.block_hash.flush()?;
		self.extrinsic_hash.flush()?;
		Ok(())
	}
}

#[derive(Serialize, Deserialize, Clone, Debug, Eq, PartialEq, Hash)]
#[serde(tag = "type", content = "value")]
pub enum ChainKey {
	AccountId(Bytes32),
	BlockHash(Bytes32),
	ExtrinsicHash(Bytes32),
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
			ChainKey::AccountId(account_id) => {
				let key = Bytes32Key { key: account_id.0, block_number, event_index };
				trees.account_id.insert(key.as_bytes(), &[])?
			},
			ChainKey::BlockHash(block_hash) => {
				let key = Bytes32Key { key: block_hash.0, block_number, event_index };
				trees.block_hash.insert(key.as_bytes(), &[])?
			},
			ChainKey::ExtrinsicHash(extrinsic_hash) => {
				let key = Bytes32Key { key: extrinsic_hash.0, block_number, event_index };
				trees.extrinsic_hash.insert(key.as_bytes(), &[])?
			},
		};
		Ok(())
	}

	fn get_key_events(&self, trees: &ChainTrees) -> Vec<Event> {
		match self {
			ChainKey::AccountId(account_id) => get_events_bytes32(&trees.account_id, account_id),
			ChainKey::BlockHash(block_hash) => get_events_bytes32(&trees.block_hash, block_hash),
			ChainKey::ExtrinsicHash(extrinsic_hash) =>
				get_events_bytes32(&trees.extrinsic_hash, extrinsic_hash),
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

#[tokio::main]
async fn main() {
	// Check command line parameters.
	let args = Args::parse();
	let db_cache_capacity =
		Byte::from_str(args.db_cache_capacity).unwrap().get_bytes().try_into().unwrap();
	let log_level = args.verbose.log_level_filter().as_trace();

	// Start the indexer.
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
