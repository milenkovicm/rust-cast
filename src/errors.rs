use std::error::Error as StdError;
use std::fmt;
use std::io::Error as IoError;
use openssl::ssl::error::SslError;
use protobuf::ProtobufError;
use serde_json::error::Error as SerializationError;

#[derive(Debug)]
pub enum Error {
    Internal(String),
    Io(IoError),
    Protobuf(ProtobufError),
    Serialization(SerializationError),
    Ssl(SslError)
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            Error::Internal(ref message) => f.write_str(message),
            Error::Io(ref err) => err.fmt(f),
            Error::Protobuf(ref err) => err.fmt(f),
            Error::Serialization(ref err) => err.fmt(f),
            Error::Ssl(ref err) => err.fmt(f),
        }
    }
}

impl StdError for Error {
    fn description(&self) -> &str {
        match *self {
            Error::Internal(ref message) => message,
            Error::Io(ref err) => err.description(),
            Error::Protobuf(ref err) => err.description(),
            Error::Serialization(ref err) => err.description(),
            Error::Ssl(ref err) => err.description(),
        }
    }

    fn cause(&self) -> Option<&StdError> {
        match *self {
            Error::Io(ref err) => Some(err),
            Error::Protobuf(ref err) => Some(err),
            Error::Ssl(ref err) => Some(err),
            Error::Serialization(ref err) => Some(err),
            Error::Internal(_) => None,
        }
    }
}

impl From<IoError> for Error {
    fn from(err: IoError) -> Error {
        Error::Io(err)
    }
}

impl From<ProtobufError> for Error {
    fn from(err: ProtobufError) -> Error {
        Error::Protobuf(err)
    }
}

impl From<SerializationError> for Error {
    fn from(err: SerializationError) -> Error {
        Error::Serialization(err)
    }
}

impl From<SslError> for Error {
    fn from(err: SslError) -> Error {
        Error::Ssl(err)
    }
}