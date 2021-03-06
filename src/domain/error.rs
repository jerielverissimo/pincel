use x11rb::{
    errors::ConnectionError,
    rust_connection::{ConnectError, ReplyError, ReplyOrIdError},
};

use std::error::Error;
use std::fmt::{Display, Formatter};

pub type Result = std::result::Result<(), PincelError>;

#[derive(Debug)]
pub enum PincelError {
    MissingWinParams,
    ConnectionError(ConnectionError),
    ReplyOrIdError(ReplyOrIdError),
    ReplyError(ReplyError),
    ConnectError(ConnectError),
    XlibError(x11rb::protocol::Error),
    GenericError(Box<dyn Error>),
}

impl Display for PincelError {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::MissingWinParams => return write!(f, "Missing window param!!!"),
            Self::ConnectionError(err) => return write!(f, "Error: {:?}", err),
            Self::ReplyOrIdError(err) => return write!(f, "Error: {:?}", err),
            Self::ReplyError(err) => return write!(f, "Error: {:?}", err),
            Self::ConnectError(err) => return write!(f, "Error: {:?}", err),
            Self::XlibError(err) => return write!(f, "Error: {:?}", err),
            Self::GenericError(err) => return write!(f, "Error: {:?}", err),
        }
    }
}

impl Error for PincelError {}

impl From<Box<dyn Error>> for PincelError {
    fn from(e: Box<dyn Error>) -> Self {
        Self::GenericError(e)
    }
}

impl From<ConnectionError> for PincelError {
    fn from(e: ConnectionError) -> Self {
        PincelError::ConnectionError(e)
    }
}

impl From<ReplyOrIdError> for PincelError {
    fn from(e: ReplyOrIdError) -> Self {
        PincelError::ReplyOrIdError(e)
    }
}

impl From<ReplyError> for PincelError {
    fn from(e: ReplyError) -> Self {
        PincelError::ReplyError(e)
    }
}

impl From<ConnectError> for PincelError {
    fn from(e: ConnectError) -> Self {
        PincelError::ConnectError(e)
    }
}

impl From<x11rb::protocol::Error> for PincelError {
    fn from(e: x11rb::protocol::Error) -> Self {
        Self::XlibError(e)
    }
}
