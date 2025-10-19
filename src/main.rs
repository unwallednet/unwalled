mod primitives;
mod config;
mod state;
mod network;
mod rpc;
mod consensus;

use anyhow::Result;

#[tokio::main]
async fn main() -> Result<()> {
    // Initialize logging
    env_logger::init();
    log::info!("Starting Unwalled Node");

    // Load configuration
    let config = config::load_config()?;
    log::info!("Configuration loaded: {:?}", config);

    // Initialize the state manager (and RocksDB)
    let state_manager = state::StateManager::new(&config.db_path)?;
    log::info!("State manager initialized at {}", &config.db_path);

    // Initialize consensus engine
    let consensus_engine = consensus::Consensus::new();
    log::info!("Consensus engine initialized");

    // Initialize the network layer
    let mut network_manager = network::NetworkManager::new().await?;
    log::info!("Network manager initialized with Peer ID: {}", network_manager.peer_id);

    // Start the RPC server
    let rpc_server = rpc::run_server(config.rpc_listen_address);
    log::info!("RPC server starting on {}", config.rpc_listen_address);

    // Main event loop
    tokio::select! {
        _ = rpc_server => {
            log::info!("RPC server task finished.");
        },
        event = network::event_loop(network_manager) => {
            log::info!("Network event loop finished: {:?}", event);
        }
    }

    Ok(())
}
