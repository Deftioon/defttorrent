use std::collections::HashMap;
use std::error::Error;
use std::io::{self, Read};
use std::net::{SocketAddr, ToSocketAddrs};
use tokio::net::UdpSocket;
use tokio::time::{timeout, Duration};

use super::file;

#[derive(Debug)]
pub struct TorrentItem {
    object: file::Torrent,
    status: String,
    id: usize,
}

pub struct TorrentList {
    pub list: HashMap<usize, TorrentItem>,
}

impl TorrentList {
    pub fn new() -> TorrentList {
        TorrentList {
            list: HashMap::new(),
        }
    }

    pub fn push_with_id_and_url(&mut self, id: usize, url: String) {
        let data = file::read_torrent_file(&url).unwrap();
        let status = "Test Message Hello".to_string();
        self.list.insert(
            id,
            TorrentItem {
                object: data,
                status,
                id,
            },
        );
    }

    pub fn get_status(&mut self, id: &usize) -> String {
        let hashmap = &self.list;
        println!("{:#?}", hashmap);
        let (retrieved_id, retrieved_item) = self.list.get_key_value(id).unwrap();
        assert_eq!(id, retrieved_id);

        let status = &retrieved_item.status;
        status.clone()
    }
}
