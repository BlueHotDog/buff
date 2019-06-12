#[macro_use]
extern crate clap;
extern crate directories;


use clap::{App, AppSettings, Arg, SubCommand};
use directories::{BaseDirs, ProjectDirs, UserDirs};
use std::fs;
use std::io;

fn main()-> Result<(), std::option::NoneError> {
    let proj_dirs = ProjectDirs::from("com", "", env!("CARGO_PKG_NAME")).unwrap();
    let b = proj_dirs.config_dir().to_str()?;
    // let a = fs::create_dir()?;


    let app = App::new(env!("CARGO_PKG_NAME"))
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
    Ok(())
}


fn login_sub_command<'a, 'b>() -> clap::App<'a, 'b> {
    return SubCommand::with_name("login").about("Login to buff").arg(
        Arg::with_name("username")
            .short("u")
            .help("Case sensative username"),
    );
}
