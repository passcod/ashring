use std::fmt;
use std::io::MemReader;

pub struct EncryptedHeader {
  raw: MemReader,
  iv: Vec<u8>,
  key: Option<Vec<u8>>,
  decrypted: Option<Vec<u8>>
}

impl EncryptedHeader {
  /// Initialise a header, in encrypted state.
  pub fn new(m: MemReader, iv: Vec<u8>) -> EncryptedHeader {
    EncryptedHeader {
      raw: m,
      iv: iv,
      key: None,
      decrypted: None
    }
  }
}

impl fmt::Show for EncryptedHeader {
  fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
    self.raw.get_ref().fmt(f)
  }
}
