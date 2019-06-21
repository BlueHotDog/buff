use quicli::prelude::*;
use structopt::StructOpt;

mod artifact;
mod protobuffers;
mod package_metadata;
mod commands;

#[derive(StructOpt, Debug)]
#[structopt(name = "buff", about = "Buff registry CLI")]
enum Command {
  #[structopt(name = "login", about = "Login to the buff registry")]
  Login {
    #[structopt(short = "e", long = "email")]
    email: String,
    #[structopt(short = "p", long = "password")]
    password: String
  },
}

#[derive(StructOpt)]
struct Cli {
  #[structopt(subcommand)]
  cmd: Command,
  #[structopt(flatten)]
  verbosity: Verbosity,
}

fn main() -> CliResult {
  
  let args = Cli::from_args();
  args.verbosity.setup_env_logger("head")?;

  match args.cmd {
    Command::Login { email, password } => {
      commands::login::execute(&email, &password)
    },
    _ => (),
  }
  Ok(())
}