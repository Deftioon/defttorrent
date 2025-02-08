use std::net::{SocketAddr, ToSocketAddrs};
pub mod tracker;

pub async fn announce(
    info_hash: &[u8; 20],
    announce_url: &str,
) -> Result<Vec<SocketAddr>, Box<dyn std::error::Error>> {
    let peers = tracker::request(announce_url, info_hash).await?;
    Ok(peers)
}
