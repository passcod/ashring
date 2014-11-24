#![allow(dead_code)]
use std::io::{File, IoError, IoResult, MemReader, MismatchedFileTypeForOperation};

pub use self::data::EncryptedData;
pub use self::filename::{EncryptedFilename, Filename};
pub use self::metadata::{EncryptedMetadata, Metadata};

pub mod data;
pub mod filename;
pub mod metadata;

const MAGIC: [u8, ..7] = [0x53, 0x68, 0x65, 0x72, 0x6d, 0x61, 0x6e];

bitflags! {
  #[deriving(Show)]
  flags Flags: u32 {
    const EXTENDED_FILENAME = 0x00000001
  }
}

pub struct EncryptedFile<'file> {
  pub version: u8,
  pub flags: Flags,
  pub metadata: EncryptedMetadata,
  pub data: EncryptedData<'file>
}

impl<'r> EncryptedFile<'r> {
  /// Loads a normal file and parses the pre- and plain- header.
  pub fn from_file(f: &'r mut File) -> IoResult<EncryptedFile> {
    debug!("Path: {}", f.path().to_c_str());

    let magic = try!(f.read_exact(7));
    if magic.as_slice() != &MAGIC {
      return Err(IoError {
        kind: MismatchedFileTypeForOperation,
        desc: "Magic not found",
        detail: None
      });
    } else {
      debug!("Magic number: ok");
    }

    let version = try!(f.read_byte());
    debug!("Version: {}", version);

    let header_size = try!(f.read_be_u64());
    debug!("Header Size: {}", header_size);

    let header_iv = try!(f.read_exact(16));
    debug!("Header IV: {}", header_iv);

    let filename_iv = try!(f.read_exact(16));
    debug!("Filename IV: {}", filename_iv);

    let payload_iv = try!(f.read_exact(16));
    debug!("Payload IV: {}", payload_iv);

    let flags = Flags {bits: try!(f.read_be_u32()) };
    debug!("Flags: {}", flags);

    let header = try!(f.read_exact(header_size as uint));
    debug!("Header: {}", header);
 
    let mut filename = EncryptedFilename::new(f.path().clone(), filename_iv);
    filename.set_flags(flags);

    let mut metadata = EncryptedMetadata::new(MemReader::new(header), header_iv);
    metadata.set_flags(flags);
    metadata.set_filename(filename);

    let mut data = EncryptedData::new(f, payload_iv);
    data.set_flags(flags);
    data.set_offset((68 + header_size) as i64);

    Ok(EncryptedFile{
      version: version,
      flags: flags,
      metadata: metadata,
      data: data
    })
  }
}
