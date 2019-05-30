#[macro_use]
extern crate clap;

use clap::{Arg, App, SubCommand};

fn main() {
    let matches = App::new("Puff")
        .about("Like NPM but for Protobuffers")
        .version(crate_version!())
        .author(crate_authors!())
        .subcommand(loginSubCommand())
        .get_matches();
    let url = matches.value_of("URL").unwrap();
    println!("{}", url);
}


fn loginSubCommand() -> clap::App {
    return SubCommand::with_name("test")
        .about("Adds a registy user account")
        .arg(Arg::with_name("debug")
            .short("d")
            .help("print debug information verbosely"))
}