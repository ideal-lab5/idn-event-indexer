use acuity_index_substrate::{shared::*, substrate::*, websockets::*, *};

use crate::{ChainKey, IdealIndexer};

#[tokio::test]
<<<<<<< HEAD
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
=======
async fn test_process_msg_auction_index() {
	let db_config = sled::Config::new().temporary(true);
	let trees = open_trees::<PolkadotIndexer>(db_config).unwrap();
	let indexer = Indexer::<PolkadotIndexer>::new_test(trees.clone());
	let auction_index = 88;
	let key = Key::Chain(ChainKey::AuctionIndex(auction_index));
	indexer.index_event(key.clone(), 4, 5).unwrap();
	indexer.index_event(key.clone(), 8, 5).unwrap();
	indexer.index_event(key.clone(), 10, 5).unwrap();

	let response = process_msg_get_events::<PolkadotIndexer>(&trees, key.clone());
>>>>>>> origin/main

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
<<<<<<< HEAD
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
=======
async fn test_process_msg_candidate_hash() {
	let db_config = sled::Config::new().temporary(true);
	let trees = open_trees::<PolkadotIndexer>(db_config).unwrap();
	let indexer = Indexer::<PolkadotIndexer>::new_test(trees.clone());
	let candidate_hash = Bytes32([8; 32]);
	let key = Key::Chain(ChainKey::CandidateHash(candidate_hash));
	indexer.index_event(key.clone(), 4, 5).unwrap();
	indexer.index_event(key.clone(), 8, 5).unwrap();
	indexer.index_event(key.clone(), 10, 5).unwrap();

	let response = process_msg_get_events::<PolkadotIndexer>(&trees, key.clone());

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
async fn test_process_msg_para_id() {
	let db_config = sled::Config::new().temporary(true);
	let trees = open_trees::<PolkadotIndexer>(db_config).unwrap();
	let indexer = Indexer::<PolkadotIndexer>::new_test(trees.clone());
	let para_id = 88;
	let key = Key::Chain(ChainKey::ParaId(para_id));
	indexer.index_event(key.clone(), 4, 5).unwrap();
	indexer.index_event(key.clone(), 8, 5).unwrap();
	indexer.index_event(key.clone(), 10, 5).unwrap();

	let response = process_msg_get_events::<PolkadotIndexer>(&trees, key.clone());
>>>>>>> origin/main

	let ResponseMessage::Events { key: response_key, events } = response else {
		panic!("Wrong response message.");
	};
	assert_eq!(key, response_key);
	assert_eq!(events.len(), 3);
	assert_eq!(events[0].block_number, 10);
	assert_eq!(events[1].block_number, 8);
	assert_eq!(events[2].block_number, 4);
}
