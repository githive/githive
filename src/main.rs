extern crate rmp as msgpack;

mod listener;

use std::io;
use std::io::prelude::*;
use std::net::{TcpStream, TcpListener};
use std::thread;
use std::string::String;

fn handle_client(mut stream: TcpStream) {
    println!("Connection received.");
}

fn main() {

    // Index file system objects to build local model.

    // Announce existence to tracker server, begin spawning peer connections.

    // Start a service to spawn peer connections on incoming requests.
    listener::start_listening_for_peers(33317);



    // The following code initiates a connection and sends hello world.

    let mut stream = TcpStream::connect(("0.0.0.0", 33317)).unwrap();
    
    let message = String::from("hello world");

    let _ = stream.write(&message.into_bytes());
    
    let _ = stream.read(&mut [0; 128]);
    loop {
    }
}
