extern crate rmp as msgpack;
extern crate time as time;
extern crate byteorder;

use byteorder::{BigEndian, WriteBytesExt, ByteOrder};

use std::io::prelude::*;
use std::net::TcpStream;
use std::string::String;

mod listener;
mod message_structures;
mod streamutils;
mod shared_constants;

use shared_constants::{CLIENT_NAME, CLIENT_VERSION, PROTOCOL_NAME, PROTOCOL_VERSION};

fn main() {

    // Index file system objects to build local model.

    // Announce existence to tracker server, begin spawning peer connections.

    // Start a service to spawn peer connections on incoming requests.
    listener::start_listening_for_peers(33317);

    // The following code initiates a connection and sends hello world.

    let mut stream = TcpStream::connect(("0.0.0.0", 33317)).unwrap();

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
    message_id_buf.write_u16::<BigEndian>(51733 as u16).unwrap();
    message_metadata.extend(message_id_buf.into_iter());

    let mut message_type_buf = vec![];
    message_type_buf.write_u16::<BigEndian>(0 as u16).unwrap();
    message_metadata.extend(message_type_buf.into_iter());

    message_metadata.extend(message.serialize().unwrap());

    stream.write_all(&message_metadata).unwrap();

    let protocol_name = String::from("Git Hive Protocol");
    let protocol_version = String::from("0.0.1");

    let directory_one = String::from("/");

    let file_one_name = String::from("README.md");

    let block_hash = String::from("e3b0c44298fc1c149afbf4c8996fb92427ae41e4649b934ca495991b7852b855");

    let message = message_structures::Message::RepositoryIndexMessage {
        directories: vec![
            message_structures::DirectoryInformation{
                directory_path: directory_one.into_bytes(),
                files: vec![
                    message_structures::FileInformation{
                        filename: file_one_name.into_bytes(),
                        modified: time::get_time(),
                        version: 0 as u32,
                        local_version: 0 as u32,
                        blocks: vec![
                            message_structures::BlockInformation{
                                size: 1 as u32,
                                hash: block_hash.into_bytes(),
                            }
                        ],
                    }
                ],
            }
        ],
    };

    let mut message_metadata = vec![];
    message_metadata.push(protocol_name.len() as u8);
    message_metadata.extend(protocol_name.into_bytes());
    message_metadata.push(protocol_version.len() as u8);
    message_metadata.extend(protocol_version.into_bytes());

    let mut message_id_buf = vec![];
    message_id_buf.write_u16::<BigEndian>(51734 as u16).unwrap();
    message_metadata.extend(message_id_buf.into_iter());

    let mut message_type_buf = vec![];
    message_type_buf.write_u16::<BigEndian>(1 as u16).unwrap();
    message_metadata.extend(message_type_buf.into_iter());

    message_metadata.extend(message.serialize().unwrap());

    stream.write_all(&message_metadata).unwrap();

    // let _ = stream.write_all(&message.serialize());

    loop {
    }
}
