pub mod parser;

pub fn read_torrent_file(file_path: &str) -> Result<parser::Torrent, Box<dyn std::error::Error>> {
    let data = std::fs::read(file_path)?;
    let mut parser = parser::BencodeParser::new(&data);
    let bencode = parser.parse()?;
    let torrent = parser::Torrent::from_bencode(&bencode)?;
    Ok(torrent)
}