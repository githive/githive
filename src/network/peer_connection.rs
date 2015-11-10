extern crate byteorder;
use self::byteorder::{BigEndian, WriteBytesExt, ByteOrder};

use std::io::prelude::*;
use std::net::{Shutdown, TcpStream};
use std::sync::mpsc::{channel, Receiver, Sender};
use std::thread;

use errors::Error;
use super::message_structures;
use super::streamutils;
use shared_constants::{CLIENT_NAME, CLIENT_VERSION, PROTOCOL_NAME, PROTOCOL_VERSION};
use files::file_manager::SingleFileManager;
use repositories::OwnerTree;

pub fn initiate_outgoing_peer_connection (ip_string: &str, port_number: u16) -> Result<(), Error> {
    let stream = try!(TcpStream::connect((ip_string, port_number)));
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

		let mut repositories;

		if is_incoming {
			// Accept incoming SwarmConfigurationMessage here, before sending our own.
			repositories = try!(self.receive_swarm_config());
			try!(self.send_swarm_config());
		} else {
			// Initiate the connection with a SwarmConfigurationMessage, and wait for the incoming one.
			try!(self.send_swarm_config());
			repositories = try!(self.receive_swarm_config());
		}

		/*
		Instead of using static repository names here, let's pull the repos this client is interested
		in from the file system as a tree-like structure.
		*/

		let mut owners = vec![];

		let mut owner_instance = OwnerTree{
			owner: String::from("test"),
			repositories: vec![],
		};

		try!(owner_instance.add_repo(String::from("repo")));

		owners.push(owner_instance);

		let repo_paths: Vec<String> = repositories
			.iter()
			.map(|repo| String::from_utf8(repo.path.clone()).unwrap())
			.collect();

		let mut shared_repos = vec![];

		println!("We share these repositories in common: ");

		for owner in owners {
			for repo in &owner.get_repo_names() {
				if repo_paths.contains(&repo) {
					println!("{:?}", &repo);

					shared_repos.push(repo.clone());
				}
			}
		}

		/*
		Here, we will likely have the data folder already indexed. If that were the case, we would
		merely need to announce the indexing metadata to the other client. As it stands, we will need
		to do indexing directly in place. 
		*/

		for single_repo in shared_repos {
			let mut repo_data_path = single_repo.clone();
			repo_data_path.remove(0);
			repo_data_path.push('/');
			repo_data_path.push_str(&String::from("test_file.txt"));
			if is_incoming {
				try!(SingleFileManager::new(&repo_data_path));
			}
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

	    let repo_path = String::from("/test/repo");

	    let message = message_structures::Message::SwarmConfigurationMessage{
	        client_name: String::from(CLIENT_NAME).into_bytes(),
	        client_version: String::from(CLIENT_VERSION).into_bytes(),
	        repositories: vec![
	            message_structures::RepositoryInformation{
	                path: repo_path.into_bytes(),
	            },
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

	fn receive_swarm_config(&mut self) -> Result<Vec<message_structures::RepositoryInformation>, Error> {
		// Let's attempt to get a SwarmConfigurationMessage out of the channel.

		let swarm_config_message = try!(self.rx.recv());
		match swarm_config_message {
			message_structures::Message::SwarmConfigurationMessage{
				client_name,
				client_version,
				repositories,
			} => {
				if String::from_utf8(client_name).unwrap() != CLIENT_NAME || String::from_utf8(client_version).unwrap() != CLIENT_VERSION {
					return Err(Error::UnknownMessageType);
				}
				println!("Received Swarm Config.");

				/* 
				Here is where we would establish which repositories we are interested in for the
				purposes of this client connection. In the first proof of concept, this will be a
				single "repository", but in the future we'll allow peers to interact over multiple
				collections of data.
				*/

				let mut repos = vec![];

				for repo in repositories {
					repos.push(repo);	
				}
				return Ok(repos);
			},
			_ => return Err(Error::NotSwarmConfigurationMessage),
		}
	}

	fn process_message(&mut self, message: message_structures::Message) -> Result<(), Error> {
		match message {
			message_structures::Message::RepositoryIndexMessage{
				directories,
			} => {
				println!("Got a repo index message.")

			},
			_ => return Err(Error::UnknownMessageType),
		}
		// message.print_details();
		Ok(())
	}
}
