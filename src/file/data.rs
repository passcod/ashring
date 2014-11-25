use std::{fmt, mem};
use std::io::{File, IoResult, SeekSet};
use super::Flags;

pub struct EncryptedData<'file> {
  raw: &'file mut File,
  iv: Vec<u8>,
  key: Option<Vec<u8>>,
  flags: Flags,

  /// Offset — where the data starts in `raw`.
  offset: i64
}

impl<'file> EncryptedData<'file> {
  pub fn new(f: &'file mut File, iv: Vec<u8>) -> EncryptedData {
    EncryptedData {
      raw: f,
      iv: iv,
      key: None,
      flags: Flags { bits: 0 },
      offset: 0
    }
  }

  pub fn set_flags(&mut self, fl: Flags) -> Flags {
    use std::mem::replace;
    replace(&mut self.flags, fl)
  }

  pub fn set_offset(&mut self, off: i64) -> i64 {
    use std::mem::replace;
    replace(&mut self.offset, off)
  }

  /// Rewind to original position.
  ///
  /// This doesn't rewind to the beginning of the file, but rather to just
  /// before the first byte of the encrypted data within the file.
  pub fn rewind(&mut self) -> IoResult<()> {
    self.raw.seek(self.offset, SeekSet)
  }

  // BADCODE
  #[doc(hidden)]
  unsafe fn immut_read_to_end(&self) -> IoResult<Vec<u8>> {
    let muts: &mut EncryptedData = mem::transmute(self);
    match self.raw.tell() {
      Ok(cur) => {
        match muts.rewind() {
          Ok(_) => {
            let result = muts.raw.read_to_end();
            match muts.raw.seek(cur as i64, SeekSet) { _ => { } }
            result
          },
          Err(r) => Err(r)
        }
      },
      Err(r) => Err(r)
    }
  }
}

impl<'file> fmt::Show for EncryptedData<'file> {
  fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
    unsafe { self.immut_read_to_end() }.fmt(f)
  }
}
