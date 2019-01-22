use super::*;
use crate::{errors::*, html, http, models::*};

pub trait Extractor {
    // 显示平台漫画索引
    fn index(&self, more: u32) -> Result<Vec<Detail>>;
    // 抓取章节列表
    fn fetch_sections(&self, detail: &mut Detail) -> Result<()>;
    // 抓取完整的一话
    fn fetch_pages(&self, section: &mut Section) -> Result<()>;
}

impl<'a> FromLinkList<'a, Detail> {
    pub fn try_get_list(mut self) -> Result<Self> {
        let mut helper = http::SendHelper::new();
        helper.send_get(self.url)?;
        match helper.result() {
            http::Result::Ok(html_s) => {
                let doc = html::parse_document(&html_s);
                for element in doc.select(&html::parse_select(self.selector)?) {
                    let text = element
                        .text()
                        .next()
                        .ok_or(err_msg(format!("no text found, {}", element.inner_html())))?;
                    let href = element
                        .value()
                        .attr("href")
                        .ok_or(err_msg(format!("no href found, {}", element.inner_html())))?;
                    let detail = Detail::new(
                        &format!("{}{}", self.text_prefix, text),
                        &format!("{}{}", self.href_prefix, href),
                    );
                    self.list.push(detail);
                }
                Ok(self)
            }
            http::Result::Err(e) => Err(e),
        }
    }
}
