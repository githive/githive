use std::net::{TcpListener, TcpStream};
use std::thread;
use std::thread::JoinHandle;

use peer_connection;

pub fn start_listening_for_peers(port: u16) -> JoinHandle<()> {
	let tcp_listener = TcpListener::bind(("0.0.0.0", port)).unwrap();
	thread::spawn(move || {
		for stream in tcp_listener.incoming() {
			match stream {
				Ok(stream) => {spawn_peer_connection(stream); Ok(())},
				Err(e) => Err(e),
			}.unwrap();
		}
	})
}

fn spawn_peer_connection(stream: TcpStream) {
	thread::spawn(move || {
		match peer_connection::accept_incoming_peer_connection(stream) {
			Ok(_) => println!("Peer connection fully closed."),
			Err(e) => println!("Experienced an error during peer connection: {:?}", e),
		}
	});
}
