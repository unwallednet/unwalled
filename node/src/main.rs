mod primitives;
mod config;
mod state;
mod network;
mod rpc;
mod consensus;
mod settlement; // <-- new module

use anyhow::Result;
use tokio::sync::mpsc;
use crate::consensus::{ConsensusApp, Transaction};
use crate::state::StateManager;
use crate::settlement::SettlementManager; // <-- new import

#[tokio::main]
async fn main() -> Result<()> {
    env_logger::init();
    log::info!("Starting Unwalled Node");

    let config = config::load_config()?;
    log::info!("Configuration loaded: {:?}", config);

    // --- Component Initialization ---
    let state_manager = StateManager::new(&config.db_path)?;
    let settlement_manager = SettlementManager::new();
    let app = ConsensusApp::new(state_manager);
    let consensus_engine = consensus::Consensus::new();

    let (tx_to_consensus, mut rx_from_components) = mpsc::channel::<Transaction>(100);

    let mut network_manager = network::NetworkManager::new(tx_to_consensus.clone()).await?;
    let rpc_server = rpc::run_server(config.rpc_listen_address, tx_to_consensus);
    
    log::info!("All components initialized. Starting main event loop...");

    // --- Main Event Loop ---
    loop {
        tokio::select! {
            _ = rpc_server => {
                log::error!("RPC server task unexpectedly finished.");
                break;
            },
            event = network::event_loop(network_manager) => {
                log::error!("Network event loop finished: {:?}", event);
                break;
            },
            Some(transaction) = rx_from_components.recv() => {
                log::info!("Received transaction for consensus: {:?}", transaction);
                // The consensus engine would process this transaction.
            },
        }
    }

    Ok(())
}