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

    // Spawn threads to listen for incoming requests.
    listener::start_listening_for_peers(33317);

    let mut stream = TcpStream::connect(("0.0.0.0", 33317)).unwrap();
    
    let _ = stream.write(&[1]);
    
    let _ = stream.read(&mut [0; 128]);
    loop {
        // Add code here
    }
}
