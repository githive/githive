extern crate byteorder;
extern crate time;

use self::byteorder::BigEndian;
use self::byteorder::ByteOrder;
use self::time::Timespec;

use std::io::{Error, ErrorKind};
use std::io::Read;
use std::net::{TcpListener, TcpStream};
use std::thread;
use std::thread::JoinHandle;
use std::string::String;

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
	println!("");



	// Check protocol name and version

	let protocol_name_length = try!(read_bytes_from_stream(&stream, 1));
	let procotol_name = try!(read_bytes_from_stream(&stream, protocol_name_length[0] as u32));
	let protocol_version_length = try!(read_bytes_from_stream(&stream, 1));
	let protocol_version = try!(read_bytes_from_stream(&stream, protocol_version_length[0] as u32));

	// Get message identifier and type.

	let message_id = BigEndian::read_u16(&try!(read_bytes_from_stream(&stream, 2)));
	let message_type = BigEndian::read_u16(&try!(read_bytes_from_stream(&stream, 2)));


	println!("Received message with following information:");
	println!("Protocol Name: {:?}", String::from_utf8(procotol_name).unwrap());
	println!("Protocol Version: {:?}", String::from_utf8(protocol_version).unwrap());
	println!("Message ID: {:?}", message_id);
	println!("Message Type: {:?}", message_type);
	println!("");

	if message_type == 0 {
		println!("Swarm Config Options");

		// Get client name & version information.

		let client_name_length = try!(read_bytes_from_stream(&stream, 1));
		let client_name = try!(read_bytes_from_stream(&stream, client_name_length[0] as u32));
		let client_version_length = try!(read_bytes_from_stream(&stream, 1));
		let client_version = try!(read_bytes_from_stream(&stream, client_version_length[0] as u32));


		println!("Client Name: {:?}", String::from_utf8(client_name).unwrap());
		println!("Client Version: {:?}", String::from_utf8(client_version).unwrap());

		// Get repository information.

		println!("Interested in the following repos:");
		let number_of_repositories = try!(read_bytes_from_stream(&stream, 1));
		for i in 0..number_of_repositories[0] {
			let repository_path_length = try!(read_bytes_from_stream(&stream, 1));
			let repository_path = try!(read_bytes_from_stream(&stream, repository_path_length[0] as u32));
			println!("{:?}", String::from_utf8(repository_path).unwrap());			
		}
	}
	println!("");
	println!("");
	println!("");

	// Check protocol name and version

	let protocol_name_length = try!(read_bytes_from_stream(&stream, 1));
	let procotol_name = try!(read_bytes_from_stream(&stream, protocol_name_length[0] as u32));
	let protocol_version_length = try!(read_bytes_from_stream(&stream, 1));
	let protocol_version = try!(read_bytes_from_stream(&stream, protocol_version_length[0] as u32));

	// Get message identifier and type.

	let message_id = BigEndian::read_u16(&try!(read_bytes_from_stream(&stream, 2)));
	let message_type = BigEndian::read_u16(&try!(read_bytes_from_stream(&stream, 2)));


	println!("Received message with following information:");
	println!("Protocol Name: {:?}", String::from_utf8(procotol_name).unwrap());
	println!("Protocol Version: {:?}", String::from_utf8(protocol_version).unwrap());
	println!("Message ID: {:?}", message_id);
	println!("Message Type: {:?}", message_type);
	println!("");

	if message_type == 1 {
		println!("Repository Index Information");

		// Get client name & version information.

		let number_of_directories = try!(read_bytes_from_stream(&stream, 1));
		for i in 0..number_of_directories[0] {
			let directory_path_length = try!(read_bytes_from_stream(&stream, 1));
			let directory_path = try!(read_bytes_from_stream(&stream, directory_path_length[0] as u32));

			println!("Directory: {:?}", String::from_utf8(directory_path).unwrap());

			let number_of_files = try!(read_bytes_from_stream(&stream, 1));
			for j in 0..number_of_files[0] {
				let length_filename = try!(read_bytes_from_stream(&stream, 1));
				let filename = try!(read_bytes_from_stream(&stream, length_filename[0] as u32));

				println!("Filename: {:?}", String::from_utf8(filename).unwrap());

				let timespec_seconds = BigEndian::read_i64(&try!(read_bytes_from_stream(&stream, 8)));
				let timespec_nanoseconds = BigEndian::read_i32(&try!(read_bytes_from_stream(&stream, 4)));
				let time_in_tm = time::at(Timespec::new(timespec_seconds, timespec_nanoseconds));

				println!("Modified Time: {:?}", time::strftime("%F %T", &time_in_tm).unwrap());

				let version_counter = BigEndian::read_u32(&try!(read_bytes_from_stream(&stream, 4)));
				let local_version = BigEndian::read_u32(&try!(read_bytes_from_stream(&stream, 4)));

				println!("Version Counter: {:?}", version_counter);
				println!("Local Version: {:?}", local_version);

				let number_of_blocks = try!(read_bytes_from_stream(&stream, 1));

				println!("Block Count: {:?} blocks", number_of_blocks[0]);

				for k in 0..number_of_blocks[0] {

					println!("Block {:?}: ", i);

					let size = BigEndian::read_u32(&try!(read_bytes_from_stream(&stream, 4)));

					println!("Size: {:?}", size);

					let hash_length = try!(read_bytes_from_stream(&stream, 1));
					let hash = try!(read_bytes_from_stream(&stream, hash_length[0] as u32));

					println!("Hash: {:?}", String::from_utf8(hash).unwrap());
				}
			}
		}
	}

	Ok(())

	// Initiate connection for peering as usual.
		
}
