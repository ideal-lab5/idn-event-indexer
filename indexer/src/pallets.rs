#[macro_export]
macro_rules! index_claims_event {
	($event_enum: ty, $event: ident, $indexer: ident, $block_number: ident, $event_index: ident) => {
		match $event {
			<$event_enum>::Claimed { who, .. } => {
				$indexer.index_event(
					Key::Substrate(SubstrateKey::AccountId(Bytes32(who.0))),
					$block_number,
					$event_index,
				)?;
				1
			},
		}
	};
}

#[macro_export]
macro_rules! index_paras_event {
	($event_enum: ty, $event: ident, $indexer: ident, $block_number: ident, $event_index: ident) => {
		match $event {
			<$event_enum>::CurrentCodeUpdated(para_id) => {
				$indexer.index_event(
					Key::Chain(ChainKey::ParaId(para_id.0)),
					$block_number,
					$event_index,
				)?;
				1
			},
			<$event_enum>::CurrentHeadUpdated(para_id) => {
				$indexer.index_event(
					Key::Chain(ChainKey::ParaId(para_id.0)),
					$block_number,
					$event_index,
				)?;
				1
			},
			<$event_enum>::CodeUpgradeScheduled(para_id) => {
				$indexer.index_event(
					Key::Chain(ChainKey::ParaId(para_id.0)),
					$block_number,
					$event_index,
				)?;
				1
			},
			<$event_enum>::NewHeadNoted(para_id) => {
				$indexer.index_event(
					Key::Chain(ChainKey::ParaId(para_id.0)),
					$block_number,
					$event_index,
				)?;
				1
			},
			<$event_enum>::ActionQueued(para_id, session_index) => {
				$indexer.index_event(
					Key::Chain(ChainKey::ParaId(para_id.0)),
					$block_number,
					$event_index,
				)?;
				$indexer.index_event(
					Key::Substrate(SubstrateKey::SessionIndex(session_index)),
					$block_number,
					$event_index,
				)?;
				2
			},
			<$event_enum>::PvfCheckStarted(_, para_id) => {
				$indexer.index_event(
					Key::Chain(ChainKey::ParaId(para_id.0)),
					$block_number,
					$event_index,
				)?;
				1
			},
			<$event_enum>::PvfCheckAccepted(_, para_id) => {
				$indexer.index_event(
					Key::Chain(ChainKey::ParaId(para_id.0)),
					$block_number,
					$event_index,
				)?;
				1
			},
			<$event_enum>::PvfCheckRejected(_, para_id) => {
				$indexer.index_event(
					Key::Chain(ChainKey::ParaId(para_id.0)),
					$block_number,
					$event_index,
				)?;
				1
			},
		}
	};
}

#[macro_export]
macro_rules! index_hrmp_event {
	($event_enum: ty, $event: ident, $indexer: ident, $block_number: ident, $event_index: ident) => {
		match $event {
			<$event_enum>::OpenChannelRequested { sender, recipient, .. } => {
				$indexer.index_event(
					Key::Chain(ChainKey::ParaId(sender.0)),
					$block_number,
					$event_index,
				)?;
				$indexer.index_event(
					Key::Chain(ChainKey::ParaId(recipient.0)),
					$block_number,
					$event_index,
				)?;
				2
			},
			<$event_enum>::OpenChannelCanceled { by_parachain, .. } => {
				$indexer.index_event(
					Key::Chain(ChainKey::ParaId(by_parachain.0)),
					$block_number,
					$event_index,
				)?;
				1
			},
			<$event_enum>::OpenChannelAccepted { sender, recipient } => {
				$indexer.index_event(
					Key::Chain(ChainKey::ParaId(sender.0)),
					$block_number,
					$event_index,
				)?;
				$indexer.index_event(
					Key::Chain(ChainKey::ParaId(recipient.0)),
					$block_number,
					$event_index,
				)?;
				2
			},
			<$event_enum>::ChannelClosed { by_parachain, .. } => {
				$indexer.index_event(
					Key::Chain(ChainKey::ParaId(by_parachain.0)),
					$block_number,
					$event_index,
				)?;
				1
			},
			<$event_enum>::HrmpChannelForceOpened { sender, recipient, .. } => {
				$indexer.index_event(
					Key::Chain(ChainKey::ParaId(sender.0)),
					$block_number,
					$event_index,
				)?;
				$indexer.index_event(
					Key::Chain(ChainKey::ParaId(recipient.0)),
					$block_number,
					$event_index,
				)?;
				2
			},
			<$event_enum>::HrmpSystemChannelOpened { .. } => 0,
			<$event_enum>::OpenChannelDepositsUpdated { .. } => 0,
		}
	};
}

#[macro_export]
macro_rules! index_disputes_event {
	($event_enum: ty, $event: ident, $indexer: ident, $block_number: ident, $event_index: ident) => {
		match $event {
			<$event_enum>::DisputeInitiated(candidate_hash, ..) => {
				$indexer.index_event(
					Key::Chain(ChainKey::CandidateHash(Bytes32(candidate_hash.0.into()))),
					$block_number,
					$event_index,
				)?;
				1
			},
			<$event_enum>::DisputeConcluded(candidate_hash, ..) => {
				$indexer.index_event(
					Key::Chain(ChainKey::CandidateHash(Bytes32(candidate_hash.0.into()))),
					$block_number,
					$event_index,
				)?;
				1
			},
			_ => 0,
		}
	};
}

#[macro_export]
macro_rules! index_paras_registrar_event {
	($event_enum: ty, $event: ident, $indexer: ident, $block_number: ident, $event_index: ident) => {
		match $event {
			<$event_enum>::Registered { para_id, manager } => {
				$indexer.index_event(
					Key::Chain(ChainKey::ParaId(para_id.0)),
					$block_number,
					$event_index,
				)?;
				$indexer.index_event(
					Key::Substrate(SubstrateKey::AccountId(Bytes32(manager.0))),
					$block_number,
					$event_index,
				)?;
				2
			},
			<$event_enum>::Deregistered { para_id } => {
				$indexer.index_event(
					Key::Chain(ChainKey::ParaId(para_id.0)),
					$block_number,
					$event_index,
				)?;
				1
			},
			<$event_enum>::Reserved { para_id, who } => {
				$indexer.index_event(
					Key::Chain(ChainKey::ParaId(para_id.0)),
					$block_number,
					$event_index,
				)?;
				$indexer.index_event(
					Key::Substrate(SubstrateKey::AccountId(Bytes32(who.0))),
					$block_number,
					$event_index,
				)?;
				2
			},
			<$event_enum>::Swapped { para_id, other_id } => {
				$indexer.index_event(
					Key::Chain(ChainKey::ParaId(para_id.0)),
					$block_number,
					$event_index,
				)?;
				$indexer.index_event(
					Key::Chain(ChainKey::ParaId(other_id.0)),
					$block_number,
					$event_index,
				)?;
				2
			},
		}
	};
}

#[macro_export]
macro_rules! index_slots_event {
	($event_enum: ty, $event: ident, $indexer: ident, $block_number: ident, $event_index: ident) => {
		match $event {
			<$event_enum>::Leased { para_id, leaser, .. } => {
				$indexer.index_event(
					Key::Chain(ChainKey::ParaId(para_id.0)),
					$block_number,
					$event_index,
				)?;
				$indexer.index_event(
					Key::Substrate(SubstrateKey::AccountId(Bytes32(leaser.0))),
					$block_number,
					$event_index,
				)?;
				2
			},
			_ => 0,
		}
	};
}

#[macro_export]
macro_rules! index_auctions_event {
	($event_enum: ty, $event: ident, $indexer: ident, $block_number: ident, $event_index: ident) => {
		match $event {
			<$event_enum>::AuctionStarted { auction_index, .. } => {
				$indexer.index_event(
					Key::Chain(ChainKey::AuctionIndex(auction_index)),
					$block_number,
					$event_index,
				)?;
				1
			},
			<$event_enum>::AuctionClosed { auction_index } => {
				$indexer.index_event(
					Key::Chain(ChainKey::AuctionIndex(auction_index)),
					$block_number,
					$event_index,
				)?;
				1
			},
			<$event_enum>::Reserved { bidder, .. } => {
				$indexer.index_event(
					Key::Substrate(SubstrateKey::AccountId(Bytes32(bidder.0))),
					$block_number,
					$event_index,
				)?;
				1
			},
			<$event_enum>::Unreserved { bidder, .. } => {
				$indexer.index_event(
					Key::Substrate(SubstrateKey::AccountId(Bytes32(bidder.0))),
					$block_number,
					$event_index,
				)?;
				1
			},
			<$event_enum>::ReserveConfiscated { para_id, leaser, .. } => {
				$indexer.index_event(
					Key::Chain(ChainKey::ParaId(para_id.0)),
					$block_number,
					$event_index,
				)?;
				$indexer.index_event(
					Key::Substrate(SubstrateKey::AccountId(Bytes32(leaser.0))),
					$block_number,
					$event_index,
				)?;
				2
			},
			<$event_enum>::BidAccepted { bidder, para_id, .. } => {
				$indexer.index_event(
					Key::Substrate(SubstrateKey::AccountId(Bytes32(bidder.0))),
					$block_number,
					$event_index,
				)?;
				$indexer.index_event(
					Key::Chain(ChainKey::ParaId(para_id.0)),
					$block_number,
					$event_index,
				)?;
				2
			},
			<$event_enum>::WinningOffset { auction_index, .. } => {
				$indexer.index_event(
					Key::Chain(ChainKey::AuctionIndex(auction_index)),
					$block_number,
					$event_index,
				)?;
				1
			},
		}
	};
}

#[macro_export]
macro_rules! index_crowdloan_event {
	($event_enum: ty, $event: ident, $indexer: ident, $block_number: ident, $event_index: ident) => {
		match $event {
			<$event_enum>::Created { para_id } => {
				$indexer.index_event(
					Key::Chain(ChainKey::ParaId(para_id.0)),
					$block_number,
					$event_index,
				)?;
				1
			},
			<$event_enum>::Contributed { who, fund_index, .. } => {
				$indexer.index_event(
					Key::Substrate(SubstrateKey::AccountId(Bytes32(who.0))),
					$block_number,
					$event_index,
				)?;
				$indexer.index_event(
					Key::Chain(ChainKey::ParaId(fund_index.0)),
					$block_number,
					$event_index,
				)?;
				2
			},
			<$event_enum>::Withdrew { who, fund_index, .. } => {
				$indexer.index_event(
					Key::Substrate(SubstrateKey::AccountId(Bytes32(who.0))),
					$block_number,
					$event_index,
				)?;
				$indexer.index_event(
					Key::Chain(ChainKey::ParaId(fund_index.0)),
					$block_number,
					$event_index,
				)?;
				2
			},
			<$event_enum>::PartiallyRefunded { para_id } => {
				$indexer.index_event(
					Key::Chain(ChainKey::ParaId(para_id.0)),
					$block_number,
					$event_index,
				)?;
				1
			},
			<$event_enum>::AllRefunded { para_id } => {
				$indexer.index_event(
					Key::Chain(ChainKey::ParaId(para_id.0)),
					$block_number,
					$event_index,
				)?;
				1
			},
			<$event_enum>::Dissolved { para_id } => {
				$indexer.index_event(
					Key::Chain(ChainKey::ParaId(para_id.0)),
					$block_number,
					$event_index,
				)?;
				1
			},
			<$event_enum>::HandleBidResult { para_id, .. } => {
				$indexer.index_event(
					Key::Chain(ChainKey::ParaId(para_id.0)),
					$block_number,
					$event_index,
				)?;
				1
			},
			<$event_enum>::Edited { para_id } => {
				$indexer.index_event(
					Key::Chain(ChainKey::ParaId(para_id.0)),
					$block_number,
					$event_index,
				)?;
				1
			},
			<$event_enum>::MemoUpdated { who, para_id, .. } => {
				$indexer.index_event(
					Key::Substrate(SubstrateKey::AccountId(Bytes32(who.0))),
					$block_number,
					$event_index,
				)?;
				$indexer.index_event(
					Key::Chain(ChainKey::ParaId(para_id.0)),
					$block_number,
					$event_index,
				)?;
				2
			},
			<$event_enum>::AddedToNewRaise { para_id } => {
				$indexer.index_event(
					Key::Chain(ChainKey::ParaId(para_id.0)),
					$block_number,
					$event_index,
				)?;
				1
			},
		}
	};
}
