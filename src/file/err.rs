use std::io;

#[deriving(Show)]
pub enum Reason {
  MagicNotFound,
  Io(io::IoError)
}

