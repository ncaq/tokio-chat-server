extern crate bytes;
extern crate futures;
extern crate tokio;

use tokio::net::{TcpListener, TcpStream};
use tokio::prelude::*;

fn broadcast(clients: &[TcpStream], message: &[u8], sender: &TcpStream) {
    for mut client in clients {
        if client.peer_addr().unwrap() != sender.peer_addr().unwrap() {
            client.write(message).ok();
        }
    }
}

fn main() {
    let mut clients = Vec::new();

    let addr = "127.0.0.1:5000".parse().unwrap();
    let listener = TcpListener::bind(&addr).unwrap();

    let server = listener
        .incoming()
        .for_each(move |mut socket| {
            let name = format!("{:?}", socket.peer_addr().unwrap());
            socket
                .write(&format!("{}{}{}", "Welcome ", name, "\n").into_bytes())
                .ok();
            broadcast(
                &clients,
                &format!("{}{}", name, " joined the chat\n").into_bytes(),
                &socket,
            );
            clients.push(socket);
            Ok(())
        }).map_err(|err| {
            // Handle error by printing to STDOUT.
            println!("accept error = {:?}", err);
        });

    println!("server running on localhost:5000");

    tokio::run(server);
}
