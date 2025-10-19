use serde::{Deserialize, Serialize};
use anyhow::Result;

// A wrapper for a public key.
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct PublicKey(pub Vec<u8>);

// A wrapper for a cryptographic signature.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct Signature(pub Vec<u8>);

// A unique identifier for a wallet or account.
pub type Address = String;

/// Represents an entity that can sign data.
/// This trait can be implemented for different kinds of wallets
/// (e.g., local keypairs, hardware wallets, EVM wallets via EIP-712).
pub trait Signer {
    fn sign(&self, data: &[u8]) -> Result<Signature>;
    fn public_key(&self) -> PublicKey;
    fn address(&self) -> Address;
}

/// Represents a simple, local wallet for testing.
#[derive(Debug)]
pub struct LocalWallet {
    keypair: ed25519_dalek::Keypair,
}

impl LocalWallet {
    /// Creates a new random wallet.
    pub fn new() -> Self {
        let mut csprng = rand::rngs::OsRng{};
        let keypair: ed25519_dalek::Keypair = ed25519_dalek::Keypair::generate(&mut csprng);
        Self { keypair }
    }
}

impl Signer for LocalWallet {
    fn sign(&self, data: &[u8]) -> Result<Signature> {
        use ed25519_dalek::Signer;
        let signature = self.keypair.sign(data);
        Ok(Signature(signature.to_bytes().to_vec()))
    }

    fn public_key(&self) -> PublicKey {
        PublicKey(self.keypair.public.to_bytes().to_vec())
    }

    fn address(&self) -> Address {
        // For simplicity, we'll use the hex-encoded public key as the address.
        hex::encode(self.public_key().0)
    }
}
