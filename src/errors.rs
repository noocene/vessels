use std::fmt;

use failure::{Backtrace, Context, Fail};

#[derive(Debug)]
pub struct Error {
    ctx: Context<ErrorKind>,
}

impl Error {
    pub fn kind(&self) -> &ErrorKind {
        self.ctx.get_context()
    }

    pub(crate) fn color_stop() -> Error {
        Error::from(ErrorKind::ColorStopOffsetError)
    }

    pub(crate) fn address_in_use() -> Error {
        Error::from(ErrorKind::SocketError(SocketError::AddressInUse))
    }

    pub(crate) fn connection_failed() -> Error {
        Error::from(ErrorKind::SocketError(SocketError::ConnectionFailed))
    }

    pub(crate) fn feature_unavailable() -> Error {
        Error::from(ErrorKind::UnavailableFeatureError)
    }

    pub(crate) fn offer_generation_failed() -> Error {
        Error::from(ErrorKind::PeerError(PeerError::OfferGenerationFailed))
    }
}

impl Fail for Error {
    fn cause(&self) -> Option<&dyn Fail> {
        self.ctx.cause()
    }

    fn backtrace(&self) -> Option<&Backtrace> {
        self.ctx.backtrace()
    }
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        self.ctx.fmt(f)
    }
}

#[derive(Clone, Eq, PartialEq, Debug, Copy)]
pub enum PeerError {
    OfferGenerationFailed,
}

#[derive(Clone, Eq, PartialEq, Debug, Copy)]
pub enum SocketError {
    AddressInUse,
    ConnectionFailed,
}

#[derive(Clone, Eq, PartialEq, Debug, Copy)]
pub enum ErrorKind {
    SocketError(SocketError),
    PeerError(PeerError),
    ColorStopOffsetError,
    UnavailableFeatureError,

    #[doc(hidden)]
    __Nonexhaustive,
}

impl fmt::Display for ErrorKind {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match *self {
            ErrorKind::ColorStopOffsetError => write!(f, "Colorstop offset out of bounds"),
            ErrorKind::SocketError(error) => write!(f, "Socket error: {:?}", error),
            ErrorKind::PeerError(error) => write!(f, "Peer error: {:?}", error),
            ErrorKind::UnavailableFeatureError => {
                write!(f, "The requested feature is not available on this platform")
            }
            ErrorKind::__Nonexhaustive => panic!("Invalid Error!"),
        }
    }
}

impl From<ErrorKind> for Error {
    fn from(kind: ErrorKind) -> Error {
        Error::from(Context::new(kind))
    }
}

impl From<Context<ErrorKind>> for Error {
    fn from(ctx: Context<ErrorKind>) -> Error {
        Error { ctx }
    }
}
