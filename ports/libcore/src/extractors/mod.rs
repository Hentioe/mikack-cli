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

struct FromLinkList<'a, T> {
    url: &'a str,
    selector: &'a str,
    list: Vec<T>,
    href_prefix: &'a str,
    text_prefix: &'a str,
}

impl<'a, T> FromLinkList<'a, T> {
    pub fn new(url: &'a str, selector: &'a str, list: Vec<T>) -> Self {
        Self {
            url,
            selector,
            list,
            href_prefix: "",
            text_prefix: "",
        }
    }

    pub fn set_href_prefix(&mut self, prefix: &'a str) -> &Self {
        self.href_prefix = prefix;
        self
    }

    pub fn set_text_prefix(&mut self, prefix: &'a str) -> &Self {
        self.text_prefix = prefix;
        self
    }

    pub fn result(self) -> Result<Vec<T>> {
        Ok(self.list)
    }
}
