extern crate ring;
extern crate termion;

use hex;
use std::error::Error;
use std::fs::File;
use std::io::{Read, Write, stdout, stderr};
use std::path::PathBuf;
use std::str;
use self::termion::{color, style};

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
      Err(msg) => {
          return Err(format!(
          "Couldn't open {red}{file_path}{reset}:\n\t{red}{error_msg}{reset}",
            file_path = file_path.display(),
            error_msg = msg.description(),
            reset = style::Reset,
            red = color::Fg(color::Red)));
        }
    };

    let mut chunk = vec![0u8; 128 * 1024];
    loop {
      match file.read(&mut chunk[..]) {
        Ok(0) => break,
        Ok(bytes_read) => ctx.update(&chunk[0..bytes_read]),
        Err(msg) => {
          return Err(format!(
          "Couldn't open {red}{file_path}{reset}:\n\t{red}{error_msg}{reset}",
            file_path = file_path.display(),
            error_msg = msg.description(),
            reset = style::Reset,
            red = color::Fg(color::Red)));
        }
      }
    }
  }

  let actual_digest = ctx.finish();

  if let Some(ref expected_digest_hex) = digest.expected_digest_hex {
    let matched = match hex::decode_hex_buffer(expected_digest_hex.as_bytes()) {
      Ok(expected) => actual_digest.as_ref() == &expected[..],
      Err(msg) => {
        return Err(format!(
          "Syntactically invalid digest: \n\t{error_msg} \nin supposedly hex-encoded input: \n\t{red}{expected_digest_hex}{reset}",
            error_msg = msg,
            expected_digest_hex = &expected_digest_hex,
            reset = style::Reset,
            red = color::Fg(color::Red)))
      },
    };

    match matched {
      true => Ok(format!(
        "{green}Digest match{reset}. calculated:\n\t{green}{actual_digest}{reset}",
          actual_digest = encode_digest_to_string(actual_digest),
          green = color::Fg(color::Green),
          reset = style::Reset)),
      false => Err(format!(
        "{bold}Digest mismatch{reset}! expected:\n\t{green}{expected}{reset}\nbut calculated:\n\t{red}{actual}{reset}\nYour data may be {yellow}corrupt{reset} or may have been {red}maliciously altered{reset}. {bold}Beware.{reset}",
          actual = encode_digest_to_string(actual_digest),
          expected = expected_digest_hex,
          bold = style::Bold,
          reset = style::Reset,
          red = color::Fg(color::Red),
          green = color::Fg(color::Green),
          yellow = color::Fg(color::Yellow)))
    }
  } else {
    Ok(format!(
      "{green}Successfully computed{reset}:\n\t{actual_digest}",
        actual_digest = encode_digest_to_string(actual_digest),
        green = color::Fg(color::Green),
        reset = style::Reset
      ))
  }
}

pub fn run_digest_and_print(digest: &Digest) -> bool {
  match run_digest(&digest) {
    Ok(x) => {
      let _ = writeln!(&mut stdout(), "{}", x);
      true
    },
    Err(s) => {
      let _ = writeln!(&mut stderr(), "{}", s);
      false
    }
  }
}

