
extern crate ring;
#[macro_use]
extern crate structopt;
#[macro_use]
extern crate clap;

use structopt::StructOpt;
use std::process::exit;

mod hex;
mod digest;

#[derive(StructOpt, Debug)]
#[structopt(name = "circ")]
struct Opt {
  #[structopt(
    short = "d", 
    long = "debug")]
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
      if !digest::run_digest_and_print(&digest) {
        exit(1)
      }
    }
  }
}
