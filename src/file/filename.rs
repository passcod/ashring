use std::fmt;
use super::Flags;

pub struct EncryptedFilename {
  raw: Path,
  iv: Vec<u8>,
  key: Option<Vec<u8>>,
  flags: Flags,
  decrypted: Option<Vec<u8>>
}

pub struct Filename {
  raw: Path
}

impl EncryptedFilename {
  pub fn new(p: Path, iv: Vec<u8>) -> EncryptedFilename {
    EncryptedFilename {
      raw: p,
      iv: iv,
      key: None,
      flags: Flags { bits: 0 },
      decrypted: None
    }
  }
  
  pub fn set_flags(&mut self, fl: Flags) -> Flags {
    use std::mem::replace;
    replace(&mut self.flags, fl)
  }
}

impl fmt::Show for EncryptedFilename {
  fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
    self.raw.to_c_str().fmt(f)
  }
}
