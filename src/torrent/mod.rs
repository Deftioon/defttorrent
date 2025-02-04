pub mod parser;

pub fn read_torrent_file(file_path: &str) -> Result<parser::Torrent, Box<dyn std::error::Error>> {
    println!("hi");
    let data = std::fs::read(file_path)?;
    println!("hi");
    let mut parser = parser::BencodeParser::new(&data);
    println!("hi");
    let bencode = parser.parse()?;
    println!("hi"); 
    let torrent = parser::Torrent::from_bencode(&bencode, &data)?; // Pass data here
    println!("hi");
    Ok(torrent)
}