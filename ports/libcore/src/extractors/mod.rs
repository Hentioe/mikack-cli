pub mod dm5;
pub mod dmk;
pub mod dmzj;
pub mod ehentai;
pub mod fcam;
pub mod gfmh;
pub mod hhmh;
pub mod mhg;
pub mod mhr;
pub mod mht;
pub mod prelude;
pub mod xxmh;

pub use dm5::*;
pub use dmk::*;
pub use dmzj::*;
pub use ehentai::*;
pub use fcam::*;
pub use gfmh::*;
pub use hhmh::*;
pub use mhg::*;
pub use mhr::*;
pub use mht::*;
pub use xxmh::*;

use crate::errors::*;
use encoding_rs::Encoding;
use scraper::Html;
use std::option::Option;

struct LinkListConverter<'a, T> {
    url: &'a str,
    selector: &'a str,
    list: Vec<T>,
    href_prefix: &'a str,
    encoding: Option<&'static Encoding>,
    find_text_prefix: Option<&'a Fn(&Html) -> Result<String>>,
    text_in_dom: Option<String>,
}

impl<'a, T> LinkListConverter<'a, T> {
    pub fn new(url: &'a str, selector: &'a str, list: Vec<T>) -> Self {
        Self {
            url,
            selector,
            list,
            href_prefix: "",
            encoding: None,
            find_text_prefix: None,
            text_in_dom: None,
        }
    }

    pub fn set_href_prefix(&mut self, prefix: &'a str) -> &mut Self {
        self.href_prefix = prefix;
        self
    }

    pub fn set_encoding(&mut self, encoding: &'static Encoding) -> &mut Self {
        self.encoding = Some(encoding);
        self
    }

    pub fn text_prefix_finder(&mut self, finder: &'a Fn(&Html) -> Result<String>) -> &mut Self {
        self.find_text_prefix = Some(finder);
        self
    }

    pub fn set_text_in_dom(&mut self, selectors: &'a str) -> &mut Self {
        self.text_in_dom = Some(selectors.to_owned());
        self
    }

    pub fn result(self) -> Result<Vec<T>> {
        Ok(self.list)
    }
}
