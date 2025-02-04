use std::net::{SocketAddr, ToSocketAddrs};
use std::io;
use tokio::net::UdpSocket;
use tokio::time::{timeout, Duration};
use std::error::Error;
use rand;

fn parse_url(url: &str) -> Result<(String, u16, String), &'static str> {
    if !url.starts_with("http://") {
        return Err("URL must start with 'http://'");
    }
    let url = &url[7..];

    let parts: Vec<&str> = url.splitn(2, '/').collect();
    let host_port = parts[0];
    let path = parts.get(1).map_or("", |p| p);

    let mut host = host_port.to_string();
    let mut port = 6969; // Default UDP tracker port

    if let Some(colon_pos) = host_port.find(':') {
        host = host_port[..colon_pos].to_string();
        port = host_port[colon_pos + 1..]
            .parse()
            .map_err(|_| "Invalid port number")?;
    }

    Ok((host, port, path.to_string()))
}

fn parse_peers(buf: &[u8]) -> io::Result<Vec<SocketAddr>> {
    // Implement proper peer parsing according to BitTorrent protocol
    // This is a simplified example that expects 6-byte peer entries
    let peers = buf.chunks_exact(6)
        .filter_map(|chunk| {
            let ip = std::net::Ipv4Addr::new(chunk[0], chunk[1], chunk[2], chunk[3]);
            let port = u16::from_be_bytes([chunk[4], chunk[5]]);
            Some(SocketAddr::from((ip, port)))
        })
        .collect();
    
    Ok(peers)
}

pub async fn request(url: &str, info_hash: &[u8; 20]) -> io::Result<Vec<SocketAddr>> {
    let (host, port, _) = parse_url(url).map_err(|e| io::Error::new(io::ErrorKind::InvalidInput, e))?;
    
    // Resolve DNS (UDP requires IP, not hostname)
    let addr = format!("{}:{}", host, port)
        .to_socket_addrs()?
        .next()
        .ok_or_else(|| io::Error::new(io::ErrorKind::AddrNotAvailable, "DNS lookup failed"))?;
    
    let socket = UdpSocket::bind("0.0.0.0:0").await?;
    socket.connect(addr).await?; // <- Connect to resolved address
    
    // 1. Connect phase
    let conn_id = connect(&socket, &format!("{}:{}", host, port)).await?;
    
    // 2. Announce phase
    let peers = announce(&socket, conn_id, &format!("{}:{}", host, port), info_hash).await?;
    
    Ok(peers)
}

async fn connect(socket: &UdpSocket, addr: &str) -> io::Result<u64> {
    let mut payload = Vec::with_capacity(16);
    let sent_transaction_id = rand::random::<u32>();
    payload.extend(0x41727101980u64.to_be_bytes());
    payload.extend(0u32.to_be_bytes()); // action = connect
    payload.extend(sent_transaction_id.to_be_bytes());

    println!("Sending connect payload to: {:?}", addr);
    socket.send_to(&payload, addr).await?;
    println!("Sent connect payload: {:?}", payload);

    let mut buf = [0u8; 16];
    let (amt, _) = timeout(Duration::from_secs(5), socket.recv_from(&mut buf)).await
        .map_err(|_| io::Error::new(io::ErrorKind::TimedOut, "Connect timeout"))??;
    println!("Received connect response: {:?}", &buf[..amt]);
    
    if amt != 16 || u32::from_be_bytes([buf[0], buf[1], buf[2], buf[3]]) != 0 {
        return Err(io::Error::new(io::ErrorKind::InvalidData, "Invalid connect response"));
    }

    let received_transaction_id = u32::from_be_bytes([buf[4], buf[5], buf[6], buf[7]]);
    if received_transaction_id != sent_transaction_id {
        return Err(io::Error::new(io::ErrorKind::InvalidData, "Transaction ID mismatch"));
    }

    Ok(u64::from_be_bytes([buf[8], buf[9], buf[10], buf[11], buf[12], buf[13], buf[14], buf[15]]))
}

async fn announce(
    socket: &UdpSocket,
    conn_id: u64,
    addr: &str,
    info_hash: &[u8; 20],
) -> io::Result<Vec<SocketAddr>> {
    let mut payload = Vec::with_capacity(98);
    payload.extend(conn_id.to_be_bytes());
    payload.extend(1u32.to_be_bytes()); // action (1 = announce)
    payload.extend(rand::random::<u32>().to_be_bytes()); // transaction_id
    payload.extend(info_hash); // 20-byte info hash

    // Use a valid Azureus-style peer ID
    let peer_id = b"-UT2300-012345678901"; // Must be 20 bytes
    payload.extend(peer_id);

    payload.extend(0u64.to_be_bytes()); // downloaded
    payload.extend(0u64.to_be_bytes()); // left
    payload.extend(0u64.to_be_bytes()); // uploaded
    payload.extend(0u32.to_be_bytes()); // event (0 = none)
    payload.extend(0u32.to_be_bytes()); // ip (0 = default)
    payload.extend(rand::random::<u32>().to_be_bytes()); // key
    payload.extend((-1i32).to_be_bytes()); // num_want (-1 = default)
    payload.extend(6881u16.to_be_bytes()); // port

    socket.send_to(&payload, addr).await?;

    let mut buf = [0u8; 2048];
    let (amt, _) = timeout(Duration::from_secs(10), socket.recv_from(&mut buf)).await
        .map_err(|_| io::Error::new(io::ErrorKind::TimedOut, "Announce timeout"))??;

    if amt < 20 || u32::from_be_bytes([buf[0], buf[1], buf[2], buf[3]]) != 1 {
        return Err(io::Error::new(io::ErrorKind::InvalidData, "Invalid announce response"));
    }

    parse_peers(&buf[20..amt])
}

#[tokio::main]
async fn main() -> io::Result<()> {
    let announce_url = "http://tracker.example.com:6969/announce";
    let info_hash = [0u8; 20]; // Replace with actual info hash
    
    let peers = request(announce_url, &info_hash).await?;
    
    println!("Retrieved peers: {:?}", peers);
    Ok(())
}