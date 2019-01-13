use crate::errors::*;

pub mod html;
pub mod http;
pub mod origins;

// 漫画（图片集）平台
pub struct Platform {
    pub name: String,
    pub url: String,
}

// 漫画详情（例：火影忍者）
pub struct Detail {
    pub name: String,
    pub url: String,
    pub section_list: Vec<Section>,
}

// 漫画章节（例：第一话）
pub struct Section {
    pub name: String,
    pub url: String,
    pub page_list: Vec<Page>,
}

// 漫画单页（一张图）
pub struct Page {
    pub p: u32,
    pub url: String,
    pub extension_name: String,
    pub mime: String,
}

impl Platform {
    fn new(name: String, url: String) -> Self {
        Platform { name, url }
    }
}

impl Detail {
    fn new(name: &str, url: &str) -> Self {
        Detail {
            name: name.to_owned(),
            url: url.to_owned(),
            section_list: vec![],
        }
    }

    fn reset_section_list(&mut self, section_list: Vec<Section>) -> &Self {
        self.section_list = section_list;
        self
    }

    fn add_section(&mut self, section: Section) -> &Self {
        self.section_list.push(section);
        self
    }
}

impl Section {
    fn new(name: String, url: String) -> Self {
        Section {
            name,
            url,
            page_list: vec![],
        }
    }

    fn reset_page_list(&mut self, page_list: Vec<Page>) -> &Self {
        self.page_list = page_list;
        self
    }

    fn add_page(&mut self, page: Page) -> &Self {
        self.page_list.push(page);
        self
    }
}

impl Page {
    fn new(p: u32, url: String, extension_name: String, mime: String) -> Self {
        Page {
            p,
            url,
            extension_name,
            mime,
        }
    }
}

trait Fetcher {
    // 显示平台漫画索引
    fn index(&self, more: u32) -> Result<Vec<Section>>;
    // 指定漫画的章节列表
    fn fetch_sections(&self, detail: &mut Detail) -> Result<()>;
    // 抓取完整的一个章节（话）
    fn fetch_pages(&self, section: &mut Section) -> Result<()>;
}
