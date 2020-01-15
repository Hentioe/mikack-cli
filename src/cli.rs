use crate::VERSION;
use clap::App;

const AUTHOR: &'static str = "Hentioe (绅士喵), <me@bluerain.io>";

pub fn build_cli<'a, 'b>() -> App<'a, 'b> {
    App::new("manga-cli")
        .version(VERSION)
        .about("A tool for exporting online comics")
        .author(AUTHOR)
}
