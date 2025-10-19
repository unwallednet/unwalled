use serde::{Deserialize, Serialize};
use uuid::Uuid;

/// Represents a bid from an advertiser.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Bid {
    pub id: Uuid,
    pub advertiser_id: String,
    pub price: u64, // Using u64 for price in smallest currency unit
    pub targeting: Vec<String>, // Simplified targeting using a vector of strings
    pub adm: String, // Ad markup (e.g., VAST, HTML)
}

/// Represents a request from a publisher for an ad.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AuctionTrigger {
    pub id: Uuid,
    pub publisher_id: String,
    pub bid_floor: u64, // Minimum acceptable price
    pub attributes: Vec<String>, // Attributes of the impression opportunity
}

/// Represents a successful match between a Bid and an AuctionTrigger.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Match {
    pub bid_id: Uuid,
    pub auction_id: Uuid,
    pub winning_price: u64,
}
