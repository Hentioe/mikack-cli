use super::VERSION;
use clap::{App, Arg};

const AUTHOR: &'static str = "Hentioe Cl (绅士喵)";

pub fn build_cli() -> App<'static, 'static> {
    App::new("manga")
        .version(VERSION)
        .about("An online manga(comic/album) export tool")
        .author(AUTHOR)
        .arg(
            Arg::with_name("url")
                .help("Online manga address")
                .takes_value(true)
                .required(false),
        )
        .arg(
            Arg::with_name("output-directory")
                .long("output")
                .short("O")
                .help("Specify output directory")
                .takes_value(true)
                .required(false),
        )
        .subcommand(
            App::new("clean")
                .version(VERSION)
                .about("Empty the cache (the structure directory of epub)")
                .author(AUTHOR),
        )
}
