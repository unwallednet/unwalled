use anyhow::Result;
use log;

// Re-exporting primitives for convenience from the node crate.
pub use unwalled_node::primitives::{AuctionTrigger, Bid, Match};

/// A client for interacting with an Unwalled node.
pub struct Client {
    rpc_endpoint: String,
    http_client: reqwest::Client,
}

impl Client {
    pub fn new(rpc_endpoint: String) -> Self {
        Self {
            rpc_endpoint,
            http_client: reqwest::Client::new(),
        }
    }

    /// Placeholder for placing a bid via RPC.
    pub async fn place_bid(&self, bid: &Bid) -> Result<()> {
        log::info!("Sending place_bid request for bid ID: {}", bid.id);
        // In a real implementation, this would make an HTTP/3 RPC call.
        // Using a standard POST request as a placeholder.
        let response = self.http_client
            .post(&format!("{}/rpc/place_bid", self.rpc_endpoint))
            .json(bid)
            .send()
            .await?;

        if response.status().is_success() {
            log::info!("Successfully placed bid.");
            Ok(())
        } else {
            Err(anyhow::anyhow!("Failed to place bid: {}", response.status()))
        }
    }

    /// Placeholder for triggering an auction via RPC.
    pub async fn trigger_auction(&self, auction: &AuctionTrigger) -> Result<Option<Match>> {
        log::info!("Sending trigger_auction request for auction ID: {}", auction.id);
        let response = self.http_client
            .post(&format!("{}/rpc/trigger_auction", self.rpc_endpoint))
            .json(auction)
            .send()
            .await?;

        if response.status().is_success() {
            let auction_match = response.json::<Option<Match>>().await?;
            Ok(auction_match)
        } else {
            Err(anyhow::anyhow!("Failed to trigger auction: {}", response.status()))
        }
    }
}
