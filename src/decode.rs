pub(crate) mod error;
pub(crate) mod iter;
use crate::decode::error::DecoderCreationError;

pub struct GronsfeldDecoded<'a, 'b> {
    buf: &'a [u8],
    key: &'b [u8],
}

impl<'a, 'b> GronsfeldDecoded<'a, 'b> {
    pub fn new(buf: &'a [u8], key: &'b [u8]) -> Result<Self, DecoderCreationError> {
        if key.len() == 0 { return Err(DecoderCreationError::EmptyKey) }

        Ok(GronsfeldDecoded { buf, key })
    }

    pub fn get(&self, index: usize) -> Option<u8> {
        self.buf
            .get(index)
            .map(|i| (i - self.key[index % self.key.len()]))
    }

    pub fn len(&self) -> usize {
        self.buf.len()
    }
}