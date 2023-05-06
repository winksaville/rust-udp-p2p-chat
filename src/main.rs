// This is based on the answer grom GPT-4 to the question:
// "Write a rust app that uses udp for communication to
// implement a chat program. Two instances of this app
// can then be used to send text messages, read as lines
// of text, from stdin and received text messages are
// sent to stdout."

use std::env;
use std::io::{stdin, BufRead};
use std::net::SocketAddr;
use std::sync::Arc;
use tokio::net::UdpSocket;
use tokio::sync::mpsc::{unbounded_channel, UnboundedReceiver};

fn parse_socket_addr(s: &str) -> Result<SocketAddr, Box<dyn std::error::Error>> {
    match s.parse::<SocketAddr>() {
        Ok(addr) => Ok(addr),
        Err(e) => {
            eprintln!("{e} parsing {}", s);
            Err(e.into())
        }
    }
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let args: Vec<String> = env::args().collect();
    if args.len() != 3 {
        eprintln!(
            "Usage: {} <local address:port> <remote address:port>",
            args[0]
        );
        return Ok(());
    }

    let local_addr = parse_socket_addr(&args[1])?;
    let remote_addr = parse_socket_addr(&args[2])?;

    // Using Arc so we can "split" the UdpSocket between two threads
    // from: https://docs.rs/tokio/latest/tokio/net/struct.UdpSocket.html#example-splitting-with-arc
    let socket = Arc::new(UdpSocket::bind(local_addr).await?);

    // Channel for sending messages from stdin
    // to the sender thread in the loop below
    let (tx, rx) = unbounded_channel::<String>();

    // Spawn two threads, one for receiving and one for sending
    tokio::spawn(receiver(socket.clone(), remote_addr));
    tokio::spawn(sender(socket, remote_addr, rx));

    // Loop that reads from stdin and send messages to the sender thread
    let stdin = stdin();
    for line in stdin.lock().lines() {
        let message = line?;
        if message.trim().is_empty() {
            continue;
        }
        tx.send(message).expect("Failed to send message");
    }

    Ok(())
}

async fn receiver(receive_socket: Arc<UdpSocket>, remote_addr: SocketAddr) {
    let mut buf = vec![0u8; 4096];
    loop {
        let (n, addr) = receive_socket
            .recv_from(&mut buf)
            .await
            .expect("Failed to receive data");
        if addr == remote_addr {
            let message = String::from_utf8_lossy(&buf[..n]).to_string();
            println!("{}", message);
        } else {
            eprintln!("Received message from unknown peer {}", addr);
        }
    }
}

async fn sender(
    socket: Arc<UdpSocket>,
    peer_addr: SocketAddr,
    mut rx: UnboundedReceiver<String>,
) {
    while let Some(message) = rx.recv().await {
        if !message.is_empty() {
            if let Err(e) = socket.send_to(message.as_bytes(), &peer_addr).await {
                eprintln!("Failed to send data: {}", e);
            }
        }
    }
}
