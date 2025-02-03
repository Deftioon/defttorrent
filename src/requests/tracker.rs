use std::{io, net::{SocketAddr, ToSocketAddrs, UdpSocket}};

fn parse_url(url: &str) -> Result<(String, u16, String), &'static str> {
    if !url.starts_with("http://") {
        return Err("URL must start with 'http://'");
    }

    let url = &url[7..];

    let parts: Vec<&str> = url.split('/').collect();
    let host_port = parts[0];
    let path = parts[1..].join("/");

    let host_port_parts: Vec<&str> = host_port.split(':').collect();
    
    let host = host_port_parts[0].to_string();
    let port: u16 = host_port_parts[1].parse().map_err(|_| "Invalid port number")?;

    Ok((host, port, path))
}

pub fn request(url: &str, payload: &[u8]) -> io::Result<()> {
    let socket = UdpSocket::bind("127.0.0.1:34254").expect("Could not bind address");
    let (host, port, path) = parse_url(url).expect("Error occured when parsing url");
    let url = format!("{}:{}", host, port);
    let addr = url.to_socket_addrs()?.next().expect("Could not resolve address");
    println!("{}/{}", url, path);

    socket.send_to(payload, addr);

    let mut buf = [0; 1024];

    let (amt, src) = socket.recv_from(&mut buf)?;
    println!("Received {} bytes from {}: {:?}", amt, src, &buf[..amt]);

    Ok(())
}

fn main() -> io::Result<()> {
    // Bind the socket to a local address
    let socket = UdpSocket::bind("127.0.0.1:34254")?;
    
    // Define the message and the address to send to
    let msg = b"A";
    let addr: SocketAddr = "127.0.0.1:34254".parse().expect("Error occured when parsing url");
    
    // Send the message to the address
    socket.send_to(msg, addr)?;
    
    // Buffer to store the received message
    let mut buf = [0; 100];
    
    // Receive the message
    let (amt, src) = socket.recv_from(&mut buf)?;
    
    // Print the received message
    println!("Received {} bytes from {}: {:?}", amt, src, &buf[..amt]);
    
    Ok(())
}