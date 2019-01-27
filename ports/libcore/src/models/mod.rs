use crate::fix_slash;
use serde_derive::{Deserialize, Serialize};

// 漫画（图片集）平台
#[derive(Debug, Clone, Copy)]
pub struct Platform<'a> {
    pub name: &'a str,
    pub url: &'a str,
}

// 漫画详情（例：火影忍者）
#[derive(Debug, Clone)]
pub struct Detail {
    pub name: String,
    pub url: String,
    pub section_list: Vec<Section>,
}

// 漫画章节（例：第一话）
#[derive(Debug, Clone)]
pub struct Section {
    pub name: String,
    pub url: String,
    pub page_list: Vec<Page>,
}

// 漫画单页（一张图）
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct Page {
    pub p: u32,
    pub url: String,
    pub mime: String,
    pub extension: String,
}

impl Page {
    pub fn file_name(&self) -> String {
        format!("{}.{}", self.p, self.extension)
    }
}

pub const UNKNOWN_NAME: &'static str = "[UNNAMED]";

impl<'a> Platform<'a> {
    pub fn new(name: &'a str, url: &'a str) -> Self {
        Platform { name, url }
    }
}

impl Detail {
    pub fn new(name: &str, url: &str) -> Self {
        Detail {
            name: name.to_owned(),
            url: url.to_owned(),
            section_list: vec![],
        }
    }

    pub fn reset_section_list(&mut self, section_list: Vec<Section>) -> &Self {
        self.section_list = section_list;
        self
    }

    pub fn add_section(&mut self, section: Section) -> &Self {
        self.section_list.push(section);
        self
    }

    pub fn reverse_section_list(&mut self) {
        self.section_list.reverse();
    }
}

impl Section {
    pub fn new(name: &str, url: &str) -> Self {
        Section {
            name: name.to_owned(),
            url: url.to_owned(),
            page_list: vec![],
        }
    }

    pub fn fix_slash_name(&self) -> String {
        fix_slash!(self.name)
    }

    pub fn reset_page_list(&mut self, page_list: Vec<Page>) -> &Self {
        self.page_list = page_list;
        self
    }

    pub fn add_page(&mut self, page: Page) -> &Self {
        self.page_list.push(page);
        self
    }

    pub fn has_page(&self) -> bool {
        self.page_list.len() > 0
    }

    pub fn has_name(&self) -> bool {
        !(self.name == UNKNOWN_NAME)
    }
}

impl Page {
    pub fn new(p: u32, url: &str) -> Self {
        Page {
            p,
            url: url.to_owned(),
            mime: "image/*".to_owned(),
            extension: "jpg".to_owned(),
        }
    }

    pub fn set_mime(&mut self, mime: &str) {
        self.mime = mime.to_owned();
    }

    pub fn set_extension(&mut self, extension: &str) {
        self.extension = extension.to_owned();
    }
}
