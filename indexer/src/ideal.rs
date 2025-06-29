use ideal_metadata::ideal_metadata::{
	runtime_types::{
		frame_system::pallet::Event as SystemEvent,
		pallet_balances::pallet::Event as BalancesEvent,
		// IDN-specific pallet events
		pallet_idn_manager::pallet::Event as IdnManagerEvent,
	},
	Event,
};

use crate::{
	config::{DEFAULT_URL, GENESIS_HASH},
	*,
};
use acuity_index_substrate::*;

pub struct IdealIndexer;

impl acuity_index_substrate::shared::RuntimeIndexer for IdealIndexer {
	type RuntimeConfig = subxt::PolkadotConfig;
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
			},
			Event::Balances(event) => {
				index_balances_event![BalancesEvent, event, indexer, block_number, event_index]
			},
			// IDN-specific event handlers using upstream macros
			Event::IdnManager(event) => {
				index_idn_manager_event![IdnManagerEvent, event, indexer, block_number, event_index]
			},
			_ => 0,
		})
	}
}
