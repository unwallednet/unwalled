use crate::identity::{PublicKey, Signature};
use anyhow::Result;
use ed25519_dalek::Verifier;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

/// A generic wrapper for a signed transaction, now including a fee.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Signed<T> {
    pub data: T,
    pub signer: PublicKey,
    pub signature: Signature,
    pub nonce: u64,
    /// The fee offered to the validator for processing this transaction,
    /// priced in the smallest unit of KUSD.
    pub fee: u64,
}

impl<T: Serialize> Signed<T> {
    /// Verifies the signature of the wrapped data.
    pub fn verify(&self) -> Result<bool> {
        // The fee and nonce must be part of the signed payload to prevent tampering.
        let mut bytes_to_verify = serde_json::to_vec(&self.data)?;
        bytes_to_verify.extend_from_slice(&self.nonce.to_le_bytes());
        bytes_to_verify.extend_from_slice(&self.fee.to_le_bytes());

        let public_key = ed25519_dalek::PublicKey::from_bytes(&self.signer.0)?;
        let signature = ed25519_dalek::Signature::from_bytes(&self.signature.0)?;

        Ok(public_key.verify(&bytes_to_verify, &signature).is_ok())
    }
}

/// Represents a bid from an advertiser.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Bid {
    pub id: Uuid,
    pub price: u64,
    pub targeting: Vec<String>,
    pub adm: String,
}

/// Represents a request from a publisher for an ad.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AuctionTrigger {
    pub id: Uuid,
    pub bid_floor: u64,
    pub attributes: Vec<String>,
}

/// Represents a successful match between a Bid and an AuctionTrigger.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Match {
    pub bid_id: Uuid,
    pub auction_id: Uuid,
    pub winning_price: u64,
    pub advertiser_addr: String,
    pub publisher_addr: String,
}

// Type aliases for signed transactions
pub type SignedBid = Signed<Bid>;
pub type SignedAuctionTrigger = Signed<AuctionTrigger>;
