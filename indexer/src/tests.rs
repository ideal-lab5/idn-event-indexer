use acuity_index_substrate::{shared::*, substrate::*, websockets::*, *};

use crate::{ChainKey, IdealIndexer};

#[tokio::test]
async fn test_process_msg_account_balance() {
	let db_config = sled::Config::new().temporary(true);
	let trees = open_trees::<IdealIndexer>(db_config).unwrap();
	let indexer = Indexer::<IdealIndexer>::new_test(trees.clone());
	let account_id = Bytes32([1; 32]); // Example account ID
	let key = Key::Chain(ChainKey::AccountId(account_id));
	indexer.index_event(key.clone(), 4, 5).unwrap();
	indexer.index_event(key.clone(), 8, 5).unwrap();
	indexer.index_event(key.clone(), 10, 5).unwrap();

	let response = process_msg_get_events::<IdealIndexer>(&trees, key.clone());

	let ResponseMessage::Events { key: response_key, events } = response else {
		panic!("Wrong response message.");
	};
	assert_eq!(key, response_key);
	assert_eq!(events.len(), 3);
	assert_eq!(events[0].block_number, 10);
	assert_eq!(events[1].block_number, 8);
	assert_eq!(events[2].block_number, 4);
}

#[tokio::test]
async fn test_process_msg_system_event() {
	let db_config = sled::Config::new().temporary(true);
	let trees = open_trees::<IdealIndexer>(db_config).unwrap();
	let indexer = Indexer::<IdealIndexer>::new_test(trees.clone());
	let block_hash = Bytes32([2; 32]); // Example block hash
	let key = Key::Chain(ChainKey::BlockHash(block_hash));
	indexer.index_event(key.clone(), 4, 5).unwrap();
	indexer.index_event(key.clone(), 8, 5).unwrap();
	indexer.index_event(key.clone(), 10, 5).unwrap();

	let response = process_msg_get_events::<IdealIndexer>(&trees, key.clone());

	let ResponseMessage::Events { key: response_key, events } = response else {
		panic!("Wrong response message.");
	};
	assert_eq!(key, response_key);
	assert_eq!(events.len(), 3);
	assert_eq!(events[0].block_number, 10);
	assert_eq!(events[1].block_number, 8);
	assert_eq!(events[2].block_number, 4);
}

#[tokio::test]
async fn test_process_msg_extrinsic_hash() {
	let db_config = sled::Config::new().temporary(true);
	let trees = open_trees::<IdealIndexer>(db_config).unwrap();
	let indexer = Indexer::<IdealIndexer>::new_test(trees.clone());
	let extrinsic_hash = Bytes32([3; 32]); // Example extrinsic hash
	let key = Key::Chain(ChainKey::ExtrinsicHash(extrinsic_hash));
	indexer.index_event(key.clone(), 4, 5).unwrap();
	indexer.index_event(key.clone(), 8, 5).unwrap();
	indexer.index_event(key.clone(), 10, 5).unwrap();

	let response = process_msg_get_events::<IdealIndexer>(&trees, key.clone());

	let ResponseMessage::Events { key: response_key, events } = response else {
		panic!("Wrong response message.");
	};
	assert_eq!(key, response_key);
	assert_eq!(events.len(), 3);
	assert_eq!(events[0].block_number, 10);
	assert_eq!(events[1].block_number, 8);
	assert_eq!(events[2].block_number, 4);
}

// IDN-specific tests for SubstrateKey::SubscriptionId indexing using upstream types

#[tokio::test]
async fn test_subscription_id_indexing() {
	let db_config = sled::Config::new().temporary(true);
	let trees = open_trees::<IdealIndexer>(db_config).unwrap();
	let indexer = Indexer::<IdealIndexer>::new_test(trees.clone());

	// Create different subscription IDs using upstream SubscriptionId type
	let sub_id_1 = SubscriptionId::from([1u8; 32]);
	let sub_id_2 = SubscriptionId::from([2u8; 32]);

	let key1 = Key::Substrate(SubstrateKey::SubscriptionId(sub_id_1));
	let key2 = Key::Substrate(SubstrateKey::SubscriptionId(sub_id_2));

	// Index events for subscription 1
	indexer.index_event(key1.clone(), 100, 0).unwrap();
	indexer.index_event(key1.clone(), 100, 1).unwrap();

	// Index events for subscription 2
	indexer.index_event(key2.clone(), 101, 0).unwrap();

	// Verify subscription 1 events
	let response1 = process_msg_get_events::<IdealIndexer>(&trees, key1.clone());
	let ResponseMessage::Events { key: response_key1, events: events1 } = response1 else {
		panic!("Wrong response message for subscription 1.");
	};
	assert_eq!(key1, response_key1);
	assert_eq!(events1.len(), 2);
	assert_eq!(events1[0].block_number, 100);
	assert_eq!(events1[0].event_index, 1); // Latest event index first
	assert_eq!(events1[1].block_number, 100);
	assert_eq!(events1[1].event_index, 0);

	// Verify subscription 2 events
	let response2 = process_msg_get_events::<IdealIndexer>(&trees, key2.clone());
	let ResponseMessage::Events { key: response_key2, events: events2 } = response2 else {
		panic!("Wrong response message for subscription 2.");
	};
	assert_eq!(key2, response_key2);
	assert_eq!(events2.len(), 1);
	assert_eq!(events2[0].block_number, 101);
	assert_eq!(events2[0].event_index, 0);
}

#[tokio::test]
async fn test_h256_to_subscription_id_conversion() {
	// Test the upstream SubscriptionId type conversion
	let test_bytes = [
		0x78, 0x56, 0x34, 0x12, 0xAB, 0xCD, 0xEF, 0x00, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
		0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
	];
	let sub_id = SubscriptionId::from(test_bytes);

	// Verify that the SubscriptionId stores the full [u8; 32]
	assert_eq!(<SubscriptionId as AsRef<[u8; 32]>>::as_ref(&sub_id), &test_bytes);

	// Test another case
	let test_bytes2 = [
		0xFF, 0x00, 0x00, 0x00, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
		0, 0, 0, 0, 0, 0,
	];
	let sub_id2 = SubscriptionId::from(test_bytes2);
	assert_eq!(<SubscriptionId as AsRef<[u8; 32]>>::as_ref(&sub_id2), &test_bytes2);
}

#[tokio::test]
async fn test_multiple_subscription_events_same_block() {
	let db_config = sled::Config::new().temporary(true);
	let trees = open_trees::<IdealIndexer>(db_config).unwrap();
	let indexer = Indexer::<IdealIndexer>::new_test(trees.clone());

	let sub_id = SubscriptionId::from([1u8; 32]);
	let key = Key::Substrate(SubstrateKey::SubscriptionId(sub_id));
	let block_number = 100;

	// Index multiple events in the same block
	indexer.index_event(key.clone(), block_number, 0).unwrap();
	indexer.index_event(key.clone(), block_number, 5).unwrap();
	indexer.index_event(key.clone(), block_number, 2).unwrap();

	let response = process_msg_get_events::<IdealIndexer>(&trees, key.clone());
	let ResponseMessage::Events { key: response_key, events } = response else {
		panic!("Wrong response message.");
	};
	assert_eq!(key, response_key);
	assert_eq!(events.len(), 3);

	// Events should be ordered by event_index descending within the same block
	assert_eq!(events[0].event_index, 5);
	assert_eq!(events[1].event_index, 2);
	assert_eq!(events[2].event_index, 0);
}

#[tokio::test]
async fn test_subscription_events_across_blocks() {
	let db_config = sled::Config::new().temporary(true);
	let trees = open_trees::<IdealIndexer>(db_config).unwrap();
	let indexer = Indexer::<IdealIndexer>::new_test(trees.clone());

	let sub_id = SubscriptionId::from([1u8; 32]);
	let key = Key::Substrate(SubstrateKey::SubscriptionId(sub_id));

	// Index events across multiple blocks in non-sequential order
	indexer.index_event(key.clone(), 105, 1).unwrap();
	indexer.index_event(key.clone(), 100, 0).unwrap();
	indexer.index_event(key.clone(), 102, 3).unwrap();

	let response = process_msg_get_events::<IdealIndexer>(&trees, key.clone());
	let ResponseMessage::Events { key: response_key, events } = response else {
		panic!("Wrong response message.");
	};
	assert_eq!(key, response_key);
	assert_eq!(events.len(), 3);

	// Events should be ordered by block_number descending, then event_index descending
	assert_eq!(events[0].block_number, 105);
	assert_eq!(events[0].event_index, 1);
	assert_eq!(events[1].block_number, 102);
	assert_eq!(events[1].event_index, 3);
	assert_eq!(events[2].block_number, 100);
	assert_eq!(events[2].event_index, 0);
}
