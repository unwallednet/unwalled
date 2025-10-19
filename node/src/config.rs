use serde::Deserialize;
use std::net::SocketAddr;

#[derive(Debug, Deserialize)]
pub struct Config {
    pub rpc_listen_address: SocketAddr,
    pub db_path: String,
    // Add other configuration fields as needed
}

impl Default for Config {
    fn default() -> Self {
        Self {
            rpc_listen_address: "127.0.0.1:8080".parse().unwrap(),
            db_path: "/tmp/unwalled-node-db".to_string(),
        }
    }
}

/// Loads the node configuration.
/// For now, it just returns the default configuration.
pub fn load_config() -> anyhow::Result<Config> {
    Ok(Config::default())
}
