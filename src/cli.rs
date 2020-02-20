use crate::VERSION;
use clap::{App, Arg};

const AUTHOR: &'static str = "Hentioe (绅士喵), <me@bluerain.io>";

pub fn build_cli<'a, 'b>() -> App<'a, 'b> {
    App::new("mikack-cli")
        .version(VERSION)
        .about("A tool for exporting online comics")
        .author(AUTHOR)
        .arg(
            Arg::with_name("url")
                .help("The address of the comic home page or reading page")
                .takes_value(true)
                .required(false),
        )
        .arg(
            Arg::with_name("save-format")
                .long("format")
                .short("f")
                .help("Saved format (eg: epub)")
                .takes_value(true)
                .required(false),
        )
}
