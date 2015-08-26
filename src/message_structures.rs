extern crate byteorder;
extern crate time;

use std::ops::{Shl, Shr};

use self::byteorder::{BigEndian, WriteBytesExt};
use self::time::Timespec;

pub trait Messageable {
	fn serialize(&self) -> Vec<u8>;
}

pub struct Message<'a> {
	pub protocol_name: Vec<u8>,
    pub protocol_version: Vec<u8>,
    pub message_id: u16,
    pub message_type: u16,
    pub real_message: &'a (Messageable + 'a),
}

impl<'p> Messageable for Message<'p> {
	fn serialize(&self) -> Vec<u8> {
		let mut message_payload = vec![];

		// Protocol Name

		message_payload.push(self.protocol_name.len() as u8);
		message_payload.extend(self.protocol_name.iter().cloned());
		
		// Protocol Version

		message_payload.push(self.protocol_version.len() as u8);
		message_payload.extend(self.protocol_version.iter().cloned());

		// Message Identifier

		let mut message_id_buf = vec![];
		message_id_buf.write_u16::<BigEndian>(self.message_id).unwrap();
		message_payload.extend(message_id_buf.into_iter());
		
		// Message Type

		let mut message_type_buf = vec![];
		message_type_buf.write_u16::<BigEndian>(self.message_type).unwrap();
		message_payload.extend(message_type_buf.into_iter());

		// Actual Message Serialization

		message_payload.extend(self.real_message.serialize());
		return message_payload;
	}
}

pub struct SwarmConfigurationMessage {
	pub client_name: Vec<u8>,
	pub client_version: Vec<u8>,
	pub repositories: Vec<RepositoryInformation>,
}

impl Messageable for SwarmConfigurationMessage {
	fn serialize(&self) -> Vec<u8> {
		let mut message_payload = vec![];
		// Client & Version Information
		message_payload.push(self.client_name.len() as u8);
		message_payload.extend(self.client_name.iter().cloned());
		message_payload.push(self.client_version.len() as u8);
		message_payload.extend(self.client_version.iter().cloned());

		// Repo Information.
		message_payload.push(self.repositories.len() as u8);
		for repo in &self.repositories {
			message_payload.extend(repo.serialize());
		}

		return message_payload;
	}
}

pub struct RepositoryInformation {
	pub path: Vec<u8>,
}

impl Messageable for RepositoryInformation {
	fn serialize(&self) -> Vec<u8> {
		let mut message_payload = vec![];
		message_payload.push(self.path.len() as u8);
		message_payload.extend(self.path.iter().cloned());
		return message_payload;
	}
}

pub struct RepositoryIndexMessage {
	pub directories: Vec<DirectoryInformation>,
}

impl Messageable for RepositoryIndexMessage {
	fn serialize(&self) -> Vec<u8> {
		let mut message_payload = vec![];
		// Directory Listings
		message_payload.push(self.directories.len() as u8);
		for directory in &self.directories {
			message_payload.extend(directory.serialize());
		}
		return message_payload;
	}
}

pub struct DirectoryInformation {
	pub directory_path: Vec<u8>,
	pub files: Vec<FileInformation>,
}

impl Messageable for DirectoryInformation {
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
}

pub struct FileInformation {
	pub filename: Vec<u8>,
	pub modified: Timespec,
	pub version: u32,
	pub local_version: u32,
	pub blocks: Vec<BlockInformation>,
}

impl Messageable for FileInformation {
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
}

pub struct BlockInformation {
	pub size: u32,
	pub hash: Vec<u8>,
}

impl Messageable for BlockInformation {
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
