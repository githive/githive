extern crate byteorder;

use self::byteorder::BigEndian;
use self::byteorder::ByteOrder;

use std::io::{Error, ErrorKind};
use std::io::Read;
use std::net::TcpStream;

use message_structures::Message;
use shared_constants::{PROTOCOL_NAME, PROTOCOL_VERSION};

pub fn read_bytes_from_stream(stream: &TcpStream, number_of_bytes: u32) -> Result<Vec<u8>, Error> {
	let mut buffer = vec![];
	let bytes_read = stream.take(number_of_bytes as u64).read_to_end(&mut buffer);
	match bytes_read {
		Ok(n) if n == number_of_bytes as usize => Ok(buffer),
		Ok(n) => Err(Error::new(ErrorKind::Other, format!("Not Enough Data! {}", n))),
        Err(e) => Err(e),
	}
}

pub struct TcpStreamPump {
    stream: TcpStream,
    // tx: Sender<>,
}

impl TcpStreamPump {
    // fn start_pumping_message_to_channel(stream: TcpStream, tx: Sender<>) {
    pub fn start_pumping_message_to_channel(stream: TcpStream) {
        let mut funnel = TcpStreamPump {
            stream: stream,
        };
        match funnel.run() {
            Ok(_) => {},
            Err(e) => println!("Error: {:?}", e)
        }
    }

    fn run(&mut self) -> Result<(), Error> {
        loop {
            try!(self.receive_message());
            // try!(self.tx.send(IPC::Message(message)));
        }
    }

    fn receive_message(&mut self) -> Result<(), Error> {
    	println!("Trying to get a message.");

		// Check protocol name and version

		let protocol_name_length = try!(read_bytes_from_stream(&self.stream, 1));
		let protocol_name = String::from_utf8(try!(read_bytes_from_stream(&self.stream, protocol_name_length[0] as u32))).unwrap();
		let protocol_version_length = try!(read_bytes_from_stream(&self.stream, 1));
		let protocol_version = String::from_utf8(try!(read_bytes_from_stream(&self.stream, protocol_version_length[0] as u32))).unwrap();

		if protocol_name != PROTOCOL_NAME || protocol_version != PROTOCOL_VERSION {
			panic!("Peer sent message with invalid protocol name or version. Expected protocol: '{}', '{}'", PROTOCOL_NAME, PROTOCOL_VERSION);
		}

		println!("Got some meta data!\n");

		println!("protocol name: {:?}", protocol_name);
		println!("protocol version: {:?}", protocol_version);

		// Get message identifier and type.

		let message_id = BigEndian::read_u16(&try!(read_bytes_from_stream(&self.stream, 2)));
		let message_type = BigEndian::read_u16(&try!(read_bytes_from_stream(&self.stream, 2)));

		println!("message ID: {:?}", message_id);
		println!("Message Type: {:?}", message_type);

		let message = try!(Message::from_stream(message_type, &self.stream));

		message.print_details();

    	println!("\n\n");
    	Ok(())
    }
}
