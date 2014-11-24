use std::fmt;
use std::io::MemReader;
use super::{EncryptedFilename, Flags};

pub struct EncryptedMetadata {
  raw: MemReader,
  iv: Vec<u8>,
  key: Option<Vec<u8>>,
  flags: Flags,
  filename: Option<EncryptedFilename>,
  decrypted: Option<Metadata>
}

pub struct Metadata {
  raw: Vec<u8>
}

pub enum Error {
  GarbledOutput,
  StructFitFail
}

impl EncryptedMetadata {
  pub fn new(m: MemReader, iv: Vec<u8>) -> EncryptedMetadata {
    EncryptedMetadata {
      raw: m,
      iv: iv,
      key: None,
      flags: Flags { bits: 0 },
      filename: None,
      decrypted: None
    }
  }

  pub fn set_flags(&mut self, fl: Flags) -> Flags {
    use std::mem::replace;
    replace(&mut self.flags, fl)
  }

  pub fn set_filename(&mut self, fln: EncryptedFilename) {
    use std::mem::replace;
    replace(&mut self.filename, Some(fln));
  }

  pub fn decrypt(&mut self, k: Vec<u8>) -> Result<&mut Metadata,Error> {
    Err(Error::GarbledOutput)
  }
}

impl fmt::Show for EncryptedMetadata {
  fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
    self.raw.get_ref().fmt(f)
  }
}
