use crate::VERSION;
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
                .short("o")
                .help("Specify output directory")
                .takes_value(true)
                .required(false),
        )
        .arg(
            Arg::with_name("output-format(s)")
                .long("format")
                .short("f")
                .help("Saved format (eg: zip,epub,mobi,azw3,pdf)")
                .takes_value(true)
                .required(true)
                .default_value("zip"),
        )
        .subcommand(
            App::new("clean")
                .version(VERSION)
                .about("Delete all .cache directories")
                .author(AUTHOR),
        )
}
