extern crate rmp as msgpack;

mod listener;
mod message_structures;

use std::io::prelude::*;
use std::net::TcpStream;
use std::string::String;

use message_structures::Messageable;

fn main() {

    // Index file system objects to build local model.

    // Announce existence to tracker server, begin spawning peer connections.

    // Start a service to spawn peer connections on incoming requests.
    listener::start_listening_for_peers(33317);

    // The following code initiates a connection and sends hello world.

    let mut stream = TcpStream::connect(("0.0.0.0", 33317)).unwrap();
    
    let protocol_name = String::from("Git Hive Protocol");
    let protocol_version = String::from("0.0.1");
    let client_name = String::from("Git Hive");
    let client_version = String::from("0.0.1");

    let message = message_structures::Message{
        protocol_name_length: protocol_name.len() as u8,
        protocol_name: protocol_name.into_bytes(),
        protocol_version_length: protocol_version.len() as u8,
        protocol_version: protocol_version.into_bytes(),
        message_id: 51733 as u16,
        message_type: 0 as u16,
        real_message: &message_structures::SwarmConfig{
            client_name_length: client_name.len() as u8,
            client_name: client_name.into_bytes(),
            client_version_length: client_version.len() as u8,
            client_version: client_version.into_bytes(),
        },
    };

    let _ = stream.write_all(&message.serialize());

    loop {
    }
}
