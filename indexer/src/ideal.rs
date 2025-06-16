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
			// IDN-specific event handlers
			Event::IdnManager(event) => {
				match event {
					IdnManagerEvent::SubscriptionCreated { sub_id } => {
						// Convert H256 to u32 for the subscription ID (taking first 4 bytes)
						let id_u32 =
							u32::from_le_bytes([sub_id[0], sub_id[1], sub_id[2], sub_id[3]]);
						indexer.index_event(
							Key::Substrate(SubstrateKey::SubscriptionId(id_u32)),
							block_number,
							event_index,
						)?;
						1
					},
					IdnManagerEvent::SubscriptionTerminated { sub_id } => {
						let id_u32 =
							u32::from_le_bytes([sub_id[0], sub_id[1], sub_id[2], sub_id[3]]);
						indexer.index_event(
							Key::Substrate(SubstrateKey::SubscriptionId(id_u32)),
							block_number,
							event_index,
						)?;
						1
					},
					IdnManagerEvent::SubscriptionPaused { sub_id } => {
						let id_u32 =
							u32::from_le_bytes([sub_id[0], sub_id[1], sub_id[2], sub_id[3]]);
						indexer.index_event(
							Key::Substrate(SubstrateKey::SubscriptionId(id_u32)),
							block_number,
							event_index,
						)?;
						1
					},
					IdnManagerEvent::SubscriptionUpdated { sub_id } => {
						let id_u32 =
							u32::from_le_bytes([sub_id[0], sub_id[1], sub_id[2], sub_id[3]]);
						indexer.index_event(
							Key::Substrate(SubstrateKey::SubscriptionId(id_u32)),
							block_number,
							event_index,
						)?;
						1
					},
					IdnManagerEvent::SubscriptionReactivated { sub_id } => {
						let id_u32 =
							u32::from_le_bytes([sub_id[0], sub_id[1], sub_id[2], sub_id[3]]);
						indexer.index_event(
							Key::Substrate(SubstrateKey::SubscriptionId(id_u32)),
							block_number,
							event_index,
						)?;
						1
					},
					IdnManagerEvent::RandomnessDistributed { sub_id } => {
						let id_u32 =
							u32::from_le_bytes([sub_id[0], sub_id[1], sub_id[2], sub_id[3]]);
						indexer.index_event(
							Key::Substrate(SubstrateKey::SubscriptionId(id_u32)),
							block_number,
							event_index,
						)?;
						1
					},
					IdnManagerEvent::FeesCollected { sub_id, .. } => {
						let id_u32 =
							u32::from_le_bytes([sub_id[0], sub_id[1], sub_id[2], sub_id[3]]);
						indexer.index_event(
							Key::Substrate(SubstrateKey::SubscriptionId(id_u32)),
							block_number,
							event_index,
						)?;
						1
					},
					IdnManagerEvent::SubQuoted { .. } => {
						// This event doesn't have a subscription ID to index by
						0
					},
					IdnManagerEvent::SubscriptionDistributed { sub_id } => {
						let id_u32 =
							u32::from_le_bytes([sub_id[0], sub_id[1], sub_id[2], sub_id[3]]);
						indexer.index_event(
							Key::Substrate(SubstrateKey::SubscriptionId(id_u32)),
							block_number,
							event_index,
						)?;
						1
					},
				}
			},
			_ => 0,
		})
	}
}
