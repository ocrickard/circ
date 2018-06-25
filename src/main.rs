
extern crate ring;
#[macro_use]
extern crate structopt;
#[macro_use]
extern crate clap;

use structopt::StructOpt;
use std::io::Write;

mod hex;
mod digest;

#[derive(StructOpt, Debug)]
#[structopt(name = "circ")]
struct Opt {
  #[structopt(short = "d", long = "debug")]
  debug: bool,

  #[structopt(subcommand)]
  command: Command
}

#[derive(StructOpt, Debug)]
pub enum Command {

  #[structopt(name = "digest", about = "Perform a digest check of a file.")]
  Digest(digest::Digest)
}

fn main() {
  let matches = Opt::from_args();

  match matches.command {
    Command::Digest(digest) => {
      match digest::run_digest(&digest) {
        Ok(x) => {
          let _ = writeln!(&mut std::io::stdout(), "{}", x);
        },
        Err(s) => {
          let _ = writeln!(&mut std::io::stderr(), "{}", s);
          std::process::exit(1)
        }
      }
    }
  }
}
