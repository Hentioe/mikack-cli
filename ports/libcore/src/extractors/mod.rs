pub mod dmk;
pub mod dmzj;
pub mod hhmh;
pub mod mhg;
pub mod prelude;

pub use dmk::*;
pub use dmzj::*;
pub use hhmh::*;
pub use mhg::*;

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

    pub fn result(self) -> Result<Vec<T>> {
        Ok(self.list)
    }
}
