#[cfg(feature = "std")]
use thiserror::Error;

#[cfg(feature = "std")]
#[derive(Error, Debug)]
pub enum EncodingError {
    #[error("Invalid Decaf377 encoding")]
    InvalidEncoding,
    #[error("Invalid length bytes in encoded point")]
    InvalidSliceLength,
}

#[cfg(not(feature = "std"))]
#[derive(Debug)]
pub enum EncodingError {
    InvalidEncoding,
    InvalidSliceLength,
}
