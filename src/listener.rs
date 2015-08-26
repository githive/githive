use std::net::TcpListener;
use std::thread;
use std::thread::JoinHandle;

use streamutils::TcpStreamPump;

pub fn start_listening_for_peers(port: u16) -> JoinHandle<()> {
	let tcp_listener = TcpListener::bind(("0.0.0.0", port)).unwrap();
	thread::spawn(move || {
		for stream in tcp_listener.incoming() {
			match stream {
				Ok(s) => {TcpStreamPump::start_pumping_message_to_channel(s); Ok(())},
				Err(e) => Err(e),
			}.unwrap();
		}
	})
}
