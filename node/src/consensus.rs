use crate::primitives::{AuctionTrigger, Bid, Signed};
use crate::state::StateManager;
use anyhow::Result;
use hotstuff_rs::app::App;
use serde::{Deserialize, Serialize};

// The transactions that our state machine can process.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum Transaction {
    PlaceBid(Signed<Bid>),
    TriggerAuction(Signed<AuctionTrigger>),
}

// Our application state machine.
#[derive(Debug)]
pub struct ConsensusApp {
    state_manager: StateManager,
}

impl ConsensusApp {
    pub fn new(state_manager: StateManager) -> Self {
        Self { state_manager }
    }
}

impl App for ConsensusApp {
    type Transaction = Transaction;

    fn deliver(&mut self, tx: Self::Transaction) {
        log::info!("Consensus engine delivering transaction to the app state.");

        let result: Result<()> = match tx {
            Transaction::PlaceBid(signed_bid) => {
                if signed_bid.verify().unwrap_or(false) {
                    // TODO: Convert PublicKey to Address for fee application.
                    // let address = ...;
                    // self.state_manager.apply_fees(&address, signed_bid.fee)?;
                    
                    log::info!("Applying bid to state: {:?}", signed_bid.data.id);
                    self.state_manager.place_bid(&signed_bid.data)
                } else {
                    log::warn!("Invalid signature for bid {:?}", signed_bid.data.id);
                    Ok(())
                }
            }
            Transaction::TriggerAuction(signed_auction) => {
                if signed_auction.verify().unwrap_or(false) {
                    // TODO: Convert PublicKey to Address for fee application.
                    // let address = ...;
                    // self.state_manager.apply_fees(&address, signed_auction.fee)?;

                    log::info!("Matching auction in state: {:?}", signed_auction.data.id);
                    let _ = self.state_manager.find_match(&signed_auction.data);
                    Ok(())
                } else {
                    log::warn!("Invalid signature for auction {:?}", signed_auction.data.id);
                    Ok(())
                }
            }
        };

        if let Err(e) = result {
            log::error!("Failed to apply transaction to state: {}", e);
        }
    }
}
