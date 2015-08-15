extern crate rmp as msgpack;

use std::io;
use std::io::prelude::*;
use std::net::{TcpStream, TcpListener};
use std::thread;
use std::string::String;

fn handle_client(mut stream: TcpStream) {
    println!("Connection received.");
}

fn main() {

    thread::spawn(move|| {
        let listener = TcpListener::bind("127.0.0.1:33017").unwrap();
        for stream in listener.incoming() {
            match stream {
                Ok(stream) => {
                    thread::spawn(move|| {
                        // connection succeeded
                        handle_client(stream)
                    });
                }
                Err(e) => {println!("error condition {}", e);}
            }
        }
    });

    let mut stream = TcpStream::connect("127.0.0.1:33017").unwrap();
    
    let _ = stream.write(&[1]);
    
    let _ = stream.read(&mut [0; 128]);
}
