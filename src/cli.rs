use crate::VERSION;
use clap::{App, Arg};

const AUTHOR: &'static str = "Hentioe (绅士喵), <me@bluerain.io>";

pub fn build_cli<'a, 'b>() -> App<'a, 'b> {
    App::new("manga-cli")
        .version(VERSION)
        .about("A tool for exporting online comics")
        .author(AUTHOR)
        .arg(
            Arg::with_name("url")
                .help("The address of the comic home page or reading page")
                .takes_value(true)
                .required(false),
        )
}
