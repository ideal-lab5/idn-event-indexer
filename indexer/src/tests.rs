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

// IDN-specific tests for SubstrateKey::SubscriptionId indexing

#[tokio::test]
async fn test_subscription_id_indexing() {
	let db_config = sled::Config::new().temporary(true);
	let trees = open_trees::<IdealIndexer>(db_config).unwrap();
	let indexer = Indexer::<IdealIndexer>::new_test(trees.clone());

	// Test subscription ID indexing with different IDs
	let subscription_id_1 = 12345u32;
	let subscription_id_2 = 67890u32;

	let key1 = Key::Substrate(SubstrateKey::SubscriptionId(subscription_id_1));
	let key2 = Key::Substrate(SubstrateKey::SubscriptionId(subscription_id_2));

	// Index events for subscription_id_1
	indexer.index_event(key1.clone(), 4, 5).unwrap();
	indexer.index_event(key1.clone(), 8, 10).unwrap();
	indexer.index_event(key1.clone(), 12, 15).unwrap();

	// Index events for subscription_id_2
	indexer.index_event(key2.clone(), 6, 7).unwrap();
	indexer.index_event(key2.clone(), 14, 20).unwrap();

	// Test retrieval for subscription_id_1
	let response1 = process_msg_get_events::<IdealIndexer>(&trees, key1.clone());
	let ResponseMessage::Events { key: response_key1, events: events1 } = response1 else {
		panic!("Wrong response message for subscription_id_1.");
	};
	assert_eq!(key1, response_key1);
	assert_eq!(events1.len(), 3);
	assert_eq!(events1[0].block_number, 12); // Latest first
	assert_eq!(events1[1].block_number, 8);
	assert_eq!(events1[2].block_number, 4);

	// Test retrieval for subscription_id_2
	let response2 = process_msg_get_events::<IdealIndexer>(&trees, key2.clone());
	let ResponseMessage::Events { key: response_key2, events: events2 } = response2 else {
		panic!("Wrong response message for subscription_id_2.");
	};
	assert_eq!(key2, response_key2);
	assert_eq!(events2.len(), 2);
	assert_eq!(events2[0].block_number, 14);
	assert_eq!(events2[1].block_number, 6);
}

#[tokio::test]
async fn test_h256_to_u32_conversion() {
	// Test the H256 to u32 conversion logic used in IDN Manager events
	let h256_bytes = [
		0x12, 0x34, 0x56, 0x78, // First 4 bytes (little-endian u32)
		0x9A, 0xBC, 0xDE, 0xF0, // Next 4 bytes (ignored)
		0x11, 0x22, 0x33, 0x44, // More bytes (ignored)
		0x55, 0x66, 0x77, 0x88, // More bytes (ignored)
		0xAA, 0xBB, 0xCC, 0xDD, // More bytes (ignored)
		0xEE, 0xFF, 0x00, 0x11, // More bytes (ignored)
		0x22, 0x33, 0x44, 0x55, // More bytes (ignored)
		0x66, 0x77, 0x88, 0x99, // Last 4 bytes (ignored)
	];

	// Convert using the same logic as in our event handler
	let id_u32 = u32::from_le_bytes([h256_bytes[0], h256_bytes[1], h256_bytes[2], h256_bytes[3]]);

	// Expected value: 0x78563412 (little-endian interpretation of 0x12, 0x34, 0x56, 0x78)
	assert_eq!(id_u32, 0x78563412);

	// Test indexing with this converted ID
	let db_config = sled::Config::new().temporary(true);
	let trees = open_trees::<IdealIndexer>(db_config).unwrap();
	let indexer = Indexer::<IdealIndexer>::new_test(trees.clone());

	let key = Key::Substrate(SubstrateKey::SubscriptionId(id_u32));
	indexer.index_event(key.clone(), 100, 1).unwrap();

	let response = process_msg_get_events::<IdealIndexer>(&trees, key.clone());
	let ResponseMessage::Events { key: response_key, events } = response else {
		panic!("Wrong response message.");
	};
	assert_eq!(key, response_key);
	assert_eq!(events.len(), 1);
	assert_eq!(events[0].block_number, 100);
	assert_eq!(events[0].event_index, 1);
}

#[tokio::test]
async fn test_multiple_subscription_events_same_block() {
	// Test multiple subscription events in the same block with different event indices
	let db_config = sled::Config::new().temporary(true);
	let trees = open_trees::<IdealIndexer>(db_config).unwrap();
	let indexer = Indexer::<IdealIndexer>::new_test(trees.clone());

	let subscription_id = 999u32;
	let key = Key::Substrate(SubstrateKey::SubscriptionId(subscription_id));
	let block_number = 50u32;

	// Index multiple events in the same block
	indexer.index_event(key.clone(), block_number, 1).unwrap();
	indexer.index_event(key.clone(), block_number, 5).unwrap();
	indexer.index_event(key.clone(), block_number, 10).unwrap();

	let response = process_msg_get_events::<IdealIndexer>(&trees, key.clone());
	let ResponseMessage::Events { key: response_key, events } = response else {
		panic!("Wrong response message.");
	};
	assert_eq!(key, response_key);
	assert_eq!(events.len(), 3);

	// All events should be from the same block
	for event in &events {
		assert_eq!(event.block_number, block_number);
	}

	// Event indices should be as expected (ordered by event index, highest first)
	assert_eq!(events[0].event_index, 10);
	assert_eq!(events[1].event_index, 5);
	assert_eq!(events[2].event_index, 1);
}

#[tokio::test]
async fn test_subscription_events_across_blocks() {
	// Test subscription events across multiple blocks to verify proper ordering
	let db_config = sled::Config::new().temporary(true);
	let trees = open_trees::<IdealIndexer>(db_config).unwrap();
	let indexer = Indexer::<IdealIndexer>::new_test(trees.clone());

	let subscription_id = 777u32;
	let key = Key::Substrate(SubstrateKey::SubscriptionId(subscription_id));

	// Index events across different blocks (not in chronological order)
	indexer.index_event(key.clone(), 100, 1).unwrap(); // Block 100
	indexer.index_event(key.clone(), 50, 3).unwrap(); // Block 50 (earlier)
	indexer.index_event(key.clone(), 150, 2).unwrap(); // Block 150 (later)
	indexer.index_event(key.clone(), 75, 1).unwrap(); // Block 75 (middle)

	let response = process_msg_get_events::<IdealIndexer>(&trees, key.clone());
	let ResponseMessage::Events { key: response_key, events } = response else {
		panic!("Wrong response message.");
	};
	assert_eq!(key, response_key);
	assert_eq!(events.len(), 4);

	// Events should be ordered by block number descending (latest first)
	assert_eq!(events[0].block_number, 150);
	assert_eq!(events[1].block_number, 100);
	assert_eq!(events[2].block_number, 75);
	assert_eq!(events[3].block_number, 50);
}
