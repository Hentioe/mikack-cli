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

impl<'a, T> LinkListConverter<'a, T> {
    pub fn get_send_result(&self) -> Result<http::Result> {
        let mut helper = http::SendHelper::new();
        helper.send_get(self.url)?;
        if self.encoding.is_none() {
            Ok(helper.result())
        } else {
            match helper.result_bytes() {
                http::RawResult::Ok(resp_bytes) => {
                    let (cow, _encoding_used, had_errors) =
                        self.encoding.as_ref().unwrap().decode(&resp_bytes);
                    if had_errors {
                        return Ok(http::Result::Err(err_msg(format!(
                            "character encoding conversion failed, {}",
                            self.url
                        ))));
                    }
                    Ok(http::Result::Ok(cow[..].to_string()))
                }
                http::RawResult::Err(e) => Ok(http::Result::Err(e)),
            }
        }
    }
}

impl<'a, T> LinkListConverter<'a, T>
where
    T: FromLinkData,
{
    pub fn try_get_list(mut self) -> Result<Self> {
        match self.get_send_result()? {
            http::Result::Ok(html_s) => {
                let doc = html::parse_document(&html_s);
                let mut prefix = String::new();
                if self.find_text_prefix.is_some() {
                    prefix = format!(
                        "{} ",
                        self.find_text_prefix.as_ref().unwrap().call((&doc,))?
                    );
                }
                for element in doc.select(&html::parse_select(self.selector)?) {
                    let text = if self.text_in_dom.is_some() {
                        let selectors = self.text_in_dom.as_ref().unwrap();
                        html::find_text_in_element(&element, selectors)?
                    } else {
                        element.text().next().ok_or(err_msg(format!(
                            "no text found based on selector '{}'",
                            self.selector
                        )))?
                    };
                    let href = element.value().attr("href").ok_or(err_msg(format!(
                        "no href found based on selector '{}'",
                        self.selector
                    )))?;
                    let data = T::from(
                        &format!("{}{}", prefix, text).trim(),
                        &format!("{}{}", self.href_prefix, href),
                    );
                    self.list.push(data);
                }
                Ok(self)
            }
            http::Result::Err(e) => Err(e),
        }
    }
}

pub trait FromLinkData {
    fn from(text: &str, url: &str) -> Self;
}

impl FromLinkData for Detail {
    fn from(text: &str, url: &str) -> Self {
        Self::new(text, url)
    }
}

impl FromLinkData for Section {
    fn from(text: &str, url: &str) -> Self {
        Self::new(text, url)
    }
}
