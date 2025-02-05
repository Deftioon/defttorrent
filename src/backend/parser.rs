use core::fmt;
use sha1::{Sha1, Digest};

#[derive(Debug, PartialEq)]
pub enum BencodeValue {
    String(Vec<u8>),
    Integer(i64),
    List(Vec<BencodeValue>),
    Dict(Vec<(Vec<u8>, BencodeValue)>),
}

pub struct BencodeParser<'a> {
    data: &'a [u8],
    pos: usize,
}

impl<'a> BencodeParser<'a> {
    pub fn new(data: &'a [u8]) -> Self {
        BencodeParser { data, pos: 0 }
    }

    pub fn parse(&mut self) -> Result<BencodeValue, &'static str> {
        match self.peek_byte()? {
            b'i' => self.parse_integer(),
            b'l' => self.parse_list(),
            b'd' => self.parse_dict(),
            b'0'..=b'9' => self.parse_string(),
            _ => Err("Invalid Bencode format"),
        }
    }

    // --- Helper methods ---
    fn peek_byte(&self) -> Result<u8, &'static str> {
        self.data.get(self.pos).copied().ok_or("Unexpected EOF")
    }

    fn consume_byte(&mut self) -> Result<u8, &'static str> {
        let byte = self.peek_byte()?;
        self.pos += 1;
        Ok(byte)
    }

    // --- Parse individual types ---
    fn parse_integer(&mut self) -> Result<BencodeValue, &'static str> {
        assert_eq!(self.consume_byte()?, b'i');
        let start = self.pos;
        while self.peek_byte()? != b'e' {
            self.pos += 1;
        }
        let s = std::str::from_utf8(&self.data[start..self.pos]).map_err(|_| "Invalid integer")?;
        self.consume_byte()?; // Consume 'e'
        let num = s.parse().map_err(|_| "Invalid integer")?;
        Ok(BencodeValue::Integer(num))
    }

    fn parse_string(&mut self) -> Result<BencodeValue, &'static str> {
        let start = self.pos;
        while self.peek_byte()? != b':' {
            self.pos += 1;
        }
        let len_str = std::str::from_utf8(&self.data[start..self.pos])
            .map_err(|_| "Invalid string length")?;
        let len = len_str.parse::<usize>().map_err(|_| "Invalid string length")?;
        self.consume_byte()?; // Consume ':'
        let end = self.pos + len;
        if end > self.data.len() {
            return Err("String exceeds data length");
        }
        let bytes = self.data[self.pos..end].to_vec();
        self.pos = end;
        Ok(BencodeValue::String(bytes))
    }

    fn parse_list(&mut self) -> Result<BencodeValue, &'static str> {
        self.consume_byte()?; // Consume 'l'
        let mut list = Vec::new();
        while self.peek_byte()? != b'e' {
            let value = self.parse()?;
            list.push(value);
        }
        self.consume_byte()?; // Consume 'e'
        Ok(BencodeValue::List(list))
    }

    fn parse_dict(&mut self) -> Result<BencodeValue, &'static str> {
        self.consume_byte()?; // Consume 'd'
        let mut dict = Vec::new();
        while self.peek_byte()? != b'e' {
            let key = if let BencodeValue::String(k) = self.parse()? {
                k
            } else {
                return Err("Dictionary key must be a string");
            };
            let value = self.parse()?;
            dict.push((key, value));
        }
        self.consume_byte()?; // Consume 'e'
        Ok(BencodeValue::Dict(dict))
    }

    //FIXME: THIS NEVER ENDS
    pub fn find_info_position(&mut self, start_pos: usize) -> Result<usize, &'static str> {
        self.pos = start_pos;
        while self.pos < self.data.len() {
            if let Ok(BencodeValue::String(key)) = self.parse_string() {
                if key == b"info" {
                    let value_start = self.pos;
                    self.parse()?; // Parse the value to advance past it
                    return Ok(value_start);
                } else {
                    // Parse the corresponding value to move past the key-value pair
                    self.parse().ok(); // Ignore parsing errors, just advance
                }
            } else {
                // Skip invalid key and its value
                self.parse().ok();
            }
        }
        Err("Info key not found")
    }

    pub fn find_info_end(&mut self, start_pos: usize) -> Result<usize, &'static str> {
        self.pos = start_pos;
        self.parse()?; // Parse the info dict to advance pos to its end
        Ok(self.pos)
    }
}

#[derive(Debug)]
pub struct Torrent {
    pub announce: String,
    pub info: TorrentInfo,
    pub info_hash: [u8; 20],
    pub comment: Option<String>,
}

#[derive(Debug)]
pub struct TorrentInfo {
    pub name: String,
    pub piece_length: i64,
    pub pieces: Vec<[u8; 20]>,
    pub length: Option<i64>,        // For single-file torrents
    pub files: Option<Vec<TorrentFile>>, // For multi-file torrents
}

#[derive(Debug)]
pub struct TorrentFile {
    pub length: i64,
    pub path: Vec<String>,
}

impl Torrent {
    pub fn from_bencode(bencode: &BencodeValue, data: &[u8]) -> Result<Self, &'static str> {
        let dict = match bencode {
            BencodeValue::Dict(d) => d,
            _ => return Err("Root must be a dictionary"),
        };

        let mut announce = None;
        let mut info = None;
        let mut comment = None;
        let mut info_hash = [0u8; 20];

        for (key, value) in dict {
            match key.as_slice() {
                b"announce" => announce = Some(Torrent::parse_string(value)?),
                b"info" => {
                    // Calculate info_hash by finding the byte span of the info dict
                    println!("Value: {:?}", value);
                    let (start, end) = {
                        let mut parser = BencodeParser::new(data);
                        parser.parse().ok(); // Parse root dict
                        // Assuming the root is a dict, find the 'info' key's value position
                        let mut pos = 1; // Skip 'd' at position 0
                        let info_start = parser.find_info_position(pos).unwrap();
                        let info_end = parser.find_info_end(info_start).unwrap();
                        (info_start, info_end)
                    };
                    let info_bytes = &data[start..end];
                    let mut hasher = Sha1::new();
                    let info_hasher = hasher.clone();
                    let mut info_hash = [0u8; 20];
                    hasher.update(info_bytes);
                    info_hash.copy_from_slice(&hasher.finalize());
                    println!("Info hash: {:02x?}", info_hash);

                    info = Some(TorrentInfo::from_bencode(value)?);
                },
                b"comment" => comment = Some(Torrent::parse_string(value)?),
                _ => {}
            }
        }

        Ok(Torrent {
            announce: announce.ok_or("Missing announce")?,
            info: info.ok_or("Missing info")?,
            info_hash,
            comment,
        })
    }

    fn parse_string(value: &BencodeValue) -> Result<String, &'static str> {
        if let BencodeValue::String(bytes) = value {
            String::from_utf8(bytes.clone()).map_err(|_| "Invalid UTF-8 string")
        } else {
            Err("Expected string")
        }
    }
}

impl TorrentInfo {
    fn from_bencode(bencode: &BencodeValue) -> Result<Self, &'static str> {
        let dict = match bencode {
            BencodeValue::Dict(d) => d,
            _ => return Err("Info must be a dictionary"),
        };

        let mut name = None;
        let mut piece_length = None;
        let mut pieces = None;
        let mut length = None;
        let mut files = None;

        for (key, value) in dict {
            match key.as_slice() {
                b"name" => name = Some(Torrent::parse_string(value)?),
                b"piece length" => piece_length = Some(TorrentInfo::parse_integer(value)?),
                b"pieces" => pieces = Some(TorrentInfo::parse_pieces(value)?),
                b"length" => length = Some(TorrentInfo::parse_integer(value)?),
                b"files" => files = Some(TorrentInfo::parse_files(value)?),
                _ => {}
            }
        }

        Ok(TorrentInfo {
            name: name.ok_or("Missing name")?,
            piece_length: piece_length.ok_or("Missing piece length")?,
            pieces: pieces.ok_or("Missing pieces")?,
            length,
            files,
        })
    }

    fn parse_integer(value: &BencodeValue) -> Result<i64, &'static str> {
        if let BencodeValue::Integer(n) = value {
            Ok(*n)
        } else {
            Err("Expected integer")
        }
    }

    fn parse_pieces(value: &BencodeValue) -> Result<Vec<[u8; 20]>, &'static str> {
        if let BencodeValue::String(bytes) = value {
            if bytes.len() % 20 != 0 {
                return Err("Pieces must be a multiple of 20 bytes");
            }
            Ok(bytes.chunks_exact(20).map(|chunk| chunk.try_into().unwrap()).collect())
        } else {
            Err("Expected pieces string")
        }
    }

    fn parse_files(value: &BencodeValue) -> Result<Vec<TorrentFile>, &'static str> {
        let list = match value {
            BencodeValue::List(l) => l,
            _ => return Err("Files must be a list"),
        };

        let mut files = Vec::new();

        for file_entry in list {
            let file_dict = match file_entry {
                BencodeValue::Dict(d) => d,
                _ => return Err("File entry must be a dictionary"),
            };

            let mut length = None;
            let mut path = None;

            // Parse each key-value pair in the file dictionary
            for (key, value) in file_dict {
                match key.as_slice() {
                    b"length" => {
                        length = Some(TorrentInfo::parse_integer(value)?);
                    }
                    b"path" => {
                        // Path is a list of path component strings
                        let path_components = match value {
                            BencodeValue::List(l) => l,
                            _ => return Err("Path must be a list"),
                        };

                        let mut components = Vec::new();
                        for component in path_components {
                            if let BencodeValue::String(bytes) = component {
                                let s = String::from_utf8(bytes.clone())
                                    .map_err(|_| "Invalid UTF-8 in path")?;
                                components.push(s);
                            } else {
                                return Err("Path component must be a string");
                            }
                        }
                        path = Some(components);
                    }
                    _ => {} // Ignore unknown keys
                }
            }

            // Ensure required fields are present
            let length = length.ok_or("Missing length in file entry")?;
            let path = path.ok_or("Missing path in file entry")?;

            files.push(TorrentFile { length, path });
        }

        Ok(files)
    }
}

impl fmt::Display for Torrent {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Torrent {{\n")?;
        write!(f, "  announce: {}\n", self.announce)?;
        write!(f, "  info: {}\n", self.info)?;
        write!(f, "  info_hash: {:02x?}\n", self.info_hash)?;
        if let Some(comment) = &self.comment {
            write!(f, "  comment: {}\n", comment)?;
        }
        write!(f, "}}")
    }
}

impl fmt::Display for TorrentInfo {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "TorrentInfo {{\n")?;
        write!(f, "  name: {}\n", self.name)?;
        write!(f, "  piece_length: {}\n", self.piece_length)?;
        write!(f, "  pieces: {:?}\n", self.pieces)?;
        if let Some(length) = self.length {
            write!(f, "  length: {}\n", length)?;
        }
        if let Some(files) = &self.files {
            write!(f, "  files: {:?}\n", files)?;
        }
        write!(f, "}}")
    }
}