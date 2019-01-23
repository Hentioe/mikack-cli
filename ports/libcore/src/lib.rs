#![feature(fn_traits)]

pub mod archive;
pub mod check;
pub mod errors;
pub mod exporters;
pub mod extractors;
pub mod html;
pub mod http;
pub mod jsrt;
pub mod models;
pub mod progress;
pub mod storage;

pub const BASE_RES_DIR: &'static str = "manga_res";
pub const CACHE_DIR_NAME: &'static str = ".cache";
pub const ORIGIN_DIR_NAME: &'static str = "origins";
pub const DEFAULT_OUTPUT_DIR: &'static str = "manga_res/outputs";

use extractors::prelude::*;
use lazy_static::lazy_static;
use models::*;
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
    static ref DMZJ: Platform<'static> = Platform::new("动漫之家", "https://manhua.dmzj.com");
    static ref HHMH: Platform<'static> = Platform::new("汗汗漫画", "http://www.hhmmoo.com");
    static ref DMK: Platform<'static> = Platform::new("動漫狂", "https://www.cartoonmad.com");
    static ref MHG: Platform<'static> = Platform::new("漫画柜", "https://www.manhuagui.com");
    static ref FCAM: Platform<'static> = Platform::new("非常爱漫", "http://www.verydm.com");
    static ref GFMH: Platform<'static> = Platform::new("古风漫画网", "http://www.gufengmh.com");
    static ref MHT: Platform<'static> = Platform::new("漫画台", "https://www.manhuatai.com");
    static ref MHR: Platform<'static> = Platform::new("漫画人", "http://www.manhuaren.com");
    static ref XXMH: Platform<'static> = Platform::new("新新漫画网", "https://www.177mh.net");
}

lazy_static! { // Detail url matches
    static ref RE_DETAIL_DMZJ: Regex = build_regex(r#"https?://manhua\.dmzj\.com/[^/]+/$"#);
    static ref RE_DETAIL_HHMH: Regex = build_regex(r#"https?://www\.hhmmoo\.com/manhua\d+\.html"#);
    static ref RE_DETAIL_DMK: Regex = build_regex(r#"https?://www\.cartoonmad\.com/comic/\d{1,10}.html"#);
    static ref RE_DETAIL_MHG: Regex = build_regex(r#"https?://www\.manhuagui\.com/comic/\d+/"#);
    static ref RE_DETAIL_FCAM: Regex = build_regex(r#"https?://www\.verydm\.com/manhua/[^/]+"#);
    static ref RE_DETAIL_GFMH: Regex = build_regex(r#"https?://www\.gufengmh\.com/manhua/[^/]+/$"#);
    static ref RE_DETAIL_MHT: Regex = build_regex(r#"https?://www\.manhuatai\.com/[^/]+/$"#);
    static ref RE_DETAIL_MHR: Regex = build_regex(r#"https?://www\.manhuaren\.com/[^/]+/$"#);
    static ref RE_DETAIL_XXMH: Regex = build_regex(r#"https?://www\.177mh\.net/colist_\d+\.html"#);
}

lazy_static! { // Section url matches
    static ref RE_SECTION_DMZJ: Regex = build_regex(r#"^https?://manhua\.dmzj\.com/[^/]+/\d+\.shtml"#);
    static ref RE_SECTION_HHMH: Regex = build_regex(r#"^https?://www\.hhmmoo\.com/page\d+/\d+\.html$"#);
    static ref RE_SECTION_DMK: Regex = build_regex(r#"^https?://www\.cartoonmad\.com/comic/\d{11,}\.html$"#);
    static ref RE_SECTION_MHG: Regex = build_regex(r#"https?://www\.manhuagui\.com/comic/\d+/\d+.html"#);
    static ref RE_SECTION_FCAM: Regex = build_regex(r#"https?://www\.verydm\.com/chapter\.php\?id=\d+"#);
    static ref RE_SECTION_GFMH: Regex = build_regex(r#"https?://www\.gufengmh\.com/manhua/.+/\d+\.html"#);
    static ref RE_SECTION_MHT: Regex = build_regex(r#"https?://www\.manhuatai\.com/[^/]+/\d+\.html"#);
    static ref RE_SECTION_MHR: Regex = build_regex(r#"https?://www\.manhuaren\.com/m\d+/"#);
    static ref RE_SECTION_XXMH: Regex = build_regex(r#"https?://www\.177mh\.net/\d+/\d+\.html"#);
}

lazy_static! { // Extractor list
    static ref EXTRACTOR_DMZJ: &'static (Extractor + Sync) = &extractors::Dmzj {} as &(Extractor + Sync);
    static ref EXTRACTOR_HHMH: &'static (Extractor + Sync) = &extractors::Hhmh {} as &(Extractor + Sync);
    static ref EXTRACTOR_DMK: &'static (Extractor + Sync) = &extractors::Dmk {} as &(Extractor + Sync);
    static ref EXTRACTOR_MHG: &'static (Extractor + Sync) = &extractors::Mhg {} as &(Extractor + Sync);
    static ref EXTRACTOR_FCAM: &'static (Extractor + Sync) = &extractors::Fcam {} as &(Extractor + Sync);
    static ref EXTRACTOR_GFMH: &'static (Extractor + Sync) = &extractors::Gfmh {} as &(Extractor + Sync);
    static ref EXTRACTOR_MHT: &'static (Extractor + Sync) = &extractors::Mht {} as &(Extractor + Sync);
    static ref EXTRACTOR_MHR: &'static (Extractor + Sync) = &extractors::Mhr {} as &(Extractor + Sync);
    static ref EXTRACTOR_XXMH: &'static (Extractor + Sync) = &extractors::Xxmh {} as &(Extractor + Sync);
}

lazy_static! { // Matches
    pub static ref SECTION_MATCHES: Vec<(&'static Regex, &'static (Extractor + Sync), Platform<'static>)> = vec![
        (&RE_SECTION_DMZJ, *EXTRACTOR_DMZJ, *DMZJ),
        (&RE_SECTION_HHMH, *EXTRACTOR_HHMH, *HHMH),
        (&RE_SECTION_DMK, *EXTRACTOR_DMK, *DMK),
        (&RE_SECTION_MHG, *EXTRACTOR_MHG, *MHG),
        (&RE_SECTION_FCAM, *EXTRACTOR_FCAM, *FCAM),
        (&RE_SECTION_GFMH, *EXTRACTOR_GFMH, *GFMH),
        (&RE_SECTION_MHT, *EXTRACTOR_MHT, *MHT),
        (&RE_SECTION_MHR, *EXTRACTOR_MHR, *MHR),
        (&RE_SECTION_XXMH, *EXTRACTOR_XXMH, *XXMH),
    ];
    pub static ref DETAIL_MATCHES: Vec<(&'static Regex, &'static (Extractor + Sync), Platform<'static>)> = vec![
        (&RE_DETAIL_DMZJ, *EXTRACTOR_DMZJ, *DMZJ),
        (&RE_DETAIL_HHMH, *EXTRACTOR_HHMH, *HHMH),
        (&RE_DETAIL_DMK, *EXTRACTOR_DMK, *DMK),
        (&RE_DETAIL_MHG, *EXTRACTOR_MHG, *MHG),
        (&RE_DETAIL_FCAM, *EXTRACTOR_FCAM, *FCAM),
        (&RE_DETAIL_GFMH, *EXTRACTOR_GFMH, *GFMH),
        (&RE_DETAIL_MHT, *EXTRACTOR_MHT, *MHT),
        (&RE_DETAIL_MHR, *EXTRACTOR_MHR, *MHR),
        (&RE_DETAIL_XXMH, *EXTRACTOR_XXMH, *XXMH),
    ];
}
