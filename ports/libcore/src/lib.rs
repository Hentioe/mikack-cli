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
    path.push(fix_slash!(section_name));
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
macro_rules! fix_slash {
    ( $s:expr ) => {{
        $s.replace("/", "[slash]")
    }};
}

#[macro_export]
macro_rules! append_sources {
    ( $(( 'name $name:expr, 'homepage $homepage:expr, 'detail_regex $detail_regex:expr, 'section_regex $section_regex:expr, 'extractor $extractor:expr )),* ) => {
        {
            let mut section_sources: Vec<Source> = Vec::new();
            let mut detail_sources: Vec<Source> = Vec::new();
            $(
                let re_detail = build_regex($detail_regex);
                let re_section = build_regex($section_regex);
                let extractor = &$extractor as &(Extractor + Sync);
                let platform = Platform::new($name, $homepage);

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
    pub static ref MATCHES: (Vec<Source>, Vec<Source>) = append_sources![
        (

            'name "动漫之家",
            'homepage "https://manhua.dmzj.com",
            'detail_regex r#"https?://manhua\.dmzj\.com/[^/]+/$"#,
            'section_regex r#"^https?://manhua\.dmzj\.com/[^/]+/\d+\.shtml"#,
            'extractor extractors::Dmzj
        ),
        (
            'name "汗汗漫画",
            'homepage "http://www.hhmmoo.com",
            'detail_regex r#"https?://www\.hhmmoo\.com/manhua\d+\.html"#,
            'section_regex r#"^https?://www\.hhmmoo\.com/page\d+/\d+\.html"#,
            'extractor extractors::Hhmh
        ),
        (
            'name "動漫狂",
            'homepage "https://www.cartoonmad.com",
            'detail_regex r#"https?://www\.cartoonmad\.com/comic/\d{1,10}.html"#,
            'section_regex r#"^https?://www\.cartoonmad\.com/comic/\d{11,}\.html$"#,
            'extractor extractors::Dmk
        ),
        (
            'name "漫画柜",
            'homepage "https://www.manhuagui.com",
            'detail_regex r#"https?://www\.manhuagui\.com/comic/\d+/"#,
            'section_regex r#"https?://www\.manhuagui\.com/comic/\d+/\d+.html"#,
            'extractor extractors::Mhg
        ),
        (
            'name "非常爱漫",
            'homepage "http://www.verydm.com",
            'detail_regex r#"https?://www\.verydm\.com/manhua/[^/]+"#,
            'section_regex r#"https?://www\.verydm\.com/chapter\.php\?id=\d+"#,
            'extractor extractors::Fcam
        ),
        (
            'name "古风漫画网",
            'homepage "http://www.gufengmh.com",
            'detail_regex r#"https?://www\.gufengmh\.com/manhua/[^/]+/$"#,
            'section_regex r#"https?://www\.gufengmh\.com/manhua/.+/\d+\.html"#,
            'extractor extractors::Gfmh
        ),
        (
            'name "漫画台",
            'homepage "https://www.manhuatai.com",
            'detail_regex r#"https?://www\.manhuatai\.com/[^/]+/$"#,
            'section_regex r#"https?://www\.manhuatai\.com/[^/]+/\d+\.html"#,
            'extractor extractors::Mht
        ),
        (
            'name "漫画人",
            'homepage "http://www.manhuaren.com",
            'detail_regex r#"https?://www\.manhuaren\.com/[^/]+/$"#,
            'section_regex r#"https?://www\.manhuaren\.com/m\d+/"#,
            'extractor extractors::Mhr
        ),
        (
            'name "新新漫画网",
            'homepage "https://www.177mh.net",
            'detail_regex r#"https?://www\.177mh\.net/colist_\d+\.html"#,
            'section_regex r#"https?://www\.177mh\.net/\d+/\d+\.html"#,
            'extractor extractors::Xxmh
        ),
        (
            'name "E-Hentai",
            'homepage "https://e-hentai.org/",
            'detail_regex r#"https?://e-hentai\.org/uploader/.+"#,
            'section_regex r#"https?://e-hentai\.org/g/\d+/[^/]+/"#,
            'extractor extractors::Ehentai
        )
    ];
}
