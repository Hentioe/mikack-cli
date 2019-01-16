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
    static ref RE_URL_DMZJ: Regex =
        Regex::new(r#"https://manhua.dmzj.com/[^/]+/\d+\.shtml"#).unwrap();
    static ref RE_URL_HHMH: Regex =
        Regex::new(r#"http://www.hhmmoo.com/page\d+/\d+\.html"#).unwrap();
}

fn main() -> Result<()> {
    env_logger::init();
    let matches = cli::build_cli().get_matches();
    let url = matches.value_of("url").unwrap();

    let sources: [(&Regex, &Fetcher, Platform); 2] = [
        (
            &RE_URL_DMZJ,
            &upstream::dmzj::Dmzj {} as &Fetcher,
            Platform::new("动漫之家", "https://manhua.dmzj.com"),
        ),
        (
            &RE_URL_HHMH,
            &upstream::hhmh::Hhmh {} as &Fetcher,
            Platform::new("汗汗漫画", "http://www.hhmmoo.com"),
        ),
    ];
    let mut supported = false;
    for (re, fr, p) in sources.iter() {
        if re.find(&url).is_none() {
            continue;
        } else {
            let mut section = Section::new(UNKNOWN_NAME, &url);

            println!(
                "{} {}Fetching pages...",
                style("[1/2]").bold().dim(),
                LOOKING_GLASS
            );
            fr.fetch_pages(&mut section)?;
            println!("[1/2] {} Done!", Emoji("✨", ":-)"));
            println!("{} {}Saving epub...", style("[2/2]").bold().dim(), TRUCK);
            let path = epub::Epub::new(p.clone(), section).save()?;
            println!("[2/2] {} Done!", Emoji("✨", ":-)"));
            println!("Succeed: {}", &path);
            supported = true;
            break;
        }
    }
    if !supported {
        return Err(err_msg("invalid or unsupported url"));
    }
    Ok(())
}
