#[macro_use]
extern crate clap;
mod artifact;

use clap::{App, AppSettings, Arg, SubCommand};

fn main() {
    let app = App::new("buff")
        .about("Protobuf version manager - The easiest way to explore and use protobuffs and GRPC")
        .version(crate_version!())
        .author(crate_authors!())
        .setting(AppSettings::ArgRequiredElseHelp)
        .subcommand(login_sub_command());

    let matches = app.get_matches();
    match matches.subcommand_name() {
        Some("login") => println!("loginnnn",), // login
        _ => (),                                // Either no subcommand or one not tested for...
    };
}

fn login_sub_command<'a, 'b>() -> clap::App<'a, 'b> {
    return SubCommand::with_name("login").about("Login to buff").arg(
        Arg::with_name("username")
            .short("u")
            .help("Case sensative username"),
    );
}