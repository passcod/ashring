#![feature(macro_rules)]
#![feature(phase)]
#[phase(plugin, link)] extern crate log;
extern crate "rust-crypto" as crypto;
extern crate fuse;
extern crate msgpack;
extern crate openssl;
extern crate xxhash;

use std::os;
use std::io::File;

mod file;

fn main () {
  let originpath = Path::new(os::args()[1].as_slice());
  let res = File::open(&originpath);
  match res {
    Err(e) => println!("{}", e),
    Ok(mut file) => {
      match file::EncryptedFile::from_file(&mut file) {
        Err(e) => println!("{}", e),
        Ok(n) => {
          debug!("Payload: {}", n.data);
        }
      }
    }
  }
}
