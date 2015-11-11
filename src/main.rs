extern crate rmp as msgpack;
extern crate time as time;
extern crate byteorder;

mod errors;
mod files;
mod network;
use network::{listener, peer_connection};
mod repositories;
mod shared_constants;

fn main() {

    // Get a listing of the owners + repos in our local data folder.



    // Index file system objects to build local model.

    // Announce existence to tracker server, begin spawning peer connections.

    // Start a service to spawn peer connections on incoming requests.
    listener::start_listening_for_peers(33317);

    peer_connection::initiate_outgoing_peer_connection(&String::from("0.0.0.0"), 33317).unwrap();

    loop {
    }
}
