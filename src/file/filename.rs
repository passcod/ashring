use std::fmt;

pub struct EncryptedFilename {
  raw: Path,
  iv: Vec<u8>,
  key: Option<Vec<u8>>,
  decrypted: Option<Vec<u8>>
}

impl EncryptedFilename {
  /// Initialise a filename, in encrypted state.
  pub fn new(p: Path, iv: Vec<u8>) -> EncryptedFilename {
    EncryptedFilename {
      raw: p,
      iv: iv,
      key: None,
      decrypted: None
    }
  }
}

impl fmt::Show for EncryptedFilename {
  fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
    self.raw.to_c_str().fmt(f)
  }
}
