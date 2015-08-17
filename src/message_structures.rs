
use std::ops::{Shl, Shr};

pub trait Messageable {
	fn serialize(&self) -> Vec<u8>;
}

pub struct Message<'a> {
	pub protocol_name_length: u8,
	pub protocol_name: Vec<u8>,
	pub protocol_version_length: u8,
    pub protocol_version: Vec<u8>,
    pub message_id: u16,
    pub message_type: u16,
    pub real_message: &'a (Messageable + 'a),
}

pub fn bytes_to_u16(bytes: &[u8]) -> u16 {
    (bytes[0] as u16).shl(8) + 
    bytes[1] as u16
}

fn u16_to_bytes(integer: u16) -> Vec<u8> {
    let first = integer.shr(8) as u16;
    let second = integer - first.shl(8);
    vec![first as u8, second as u8]
}

impl<'p> Messageable for Message<'p> {
	fn serialize(&self) -> Vec<u8> {
		let mut message_payload = vec![];
		message_payload.push(self.protocol_name_length);
		message_payload.extend(self.protocol_name.iter().cloned());
		message_payload.push(self.protocol_version_length);
		message_payload.extend(self.protocol_version.iter().cloned());
		message_payload.extend(u16_to_bytes(self.message_id).into_iter());
		message_payload.extend(u16_to_bytes(self.message_type).into_iter());
		message_payload.extend(self.real_message.serialize());
		return message_payload;
	}
}

pub struct SwarmConfig {
	pub client_name_length: u8,
	pub client_name: Vec<u8>,
	pub client_version_length: u8,
	pub client_version: Vec<u8>,
}

impl Messageable for SwarmConfig {
	fn serialize(&self) -> Vec<u8> {
		let mut message_payload = vec![];
		message_payload.push(self.client_name_length);
		message_payload.extend(self.client_name.iter().cloned());
		message_payload.push(self.client_version_length);
		message_payload.extend(self.client_version.iter().cloned());
		return message_payload;
	}
}

// struct Index {
//     protocol_version: u16,
//     message_id: u64,
//     message_type: u16,
//     length: u64,
// }

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
