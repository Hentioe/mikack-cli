#![feature(fn_traits)]

pub mod checker;
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

type Source = (Regex, &'static (Extractor + Sync), Platform<'static>);

#[macro_export]
macro_rules! append_source {
    ( $( $source:expr ),* ) => {
        {
            let mut section_sources: Vec<Source> = Vec::new();
            let mut detail_sources: Vec<Source> = Vec::new();
            $(
                let re_detail = build_regex($source.0);
                let re_section = build_regex($source.1);
                let extractor = &$source.2 as &(Extractor + Sync);
                let platform = Platform::new($source.3, $source.4);

                detail_sources.push((re_detail, extractor, platform));
                section_sources.push((re_section, extractor, platform));
            )*
            (detail_sources, section_sources)
        }
    };
}

fn build_regex(expr: &str) -> Regex {
    Regex::new(expr).unwrap()
}

lazy_static! { // Source list
    pub static ref MATCHES: (Vec<Source>, Vec<Source>) = append_source![
        (
            r#"https?://manhua\.dmzj\.com/[^/]+/$"#,
            r#"^https?://manhua\.dmzj\.com/[^/]+/\d+\.shtml"#,
            extractors::Dmzj,
            "动漫之家", "https://manhua.dmzj.com"
        ),
        (
            r#"https?://www\.hhmmoo\.com/manhua\d+\.html"#,
            r#"^https?://www\.hhmmoo\.com/page\d+/\d+\.html"#,
            extractors::Hhmh,
            "汗汗漫画", "http://www.hhmmoo.com"
        ),
        (
            r#"https?://www\.cartoonmad\.com/comic/\d{1,10}.html"#,
            r#"^https?://www\.cartoonmad\.com/comic/\d{11,}\.html$"#,
            extractors::Dmk,
            "動漫狂", "https://www.cartoonmad.com"
        ),
        (
            r#"https?://www\.manhuagui\.com/comic/\d+/"#,
            r#"https?://www\.manhuagui\.com/comic/\d+/\d+.html"#,
            extractors::Mhg,
            "漫画柜", "https://www.manhuagui.com"
        ),
        (
            r#"https?://www\.verydm\.com/manhua/[^/]+"#,
            r#"https?://www\.verydm\.com/chapter\.php\?id=\d+"#,
            extractors::Fcam,
            "非常爱漫", "http://www.verydm.com"
        ),
        (
            r#"https?://www\.gufengmh\.com/manhua/[^/]+/$"#,
            r#"https?://www\.gufengmh\.com/manhua/.+/\d+\.html"#,
            extractors::Gfmh,
            "古风漫画网", "http://www.gufengmh.com"
        ),
        (
            r#"https?://www\.manhuatai\.com/[^/]+/$"#,
            r#"https?://www\.manhuatai\.com/[^/]+/\d+\.html"#,
            extractors::Mht,
            "漫画台", "https://www.manhuatai.com"
        ),
        (
            r#"https?://www\.manhuaren\.com/[^/]+/$"#,
            r#"https?://www\.manhuaren\.com/m\d+/"#,
            extractors::Mhr,
            "漫画人", "http://www.manhuaren.com"
        ),
        (
            r#"https?://www\.177mh\.net/colist_\d+\.html"#,
            r#"https?://www\.177mh\.net/\d+/\d+\.html"#,
            extractors::Xxmh,
            "新新漫画网", "https://www.177mh.net"
        ),
        (
            r#"https?://e-hentai\.org/uploader/.+"#,
            r#"https?://e-hentai\.org/g/\d+/[^/]+/"#,
            extractors::Ehentai,
            "E-Hentai", "https://e-hentai.org/"
        )
    ];
}
