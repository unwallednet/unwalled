use crate::identity::Address;
use crate::primitives::{AuctionTrigger, Bid, Match};
use anyhow::Result;
use rocksdb::{ColumnFamilyDescriptor, Options, DB};
use std::path::Path;

const CF_BIDS: &str = "bids";
const CF_ACCOUNTS: &str = "accounts";

/// Manages the state of the blockchain, backed by RocksDB.
#[derive(Debug)]
pub struct StateManager {
    db: DB,
}

impl StateManager {
    pub fn new<P: AsRef<Path>>(path: P) -> Result<Self> {
        let mut db_opts = Options::default();
        db_opts.create_if_missing(true);
        db_opts.create_missing_column_families(true);

        let cf_bids = ColumnFamilyDescriptor::new(CF_BIDS, Options::default());
        let cf_accounts = ColumnFamilyDescriptor::new(CF_ACCOUNTS, Options::default());

        let db = DB::open_cf_descriptors(&db_opts, path, vec![cf_bids, cf_accounts])?;
        Ok(Self { db })
    }

    // --- Account Methods ---

    pub fn get_balance(&self, address: &Address) -> Result<u64> {
        let cf = self.db.cf_handle(CF_ACCOUNTS).unwrap();
        let balance_bytes = self.db.get_cf(cf, address.as_bytes())?.unwrap_or_default();
        let balance = balance_bytes.try_into().map(u64::from_le_bytes).unwrap_or(0);
        Ok(balance)
    }

    pub fn set_balance(&self, address: &Address, amount: u64) -> Result<()> {
        let cf = self.db.cf_handle(CF_ACCOUNTS).unwrap();
        self.db.put_cf(cf, address.as_bytes(), &amount.to_le_bytes())?;
        Ok(())
    }
    
    pub fn apply_fees(&self, signer_address: &Address, fee: u64) -> Result<()> {
        let mut balance = self.get_balance(signer_address)?;
        if balance < fee {
            return Err(anyhow::anyhow!("Insufficient funds for fee"));
        }
        balance -= fee;
        self.set_balance(signer_address, balance)?;
        // TODO: Add fee to a validator reward pool
        Ok(())
    }

    // --- Bid/Auction Methods ---

    pub fn place_bid(&self, bid: &Bid) -> Result<()> {
        let cf = self.db.cf_handle(CF_BIDS).unwrap();
        let key = bid.id.as_bytes();
        let value = serde_json::to_vec(bid)?;
        self.db.put_cf(cf, key, value)?;
        Ok(())
    }

    pub fn find_match(&self, _auction: &AuctionTrigger) -> Result<Option<Match>> {
        // Logic for finding a match would go here, iterating over the `bids` CF.
        // For this scaffolding step, the conceptual placement is what matters.
        Ok(None) 
    }
}