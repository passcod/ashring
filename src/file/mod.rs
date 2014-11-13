#![allow(dead_code)]
use std::io::{File, MemReader};

pub use self::header::{EncryptedHeader, PlainHeader};
pub use self::filename::EncryptedFilename;
pub use self::data::EncryptedData;

pub mod err;
pub mod filename;
pub mod header;
pub mod data;

macro_rules! trunc(
  ($act:expr, $r:ident: $res:block) =>
  (match $act {
    Err(e) => return Err(err::Io(e)),
    Ok($r) => $res
  })
)

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
  pub filename: EncryptedFilename,
  pub header: EncryptedHeader,
  pub data: EncryptedData<'file>
}

impl<'r> EncryptedFile<'r> {
  /// Loads a normal file and parses the pre- and plain- header.
  pub fn from_file(f: &'r mut File) -> Result<EncryptedFile, err::Reason> {
    debug!("Path: {}", f.path().to_c_str());

    trunc!(f.read_exact(7), magic: {
      if magic.as_slice() != MAGIC {
        return Err(err::MagicNotFound);
      } else {
        debug!("Magic number: ok");
      }
    });

    let version: u8;
    trunc!(f.read_byte(), b: { version = b });
    debug!("Version: {}", version);

    let header_size: u64;
    trunc!(f.read_be_u64(), b: { header_size = b });
    debug!("Header Size: {}", header_size);

    let header_iv: Vec<u8>;
    trunc!(f.read_exact(16), b: { header_iv = b });
    debug!("Header IV: {}", header_iv);

    let filename_iv: Vec<u8>;
    trunc!(f.read_exact(16), b: { filename_iv = b });
    debug!("Filename IV: {}", filename_iv);

    let payload_iv: Vec<u8>;
    trunc!(f.read_exact(16), b: { payload_iv = b });
    debug!("Payload IV: {}", payload_iv);

    let flags: Flags;
    trunc!(f.read_be_u32(), b: { flags = Flags { bits: b } });
    debug!("Flags: {}", flags);

    let header: Vec<u8>;
    trunc!(f.read_exact(header_size as uint), b: { header = b });
    debug!("Header: {}", header);
 
    Ok(EncryptedFile{
      version: version,
      flags: flags,
      filename: EncryptedFilename::new(f.path().clone(), filename_iv), //FIXME: needs support for extended filenames
      header: EncryptedHeader::new(MemReader::new(header), header_iv),
      data: EncryptedData::new(f, payload_iv, (68 + header_size) as i64)
    })
  }
}
