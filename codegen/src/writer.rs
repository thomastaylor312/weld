use crate::BytesMut;

#[derive(Default)]
pub struct Writer {
    writer: BytesMut,
}

impl Writer {
    #[inline]
    pub fn write<B: ToBytes>(&mut self, bytes: B) {
        self.writer.extend_from_slice(bytes.to_bytes());
    }

    /// Returns the current buffer, zeroing out self
    pub fn take(&mut self) -> BytesMut {
        self.writer.split_to(self.writer.len())
    }
}

pub trait ToBytes {
    fn to_bytes(&self) -> &[u8];
}
impl ToBytes for &str {
    fn to_bytes(&self) -> &[u8] {
        self.as_bytes()
    }
}
impl ToBytes for &String {
    fn to_bytes(&self) -> &[u8] {
        self.as_bytes()
    }
}
impl ToBytes for &[u8] {
    fn to_bytes(&self) -> &[u8] {
        self
    }
}
impl ToBytes for BytesMut {
    fn to_bytes(&self) -> &[u8] {
        self.as_ref()
    }
}

impl<const N: usize> ToBytes for &[u8; N] {
    fn to_bytes(&self) -> &[u8] {
        *self
    }
}
