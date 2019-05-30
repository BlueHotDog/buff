#[macro_use]
extern crate clap;

use clap::{App, Arg};

fn main() {
    let matches = App::new("Puff")
        .version(crate_version!())
        .author(crate_authors!())
        .about("wget clone written in Rust")
        .arg(
            Arg::with_name("URL")
                .required(true)
                .takes_value(true)
                .index(1)
                .help("url to download"),
        )
        .get_matches();
    let url = matches.value_of("URL").unwrap();
    println!("{}", url);
}