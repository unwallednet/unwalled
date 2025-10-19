use anyhow::Result;
use unwalled_client::{Client, Bid};
use uuid::Uuid;

#[tokio::main]
async fn main() -> Result<()> {
    env_logger::init();

    let client = Client::new("http://127.0.0.1:8080".to_string());

    let bid = Bid {
        id: Uuid::new_v4(),
        advertiser_id: "my-advertiser".to_string(),
        price: 150, // e.g., 1.5 cents in smallest units
        targeting: vec!["sports".to_string(), "news".to_string()],
        adm: "<VAST version='4.2'>...</VAST>".to_string(),
    };

    log::info!("Placing a sample bid: {:?}", bid);

    match client.place_bid(&bid).await {
        Ok(_) => log::info!("Bid placed successfully!"),
        Err(e) => log::error!("Failed to place bid: {}", e),
    }

    Ok(())
}
