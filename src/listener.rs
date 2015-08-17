use std::io::{Error, ErrorKind};
use std::io::Read;
use std::net::{TcpListener, TcpStream};
use std::thread;
use std::thread::JoinHandle;
use std::string::String;

use message_structures::bytes_to_u16;

pub fn start_listening_for_peers(port: u16) -> JoinHandle<()> {
	let tcp_listener = TcpListener::bind(("0.0.0.0", port)).unwrap();
	thread::spawn(move || {
		for stream in tcp_listener.incoming() {
			match stream {
				Ok(s) => handle_connection(s),
				Err(e) => Err(e),
			};
		}
	})
}

fn read_bytes_from_stream(stream: &TcpStream, number_of_bytes: u32) -> Result<Vec<u8>, Error> {
	let mut buffer = vec![];
	let bytes_read = stream.take(number_of_bytes as u64).read_to_end(&mut buffer);
	match bytes_read {
		Ok(n) if n == number_of_bytes as usize => Ok(buffer),
		Ok(n) => Err(Error::new(ErrorKind::Other, format!("Not Enough Data! {}", n))),
        Err(e) => Err(e),
	}
}

fn handle_connection(stream: TcpStream) -> Result<(), Error>{
	println!("Got a connection from a peer!");

	// Check protocol name and version

	let protocol_name_length = try!(read_bytes_from_stream(&stream, 1));
	let procotol_name = try!(read_bytes_from_stream(&stream, protocol_name_length[0] as u32));
	let protocol_version_length = try!(read_bytes_from_stream(&stream, 1));
	let protocol_version = try!(read_bytes_from_stream(&stream, protocol_version_length[0] as u32));
	let message_id = bytes_to_u16(&try!(read_bytes_from_stream(&stream, 2)));
	let message_type = bytes_to_u16(&try!(read_bytes_from_stream(&stream, 2)));


	println!("Received message with following information:");
	println!("Protocol Name: {:?}", String::from_utf8(procotol_name).unwrap());
	println!("Protocol Version: {:?}", String::from_utf8(protocol_version).unwrap());
	println!("Message ID: {:?}", message_id);
	println!("Message Type: {:?}", message_type);

	if message_type == 0 {
		println!("Swarm Config Options");
		let client_name_length = try!(read_bytes_from_stream(&stream, 1));
		let client_name = try!(read_bytes_from_stream(&stream, client_name_length[0] as u32));
		let client_version_length = try!(read_bytes_from_stream(&stream, 1));
		let client_version = try!(read_bytes_from_stream(&stream, client_version_length[0] as u32));

		println!("Client Name: {:?}", String::from_utf8(client_name).unwrap());
		println!("Client Version: {:?}", String::from_utf8(client_version).unwrap());
	}

	Ok(())

	// Initiate connection for peering as usual.
		
}
