extern crate byteorder;

use self::byteorder::BigEndian;
use self::byteorder::ByteOrder;

use std::io::Read;
use std::net::TcpStream;
use std::sync::mpsc::Sender;

use errors::Error;
use super::message_structures::Message;
use shared_constants::{PROTOCOL_NAME, PROTOCOL_VERSION};

pub fn read_bytes_from_stream(stream: &TcpStream, number_of_bytes: u32) -> Result<Vec<u8>, Error> {
	let mut buffer = vec![];
	let bytes_read = stream.take(number_of_bytes as u64).read_to_end(&mut buffer);
	match bytes_read {
		Ok(n) if n == number_of_bytes as usize => Ok(buffer),
		Ok(n) => Err(Error::NotEnoughData(n as u32)),
        Err(e) => try!(Err(e)),
	}
}

pub struct TcpStreamPump {
    stream: TcpStream,
    tx: Sender<Message>,
}

impl TcpStreamPump {
    // fn start_pumping_message_to_channel(stream: TcpStream, tx: Sender<>) {
    pub fn start_pumping_message_to_channel(stream: TcpStream, tx: Sender<Message>) {
        let mut funnel = TcpStreamPump {
            stream: stream,
            tx: tx,
        };
        match funnel.run() {
            Ok(_) => {},
            Err(e) => println!("Error: {:?}", e)
        }
    }

    fn run(&mut self) -> Result<(), Error> {
        loop {
            let message = try!(self.receive_message());
            try!(self.tx.send(message));
        }
    }

    fn receive_message(&mut self) -> Result<Message, Error> {

		// Check protocol name and version

		let protocol_name_length = try!(read_bytes_from_stream(&self.stream, 1));
		let protocol_name = String::from_utf8(try!(read_bytes_from_stream(&self.stream, protocol_name_length[0] as u32))).unwrap();
		let protocol_version_length = try!(read_bytes_from_stream(&self.stream, 1));
		let protocol_version = String::from_utf8(try!(read_bytes_from_stream(&self.stream, protocol_version_length[0] as u32))).unwrap();

		if protocol_name != PROTOCOL_NAME || protocol_version != PROTOCOL_VERSION {
			panic!("Peer sent message with invalid protocol name or version. Expected protocol: '{}', '{}'", PROTOCOL_NAME, PROTOCOL_VERSION);
		}

		// Get message identifier and type.

		let message_id = BigEndian::read_u16(&try!(read_bytes_from_stream(&self.stream, 2)));
		let message_type = BigEndian::read_u16(&try!(read_bytes_from_stream(&self.stream, 2)));

		let message = try!(Message::from_stream(message_type, &self.stream));

    	return Ok(message);
    }
}
