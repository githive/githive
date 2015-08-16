use std::net::{TcpListener, TcpStream};
use std::thread;
use std::thread::JoinHandle;

pub fn start_listening_for_peers(port: u16) -> JoinHandle<()> {
	let tcp_listener = TcpListener::bind(("0.0.0.0", port)).unwrap();
	thread::spawn(move || {
		for stream in tcp_listener.incoming() {
			match stream {
				Ok(s) => handle_connection(s),
				Err(e) => println!("Error: {:?}", e),
			}
		}
	})
}

fn handle_connection(stream: TcpStream) {
	println!("Got a connection from a peer!");
	// Initiate connection for peering as usual.
}
