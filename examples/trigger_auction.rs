use anyhow::Result;
use unwalled_client::{Client, AuctionTrigger};
use uuid::Uuid;

#[tokio::main]
async fn main() -> Result<()> {
    env_logger::init();

    let client = Client::new("http://127.0.0.1:8080".to_string());

    let auction = AuctionTrigger {
        id: Uuid::new_v4(),
        publisher_id: "my-publisher".to_string(),
        bid_floor: 100, // 1 cent floor
        attributes: vec!["sports".to_string(), "news".to_string(), "premium".to_string()],
    };

    log::info!("Triggering an auction: {:?}", auction);

    match client.trigger_auction(&auction).await {
        Ok(Some(match_result)) => {
            log::info!("Auction won! Match details: {:?}", match_result)
        }
        Ok(None) => {
            log::info!("Auction completed with no matching bids.")
        }
        Err(e) => {
            log::error!("Failed to trigger auction: {}", e)
        }
    }

    Ok(())
}
