extern crate ring;

use std::error::Error;
use std::fs::File;
use std::io::Read;
use std::path::PathBuf;
use std::str;
use hex;


arg_enum! {
  #[derive(Debug)]
  pub enum Algorithm {
    SHA1,
    SHA256,
    SHA384,
    SHA512,
    SHA512_256
  }
}

#[derive(StructOpt, Debug)]
pub struct Digest {
  #[structopt(short = "a",
      long = "algo",
      help = "Algorithm used to compute a digest of your file.",
      raw(
        possible_values = "&Algorithm::variants()",
        case_insensitive = "true"),
      default_value = "SHA256")]
    algorithm: Algorithm,

    #[structopt(short = "f", 
      long = "file",
      help = "File on which to perform the digest.",
      parse(from_os_str))]
    file: PathBuf,


    #[structopt(short = "d",
      long = "digest",
      help = "Expected output of the digest for verification encoded as hex. If not provided the computed digest will be printed instead.")]
    expected_digest_hex: Option<String>
}

pub fn run_digest(digest: &Digest) -> Result<String, String> {
  fn encode_digest_to_string(digest: ring::digest::Digest) -> String {
      let buf = hex::encode_hex_buffer(digest.as_ref()).unwrap();
      str::from_utf8(&buf).unwrap().to_string()
    }

  let file_path = digest.file.as_path();
  let digest_alg = match digest.algorithm {
    Algorithm::SHA1 => &ring::digest::SHA1,
    Algorithm::SHA256 => &ring::digest::SHA256,
    Algorithm::SHA384 => &ring::digest::SHA384,
    Algorithm::SHA512 => &ring::digest::SHA512,
    Algorithm::SHA512_256 => &ring::digest::SHA512_256
  };

  let mut ctx = ring::digest::Context::new(digest_alg);

  {
    let mut file = match File::open(file_path) {
      Ok(file) => file,
      // TODO: don't use panic here.
      Err(why) => panic!("Couldn't open {}: {}", file_path.display(),
                         why.description())
    };

    let mut chunk = vec![0u8; 128 * 1024];
    loop {
      match file.read(&mut chunk[..]) {
        Ok(0) => break,
        Ok(bytes_read) => ctx.update(&chunk[0..bytes_read]),
        // TODO: don't use panic here
        Err(why) => panic!("Couldn't open {}: {}", file_path.display(),
                       why.description())
      }
    }
  }

  let actual_digest = ctx.finish();

  if let Some(ref expected_digest_hex) = digest.expected_digest_hex {
    let matched = match hex::decode_hex_buffer(expected_digest_hex.as_bytes()) {
      Ok(expected) => actual_digest.as_ref() == &expected[..],
      Err(msg) => panic!("Syntactically invalid digest: {} in {}", msg,
                         &expected_digest_hex),
    };

    match matched {
      true => Ok(encode_digest_to_string(actual_digest)),
      false => Err(format!("Digest mismatch, expected {} but calculated {}", expected_digest_hex, encode_digest_to_string(actual_digest)))
    }
  } else {
    Ok(encode_digest_to_string(actual_digest))
  }
}

