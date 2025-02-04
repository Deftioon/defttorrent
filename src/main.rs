pub mod torrent;
pub mod requests;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let data = torrent::read_torrent_file("test2.torrent")?;
    println!("{:?}", data);
    let info_hash_str = String::from_utf8_lossy(&data.info_hash);
    println!("{}", info_hash_str);

    let announce_url = "http://tracker.opentrackr.org:1337/announce";
    println!("Announcing to tracker: {}", announce_url);
    println!("Tracker in torrent: {:?}", data.announce);
    let peers = requests::announce(&data.info_hash, announce_url).await?;
    println!("{:?}", peers);
    Ok(())
}