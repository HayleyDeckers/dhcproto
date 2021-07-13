use thiserror::Error;

/// An alias for results returned by functions of this crate
pub type DecodeResult<T> = Result<T, DecodeError>;

/// The error kind for errors that get returned in the crate
#[derive(Error, Clone, Debug)]
pub enum DecodeError {
    #[error("decoder ran out of bytes to read on byte {index}")]
    EndOfBuffer { index: usize },

    #[error("decoder checked_add failed")]
    AddOverflow,

    /// An error with an arbitrary message
    #[error("{0}")]
    Message(String),

    /// An error with an arbitrary message
    #[error("{0}")]
    Msg(&'static str),

    #[error("error converting from slice")]
    SliceError(#[from] std::array::TryFromSliceError),

    #[error("error getting null terminated string")]
    NulError(#[from] std::ffi::FromBytesWithNulError),

    #[error("error converting to UTF-8")]
    Utf8Error(#[from] std::str::Utf8Error),
}

/// The error kind for errors that get returned in the crate
#[derive(Error, Clone, Debug)]
pub enum EncodeError {
    #[error("encoder checked_add failed")]
    AddOverflow,
    /// An error with an arbitrary message
    #[error("{0}")]
    Message(String),
}

pub type EncodeResult<T> = Result<T, EncodeError>;
