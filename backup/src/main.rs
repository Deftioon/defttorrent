use defttorrent::{requests, backend, gtk};

#[tokio::main]
async fn torrent_main() -> Result<(), Box<dyn std::error::Error>> {
    let data = backend::read_torrent_file("test2.torrent")?;
    let info_hash_str = String::from_utf8_lossy(&data.info_hash);
    println!("Torrent file: {:?}", data);
    println!("{}", info_hash_str);

    let announce_url = "http://tracker.opentrackr.org:1337/announce";
    println!("Announcing to tracker: {}", announce_url);
    println!("Tracker in torrent: {:?}", data.announce);
    let peers = requests::announce(&data.info_hash, &data.announce).await?;
    // println!("{:?}", peers);
    Ok(())
}

fn gtk_main() {
    gtk::main::main();
}

fn main() {
    gtk_main();
}