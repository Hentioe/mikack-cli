use console::{style, Emoji};
use lazy_static::lazy_static;
use libcore::{
    errors::*,
    export::{prelude::*, *},
    fetch::{prelude::*, *},
};
use regex::Regex;

mod cli;

static LOOKING_GLASS: Emoji = Emoji("🔍  ", "");
static TRUCK: Emoji = Emoji("🚚  ", "");

lazy_static! {
    static ref RE_URL: Regex = Regex::new(r#"https://manhua.dmzj.com/[^/]+/\d+\.shtml"#).unwrap();
}

///** 暂时仅支持 URL 识别模式，仅写死支持动漫之家漫画地址 **
fn main() -> Result<()> {
    env_logger::init();
    let matches = cli::build_cli().get_matches();
    let url = matches.value_of("url").unwrap();
    RE_URL
        .find(&url)
        .ok_or(err_msg("invalid or unsupported address"))?;
    let platform = Platform::new("动漫之家", "https://manhua.dmzj.com");
    let mut section = Section::new(UNKNOWN_NAME, &url);

    println!(
        "{} {}Fetching pages...",
        style("[1/2]").bold().dim(),
        LOOKING_GLASS
    );
    upstream::dmzj::Dmzj {}.fetch_pages(&mut section)?;
    println!("[1/2] {} Done!", Emoji("✨", ":-)"));
    println!("{} {}Saving epub...", style("[2/2]").bold().dim(), TRUCK);
    let path = epub::Epub::new(platform, section).save()?;
    println!("[2/2] {} Done!", Emoji("✨", ":-)"));
    println!("Succeed: {}", &path);
    Ok(())
}
