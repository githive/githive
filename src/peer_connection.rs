extern crate byteorder;
use self::byteorder::{BigEndian, WriteBytesExt, ByteOrder};

use std::io::prelude::*;
use std::net::{Shutdown, TcpStream};
use std::sync::mpsc::{channel, Receiver, Sender};
use std::thread;

use errors::Error;
use message_structures;
use streamutils;
use shared_constants::{CLIENT_NAME, CLIENT_VERSION, PROTOCOL_NAME, PROTOCOL_VERSION};

pub fn initiate_outgoing_peer_connection (stream: TcpStream) -> Result<(), Error> {
	PeerConnection::initiate_outgoing_peer_connection(stream)
}

pub fn accept_incoming_peer_connection (stream: TcpStream) -> Result<(), Error> {
	PeerConnection::accept_incoming_peer_connection(stream)
}


struct PeerConnection {
	choke: bool,
	stream: TcpStream,
	rx: Receiver<message_structures::Message>,
	tx: Sender<message_structures::Message>,
}

impl PeerConnection {
	fn initiate_outgoing_peer_connection(stream: TcpStream) -> Result<(), Error> {
		println!("Beginning new peer connection.");
		PeerConnection::new(stream, false)
	}

	fn accept_incoming_peer_connection(stream: TcpStream) -> Result<(), Error> {
		println!("Got a new peer connection.");
		PeerConnection::new(stream, true)
	}

	fn new(stream: TcpStream, is_incoming: bool) -> Result<(), Error> {
		// Do any start up tasks here.

		let (tx, rx) = channel::<message_structures::Message>();

		// Actually create the peer connection.

		let this_peer_connection = PeerConnection {
			choke: false,
			stream: stream,
			rx: rx,
			tx: tx,
		};

		// Automatically raise errors above this method if they're encountered during runtime.

		try!(this_peer_connection.run(is_incoming));

		// Once we're done with the run loop, we'll alert that we've dropped this peer connection.

		println!("Peer Disconnected");
		Ok(())
	}

	fn run(mut self, is_incoming: bool) -> Result<(), Error> {
		// Spawn the TcpStreamPump

		let tx_clone = self.tx.clone();
		let stream_clone = self.stream.try_clone().unwrap();
		let stream_pump_thread = thread::spawn(move || streamutils::TcpStreamPump::start_pumping_message_to_channel(stream_clone, tx_clone));

		if is_incoming {
			// Accept incoming SwarmConfigurationMessage here, before sending our own.
			try!(self.receive_swarm_config());
			try!(self.send_swarm_config());
		} else {
			// Initiate the connection with a SwarmConfigurationMessage, and wait for the incoming one.
			try!(self.send_swarm_config());
			try!(self.receive_swarm_config());
		}

		// Process additional message until we're told to choke.

		while !self.choke {
			let next_message = try!(self.rx.recv());
			try!(self.process_message(next_message));
		}

		println!("Disconnecting from peer.");

		try!(self.stream.shutdown(Shutdown::Both));
		try!(stream_pump_thread.join());

		Ok(())

	}

	fn send_swarm_config(&mut self) -> Result<(), Error> {

	    let repo_path = String::from("/githive/githive-protocol");
	    let repo_path_two = String::from("/githive/githive-client");

	    let message = message_structures::Message::SwarmConfigurationMessage{
	        client_name: String::from(CLIENT_NAME).into_bytes(),
	        client_version: String::from(CLIENT_VERSION).into_bytes(),
	        repositories: vec![
	            message_structures::RepositoryInformation{
	                path: repo_path.into_bytes(),
	            },
	            message_structures::RepositoryInformation{
	                path: repo_path_two.into_bytes(),
	            }
	        ],
	    };

	    let mut message_metadata = vec![];
	    message_metadata.push(PROTOCOL_NAME.len() as u8);
	    message_metadata.extend(String::from(PROTOCOL_NAME).into_bytes());
	    message_metadata.push(PROTOCOL_VERSION.len() as u8);
	    message_metadata.extend(String::from(PROTOCOL_VERSION).into_bytes());

	    let mut message_id_buf = vec![];
	    try!(message_id_buf.write_u16::<BigEndian>(51733 as u16));
	    message_metadata.extend(message_id_buf.into_iter());

	    let mut message_type_buf = vec![];
	    try!(message_type_buf.write_u16::<BigEndian>(0 as u16));
	    message_metadata.extend(message_type_buf.into_iter());

	    message_metadata.extend(try!(message.serialize()));

	    try!(self.stream.write_all(&message_metadata));

	    Ok(())
	}

	fn receive_swarm_config(&mut self) -> Result<(), Error> {
		// Let's attempt to get a SwarmConfigurationMessage out of the channel.

		let mut swarm_config_message = try!(self.rx.recv());
		match swarm_config_message {
			message_structures::Message::SwarmConfigurationMessage{
				client_name,
				client_version,
				repositories,
			} => {
				println!("Received Swarm Config.");
				println!("Interested in repos: ");
				for repo in repositories {
					repo.print_details();	
				}
			},
			_ => return Err(Error::NotSwarmConfigurationMessage),
		}

		Ok(())
	}

	fn process_message(&mut self, message: message_structures::Message) -> Result<(), Error> {
		message.print_details();
		Ok(())
	}
}
