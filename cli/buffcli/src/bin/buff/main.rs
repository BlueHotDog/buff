use quicli::prelude::*;
use structopt::StructOpt;

mod commands;

#[derive(StructOpt, Debug)]
#[structopt(name = "buff", about = "Buff CLI")]
enum Command {
  #[structopt(name = "login", about = "Login to the buff registry")]
  Login {
    #[structopt(short = "e", long = "email")]
    email: String,
    #[structopt(short = "p", long = "password")]
    password: String,
  },
  #[structopt(
    name = "publish",
    about = "Publishes the package as configured in buff.toml"
  )]
  Publish {},
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
  args.verbosity.setup_env_logger(&env!("CARGO_PKG_NAME"))?;
  info!("asfasf");

  match args.cmd {
    Command::Login { email, password } => commands::login::execute(&email, &password),
    Command::Publish {} => commands::publish::execute(),
  }
  Ok(())
}