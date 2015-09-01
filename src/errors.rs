extern crate byteorder;

use std::{convert, io, any};
use std::sync::mpsc::{SendError, RecvError};

use message_structures::Message;

#[derive(Debug)]
pub enum Error {
	NotSwarmConfigurationMessage,
	UnknownMessageType,
	NotEnoughData(u32),
	ByteOrderError(byteorder::Error),
	IoError(io::Error),
	SendError(SendError<Message>),
	ReceiveError(RecvError),
	Any(Box<any::Any + Send>),
}

// Basic Error Definitions.

impl convert::From<byteorder::Error> for Error {
	fn from(err: byteorder::Error) -> Error {
		Error::ByteOrderError(err)
	}
}

impl convert::From<io::Error> for Error {
	fn from(err: io::Error) -> Error {
		Error::IoError(err)
	}
}

impl convert::From<RecvError> for Error {
    fn from(err: RecvError) -> Error {
        Error::ReceiveError(err)
    }
}

impl convert::From<Box<any::Any + Send>> for Error {
    fn from(err: Box<any::Any + Send>) -> Error {
        Error::Any(err)
    }
}

// Custom Error Definitions.

impl convert::From<SendError<Message>> for Error {
	fn from(err: SendError<Message>) -> Error {
		Error::SendError(err)
	}
}
