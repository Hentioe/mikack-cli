use crate::{errors::*, models::*};

pub trait Extractor {
    // 显示平台漫画索引
    fn index(&self, more: u32) -> Result<Vec<Detail>>;
    // 抓取章节列表
    fn fetch_sections(&self, detail: &mut Detail) -> Result<()>;
    // 抓取完整的一话
    fn fetch_pages(&self, section: &mut Section) -> Result<()>;
}
