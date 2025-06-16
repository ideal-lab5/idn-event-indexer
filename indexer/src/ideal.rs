use ideal_metadata::ideal_metadata::{
	runtime_types::{
		frame_system::pallet::Event as SystemEvent,
		pallet_balances::pallet::Event as BalancesEvent,
		// IDN-specific pallet events - now with correct names from updated fork
		pallet_idn_manager::pallet::Event as IdnManagerEvent,
		// pallet_randomness_beacon::pallet::Event as RandomnessBeaconEvent, // May not be in
		// runtime yet Add other relevant event imports for your ideal network
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
		// Let's debug print event names to see what's available
		println!("Event pallet: {}, variant: {}", event.pallet_name(), event.variant_name());

		Ok(match event.as_root_event::<Event>()? {
			Event::System(event) => {
				index_system_event![SystemEvent, event, indexer, block_number, event_index]
			},
			Event::Balances(event) => {
				index_balances_event![BalancesEvent, event, indexer, block_number, event_index]
			},
			// IDN-specific event handlers - using traditional pattern matching like other Substrate
			// events
			Event::IdnManager(event) => {
				match event {
					IdnManagerEvent::SubscriptionCreated { sub_id } => {
						// Convert H256 to u32 for the subscription ID
						// Taking the first 4 bytes as u32 (this might need adjustment based on your
						// actual type)
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
			// Event::RandomnessBeacon(_) => {
			// 	index_randomness_beacon_event![RandomnessBeaconEvent, event, indexer, block_number,
			// event_index] },
			// Add other event handlers as needed
			_ => 0,
		})
	}
}
