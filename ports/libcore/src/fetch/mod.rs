use crate::errors::*;
use serde_derive::{Deserialize, Serialize};

pub mod html;
pub mod http;
pub mod upstream;

// 漫画（图片集）平台
#[derive(Debug)]
pub struct Platform {
    pub name: String,
    pub url: String,
}

// 漫画详情（例：火影忍者）
#[derive(Debug)]
pub struct Detail {
    pub name: String,
    pub url: String,
    pub section_list: Vec<Section>,
}

// 漫画章节（例：第一话）
#[derive(Debug)]
pub struct Section {
    pub name: String,
    pub url: String,
    pub page_list: Vec<Page>,
}

// 漫画单页（一张图）
#[derive(Debug, Deserialize, Serialize)]
pub struct Page {
    pub p: u32,
    pub url: String,
    pub mime: String,
    pub extension: String,
}

impl Platform {
    pub fn new(name: &str, url: &str) -> Self {
        Platform {
            name: name.to_string(),
            url: url.to_string(),
        }
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
}

impl Section {
    pub fn new(name: &str, url: &str) -> Self {
        Section {
            name: name.to_string(),
            url: url.to_string(),
            page_list: vec![],
        }
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
}

impl Page {
    pub fn new(p: u32, url: &str) -> Self {
        Page {
            p,
            url: url.to_string(),
            mime: "image/jpeg".to_string(),
            extension: "jpg".to_string(),
        }
    }

    pub fn set_mime(&mut self, mime: &str) {
        self.mime = mime.to_string();
    }

    pub fn set_extension(&mut self, extension: &str) {
        self.extension = extension.to_string();
    }
}

trait Fetcher {
    // 显示平台漫画索引
    fn index(&self, more: u32) -> Result<Vec<Detail>>;
    // 抓取章节列表
    fn fetch_sections(&self, detail: &mut Detail) -> Result<()>;
    // 抓取完整的一话
    fn fetch_pages(&self, section: &mut Section) -> Result<()>;
}
