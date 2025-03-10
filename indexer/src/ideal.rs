use ideal_metadata::ideal_metadata::{
    runtime_types::{
        frame_system::pallet::Event as SystemEvent,
        pallet_balances::pallet::Event as BalancesEvent,
        // Add other relevant event imports for your ideal network
    },
    Event,
};

use crate::*;
use crate::config::{GENESIS_HASH, DEFAULT_URL};
use acuity_index_substrate::*;

pub struct IdealIndexer;

impl acuity_index_substrate::shared::RuntimeIndexer for IdealIndexer {
    type RuntimeConfig = subxt::PolkadotConfig; // You might need to adjust this based on your chain's configuration
    type ChainKey = ChainKey;

    fn get_name() -> &'static str {
        "ideal"
    }

    fn get_genesis_hash() -> <Self::RuntimeConfig as subxt::Config>::Hash {
        (*GENESIS_HASH).into()
    }

    fn get_versions() -> &'static [u32] {
        &[0]
    }

    fn get_default_url() -> &'static str {
        &DEFAULT_URL
    }

    fn process_event(
        indexer: &acuity_index_substrate::substrate::Indexer<Self>,
        block_number: u32,
        event_index: u16,
        event: subxt::events::EventDetails<Self::RuntimeConfig>,
    ) -> Result<u32, IndexError> {
        Ok(match event.as_root_event::<Event>()? {
            Event::System(event) => {
                index_system_event![SystemEvent, event, indexer, block_number, event_index]
            }
            Event::Balances(event) => {
                index_balances_event![BalancesEvent, event, indexer, block_number, event_index]
            }
            // Add other event handlers as needed
            _ => 0,
        })
    }
}
