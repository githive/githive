use std::io::{Read, Write};
use std::net::{TcpListener, TcpStream};
use std::thread;
use std::thread::JoinHandle;
use std::error::Error;
use std::string::String;

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

	// The following pulls an 11 byte string out of the stream and prints it.

	let mut buf = vec![];
	let bytes_read = stream.take(11).read_to_end(&mut buf);
	match bytes_read {
		Ok(n) => if n == 11 as usize { 
			let s = match String::from_utf8(buf) {
				Ok(v) => v,
				Err(e) => String::from("Invalid UTF-8 sequence"),
			};
			println!("{:?}", s);},
        Err(e) => println!("error"),
	}
	
}
