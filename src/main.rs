// This is the answer grom GPT-4 to the question:
// "Write a rust app that uses udp for communication to
// implement a chat program. Two instances of this app
// can then be used to send text messages, read as lines
// of text, from stdin and received text messages are
// sent to stdout."

use std::env;
use std::io::{stdin, BufRead};
use std::net::SocketAddr;
use tokio::net::UdpSocket;
use futures::{StreamExt, SinkExt};
use futures::channel::mpsc::{unbounded, UnboundedSender, UnboundedReceiver};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let args: Vec<String> = env::args().collect();
    if args.len() != 3 {
        eprintln!("Usage: {} <local address:port> <remote address:port>", args[0]);
        return Ok(());
    }

    let local_addr = args[1].parse::<SocketAddr>()?;
    let remote_addr = args[2].parse::<SocketAddr>()?;

    let socket = UdpSocket::bind(local_addr).await?;

    let (tx, rx) = unbounded();

    tokio::spawn(receiver(socket, remote_addr, tx));
    tokio::spawn(sender(rx));

    let stdin = stdin();
    for line in stdin.lock().lines() {
        let message = line?;
        if message.trim().is_empty() {
            continue;
        }
        tx.unbounded_send(message).expect("Failed to send message");
    }

    Ok(())
}

async fn receiver(socket: UdpSocket, remote_addr: SocketAddr, tx: UnboundedSender<String>) {
    let mut buf = vec![0u8; 4096];
    loop {
        let (n, addr) = socket.recv_from(&mut buf).await.expect("Failed to receive data");
        if addr == remote_addr {
            let message = String::from_utf8_lossy(&buf[..n]).to_string();
            tx.unbounded_send(message).expect("Failed to send received message");
        }
    }
}

async fn sender(rx: UnboundedReceiver<String>) {
    while let Some(message) = rx.next().await {
        println!("{}", message);
    }
}
