pub mod torrent;
pub mod requests;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let data = torrent::read_torrent_file("test.torrent")?;
    println!("Announce: {}", data.announce);

    let url = &data.announce;
    let data = b"Hello, World!";
    
    requests::tracker::request(url, data)?;

    Ok(())
}