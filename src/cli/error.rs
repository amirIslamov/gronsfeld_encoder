use std::io::{self, Error};
use crate::decode::error::DecoderCreationError;
use crate::encode::error::EncoderCreationError;


pub enum EncodingError {
    IoError(io::Error),
    EncoderCreationError(EncoderCreationError),
}

pub enum DecodingError {
    IoError(io::Error),
    DecoderCreationError(DecoderCreationError),
}

impl From<io::Error> for EncodingError {
    fn from(e: Error) -> Self {
        EncodingError::IoError(e)
    }
}

impl From<EncoderCreationError> for EncodingError {
    fn from(e: EncoderCreationError) -> Self {
        EncodingError::EncoderCreationError(e)
    }
}

impl From<io::Error> for DecodingError {
    fn from(e: Error) -> Self {
        DecodingError::IoError(e)
    }
}

impl From<DecoderCreationError> for DecodingError {
    fn from(e: DecoderCreationError) -> Self {
        DecodingError::DecoderCreationError(e)
    }
}
