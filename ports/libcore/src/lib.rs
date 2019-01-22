pub mod archive;
pub mod check;
pub mod errors;
pub mod export;
pub mod fetch;
pub mod jsrt;
pub mod progress;
pub mod storage;

pub const BASE_RES_DIR: &'static str = "manga_res";
pub const CACHE_DIR_NAME: &'static str = ".cache";
pub const ORIGIN_DIR_NAME: &'static str = "origins";
pub const DEFAULT_OUTPUT_DIR: &'static str = "manga_res/outputs";

use fetch::{prelude::*, *};
use lazy_static::lazy_static;
use regex::Regex;
use std::path::PathBuf;

pub const VERSION: &'static str = env!("CARGO_PKG_VERSION");

pub fn get_cache_path(section_name: &str) -> errors::Result<String> {
    let mut path = PathBuf::from(BASE_RES_DIR);
    path.push(section_name);
    path.push(CACHE_DIR_NAME);

    Ok(path
        .to_str()
        .ok_or(errors::err_msg(format!(
            "error getting cache directory for {}",
            section_name
        )))?
        .to_string())
}

pub fn get_origin_path(section_name: &str) -> errors::Result<String> {
    let mut path = PathBuf::from(BASE_RES_DIR);
    path.push(section_name);
    path.push(ORIGIN_DIR_NAME);

    Ok(path
        .to_str()
        .ok_or(errors::err_msg(format!(
            "error getting origin directory for {}",
            section_name
        )))?
        .to_string())
}

fn build_regex(expr: &str) -> Regex {
    Regex::new(expr).unwrap()
}

lazy_static! { // Platform list
    static ref DMZJ: Platform = Platform::new("动漫之家", "https://manhua.dmzj.com");
    static ref HHMH: Platform = Platform::new("汗汗漫画", "http://www.hhmmoo.com");
    static ref DMK: Platform = Platform::new("動漫狂", "https://www.cartoonmad.com");
    static ref MHG: Platform = Platform::new("漫画柜", "https://www.manhuagui.com");
}

lazy_static! { // Detail url matches
    static ref RE_DETAIL_DMZJ: Regex = build_regex(r#"https?://manhua\.dmzj\.com/[^/]+/$"#);
    static ref RE_DETAIL_HHMH: Regex = build_regex(r#"https?://www\.hhmmoo\.com/manhua\d+\.html"#);
    static ref RE_DETAIL_DMK: Regex = build_regex(r#"https?://www\.cartoonmad\.com/comic/\d{1,10}.html"#);
    static ref RE_DETAIL_MHG: Regex = build_regex(r#"https?://www\.manhuagui\.com/comic/\d+/"#);
}

lazy_static! { // Section url matches
    static ref RE_SECTION_DMZJ: Regex = build_regex(r#"^https?://manhua\.dmzj\.com/[^/]+/\d+\.shtml"#);
    static ref RE_SECTION_HHMH: Regex = build_regex(r#"^https?://www\.hhmmoo\.com/page\d+/\d+\.html$"#);
    static ref RE_SECTION_DMK: Regex = build_regex(r#"^https?://www\.cartoonmad\.com/comic/\d{11,}\.html$"#);
    static ref RE_SECTION_MHG: Regex = build_regex(r#"https?://www\.manhuagui\.com/comic/\d+/\d+.html"#);
}

lazy_static! { // Extractor list
    static ref EXTRACTOR_DMZJ: &'static (Extractor + Sync) = &extractors::Dmzj {} as &(Extractor + Sync);
    static ref EXTRACTOR_HHMH: &'static (Extractor + Sync) = &extractors::Hhmh {} as &(Extractor + Sync);
    static ref EXTRACTOR_DMK: &'static (Extractor + Sync) = &extractors::Dmk {} as &(Extractor + Sync);
    static ref EXTRACTOR_MHG: &'static (Extractor + Sync) = &extractors::Mhg {} as &(Extractor + Sync);
}

lazy_static! { // Matches
    pub static ref SECTION_MATCHES: Vec<(&'static Regex, &'static (Extractor + Sync), Platform)> = vec![
        (&RE_SECTION_DMZJ, *EXTRACTOR_DMZJ, DMZJ.clone()),
        (&RE_SECTION_HHMH, *EXTRACTOR_HHMH, HHMH.clone()),
        (&RE_SECTION_DMK, *EXTRACTOR_DMK, DMK.clone()),
        (&RE_SECTION_MHG, *EXTRACTOR_MHG, MHG.clone()),
    ];
    pub static ref DETAIL_MATCHES: Vec<(&'static Regex, &'static (Extractor + Sync), Platform)> = vec![
        (&RE_DETAIL_DMZJ, *EXTRACTOR_DMZJ, DMZJ.clone()),
        (&RE_DETAIL_HHMH, *EXTRACTOR_HHMH, HHMH.clone()),
        (&RE_DETAIL_DMK, *EXTRACTOR_DMK, DMK.clone()),
        (&RE_DETAIL_MHG, *EXTRACTOR_MHG, MHG.clone()),
    ];
}
