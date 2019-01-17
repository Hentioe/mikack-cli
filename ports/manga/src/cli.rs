use clap::{App, Arg};

pub const VERSION: &'static str = env!("CARGO_PKG_VERSION");

pub fn build_cli() -> App<'static, 'static> {
    App::new("manga")
        .version(VERSION)
        .about("An online manga(comic/album) export tool")
        .author("Hentioe Cl (绅士喵)")
        .arg(
            Arg::with_name("url")
                .help("Online manga address")
                .required(false),
        )
        .subcommand(
            App::new("clean")
                .version(VERSION)
                .about("Empty the cache (the structure directory of epub)")
                .author("Hentioe Cl (绅士喵)"),
        )
}
