extern crate rmp as msgpack;
extern crate time as time;
extern crate byteorder;

mod errors;
mod listener;
mod message_structures;
mod streamutils;
mod shared_constants;
mod peer_connection;
mod file_manager;
mod repositories;

fn main() {

    // Index file system objects to build local model.

    // Announce existence to tracker server, begin spawning peer connections.

    // Start a service to spawn peer connections on incoming requests.
    listener::start_listening_for_peers(33317);

    peer_connection::initiate_outgoing_peer_connection(&String::from("0.0.0.0"), 33317).unwrap();

    loop {
    }
}
