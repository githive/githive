extern crate byteorder;
extern crate time;

use self::byteorder::{BigEndian, WriteBytesExt, ByteOrder};
use self::time::Timespec;

use std::io::{Error, ErrorKind};
use std::net::TcpStream;

use streamutils::read_bytes_from_stream;

pub enum Message {
	SwarmConfigurationMessage {
		client_name: Vec<u8>,
		client_version: Vec<u8>,
		repositories: Vec<RepositoryInformation>,
	},
	RepositoryIndexMessage {
		directories: Vec<DirectoryInformation>,
	},
}

impl Message {
	pub fn from_stream(message_type: u16, stream: &TcpStream) -> Result<(Message), Error> {
		match message_type {
			0 => {

				// Get client name & version information.

				let client_name_length = try!(read_bytes_from_stream(&stream, 1));
				let client_name = try!(read_bytes_from_stream(&stream, client_name_length[0] as u32));
				let client_version_length = try!(read_bytes_from_stream(&stream, 1));
				let client_version = try!(read_bytes_from_stream(&stream, client_version_length[0] as u32));

				// Get repository information.

				let mut repositories = vec![];

				let number_of_repositories = try!(read_bytes_from_stream(&stream, 1));

				for _ in 0..number_of_repositories[0] {
					let repository_path_length = try!(read_bytes_from_stream(&stream, 1));
					let repository_path = try!(read_bytes_from_stream(&stream, repository_path_length[0] as u32));

					repositories.push(
						RepositoryInformation {
							path: repository_path,
						}
					);
				}

				Ok(
					Message::SwarmConfigurationMessage {
						client_name: client_name,
						client_version: client_version,
						repositories: repositories,
					}
				)
			},
			1 => {

				// Get Directory Information

				let mut directories = vec![];

				let number_of_directories = try!(read_bytes_from_stream(&stream, 1));

				for _ in 0..number_of_directories[0] {
					let directory_path_length = try!(read_bytes_from_stream(&stream, 1));
					let directory_path = try!(read_bytes_from_stream(&stream, directory_path_length[0] as u32));

					let mut files = vec![];

					let number_of_files = try!(read_bytes_from_stream(&stream, 1));

					for _ in 0..number_of_files[0] {
						let length_filename = try!(read_bytes_from_stream(&stream, 1));
						let filename = try!(read_bytes_from_stream(&stream, length_filename[0] as u32));

						let timespec_seconds = BigEndian::read_i64(&try!(read_bytes_from_stream(&stream, 8)));
						let timespec_nanoseconds = BigEndian::read_i32(&try!(read_bytes_from_stream(&stream, 4)));

						let version_counter = BigEndian::read_u32(&try!(read_bytes_from_stream(&stream, 4)));
						let local_version = BigEndian::read_u32(&try!(read_bytes_from_stream(&stream, 4)));

						let mut blocks = vec![];

						let number_of_blocks = try!(read_bytes_from_stream(&stream, 1));

						for _ in 0..number_of_blocks[0] {

							let size = BigEndian::read_u32(&try!(read_bytes_from_stream(&stream, 4)));

							let hash_length = try!(read_bytes_from_stream(&stream, 1));
							let hash = try!(read_bytes_from_stream(&stream, hash_length[0] as u32));

							blocks.push(
								BlockInformation{
									size: size,
									hash: hash,
								}
							)

						}

						files.push(
							FileInformation{
								filename: filename,
								modified: Timespec::new(timespec_seconds, timespec_nanoseconds),
								version: version_counter,
								local_version: local_version,
								blocks: blocks,
							}
						)

					}
					directories.push(DirectoryInformation{
						directory_path: directory_path,
						files: files,
					})
				}
				Ok(
					Message::RepositoryIndexMessage{
						directories: directories,
					}
				)
			}
			_ => return Err(Error::new(ErrorKind::Other, "Unknown Message Type")),
		}
	}

	pub fn serialize(self) -> Result<Vec<u8>, Error> {
		match self {
			Message::SwarmConfigurationMessage{client_name, client_version, repositories} => {

				let mut message_payload = vec![];
				// Client & Version Information
				message_payload.push(client_name.len() as u8);
				message_payload.extend(client_name.iter().cloned());
				message_payload.push(client_version.len() as u8);
				message_payload.extend(client_version.iter().cloned());

				// Repo Information.
				message_payload.push(repositories.len() as u8);
				for repo in repositories {
					message_payload.extend(repo.serialize());
				}

				Ok(message_payload)
			},
			Message::RepositoryIndexMessage{directories} => {
				let mut message_payload = vec![];
				// Directory Listings
				message_payload.push(directories.len() as u8);
				for directory in directories {
					message_payload.extend(directory.serialize());
				}
				Ok(message_payload)
			},
		}
	}

	pub fn print_details(self){
		match self {
			Message::SwarmConfigurationMessage{client_name, client_version, repositories} => {
				println!("client name: {:?}", String::from_utf8(client_name).unwrap());
				println!("client version: {:?}", String::from_utf8(client_version).unwrap());
				
				println!("Number of repository information structures: {:?}", repositories.len());

				for repo in repositories {
					repo.print_details();
				}
			},
			Message::RepositoryIndexMessage{directories} => {
				for directory in directories {
					directory.print_details();
				}
			},
		}
	}
}

pub struct RepositoryInformation{
	pub path: Vec<u8>,
}

impl RepositoryInformation{
	fn serialize(&self) -> Vec<u8>{
		let mut message_payload = vec![];
		message_payload.push(self.path.len() as u8);
		message_payload.extend(self.path.iter().cloned());
		return message_payload;
	}

	fn print_details(self){
		println!("Repository Path: {:?}", String::from_utf8(self.path).unwrap());
	}
}

pub struct DirectoryInformation {
	pub directory_path: Vec<u8>,
	pub files: Vec<FileInformation>,
}

impl DirectoryInformation {
	fn serialize(&self) -> Vec<u8> {
		let mut message_payload = vec![];
		message_payload.push(self.directory_path.len() as u8);
		message_payload.extend(self.directory_path.iter().cloned());

		message_payload.push(self.files.len() as u8);
		for file in &self.files {
			message_payload.extend(file.serialize());
		}
		return message_payload;
	}

	fn print_details(self){
		println!("Directory Path: {:?}", String::from_utf8(self.directory_path).unwrap());

		for file in self.files{
			file.print_details();
		}
	}
}

pub struct FileInformation {
	pub filename: Vec<u8>,
	pub modified: Timespec,
	pub version: u32,
	pub local_version: u32,
	pub blocks: Vec<BlockInformation>,
}

impl FileInformation {
	fn serialize(&self) -> Vec<u8> {
		let mut message_payload = vec![];

		// File Name

		message_payload.push(self.filename.len() as u8);
		message_payload.extend(self.filename.iter().cloned());

		// Modified Timestamp as Time Since Epoch.

		// Seconds Since Epoch

		let mut timespec_second_buf = vec![];
		timespec_second_buf.write_i64::<BigEndian>(self.modified.sec).unwrap();
		message_payload.extend(timespec_second_buf.into_iter());

		// Nanoseconds Since Epoch

		let mut timespec_nanosecond_buf = vec![];
		timespec_nanosecond_buf.write_i32::<BigEndian>(self.modified.nsec).unwrap();
		message_payload.extend(timespec_nanosecond_buf.into_iter());

		// Version Counter

		let mut version_bytes_buf = vec![];
		version_bytes_buf.write_u32::<BigEndian>(self.version).unwrap();
		message_payload.extend(version_bytes_buf.into_iter());

		// Local Version Counter

		let mut local_version_bytes_buf = vec![];
		local_version_bytes_buf.write_u32::<BigEndian>(self.local_version).unwrap();
		message_payload.extend(local_version_bytes_buf.into_iter());

		// Block Information Serialization

		message_payload.push(self.blocks.len() as u8);
		for block in &self.blocks {
			message_payload.extend(block.serialize());
		}

		return message_payload;
	}

	fn print_details(self){
		println!("Filename: {:?}", String::from_utf8(self.filename).unwrap());

		println!("Modified Time: {:?}", time::strftime("%F %T", &time::at(self.modified)).unwrap());

		println!("Version Counter: {:?}", self.version);
		println!("Local Version: {:?}", self.local_version);

		for block in self.blocks {
			block.print_details();
		}
	}
}

pub struct BlockInformation {
	pub size: u32,
	pub hash: Vec<u8>,
}

impl BlockInformation {
	fn serialize(&self) -> Vec<u8> {
		let mut message_payload = vec![];

		// Block Size

		let mut block_size_bytes_buf = vec![];
		block_size_bytes_buf.write_u32::<BigEndian>(self.size).unwrap();
		message_payload.extend(block_size_bytes_buf.into_iter());

		// Block Hash

		message_payload.push(self.hash.len() as u8);
		message_payload.extend(self.hash.iter().cloned());

		return message_payload;
	}

	fn print_details(self){
		println!("Size: {:?}", self.size);
		println!("Hash: {:?}", String::from_utf8(self.hash).unwrap());
	}
}

// struct Request {
//     protocol_version: u16,
//     message_id: u64,
//     message_type: u16,
//     length: u64,
// }

// struct Response {
//     protocol_version: u16,
//     message_id: u64,
//     message_type: u16,
//     length: u64,
// }
