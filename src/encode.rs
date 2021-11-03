use crate::encode::error::EncoderCreationError;

pub(crate) mod error;
pub(crate) mod iter;

pub struct GronsfeldEncoded<'a, 'b> {
    buf: &'a [u8],
    key: &'b [u8],
}

impl<'a, 'b> GronsfeldEncoded<'a, 'b> {
    pub fn new(buf: &'a [u8], key: &'b [u8]) -> Result<Self, EncoderCreationError> {
        if key.len() == 0 { return Err(EncoderCreationError::EmptyKey) }

        Ok(GronsfeldEncoded { buf, key })
    }

    pub fn get(&self, index: usize) -> Option<u8> {
        self.buf
            .get(index)
            .map(|i| (i + self.key[index % self.key.len()]))
    }

    pub fn len(&self) -> usize {
        self.buf.len()
    }
}


