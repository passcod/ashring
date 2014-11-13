use std::{fmt, mem};
use std::io::{File, IoResult, SeekSet};

pub struct EncryptedData<'file> {
  raw: &'file mut File,
  iv: Vec<u8>,
  key: Option<Vec<u8>>,

  /// Start of File — where the data starts in `raw`.
  sof: i64
}

impl<'file> EncryptedData<'file> {
  /// Initialise in encrypted state.
  pub fn new(f: &'file mut File, iv: Vec<u8>, sof: i64) -> EncryptedData {
    EncryptedData {
      raw: f,
      iv: iv,
      key: None,
      sof: sof
    }
  }

  /// Rewind to original position.
  ///
  /// This doesn't rewind to the beginning of the file, but rather to just
  /// before the first byte of the encrypted data within the file.
  pub fn rewind(&mut self) -> IoResult<()> {
    self.raw.seek(self.sof, SeekSet)
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
