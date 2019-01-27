use super::{prelude::*, *};
use crate::{errors::*, html, http, models::*};
use lazy_static::lazy_static;
use regex::Regex;

pub struct Ehentai;

impl Extractor for Ehentai {
    fn index(&self, more: u32) -> Result<Vec<Detail>> {
        let url = format!("https://e-hentai.org/?page={}", more);
        let fll: LinkListConverter<Detail> =
            LinkListConverter::new(&url, "tbody > tr > td.itd .it5 > a", vec![]);
        fll.try_get_list()?.result()
    }

    fn fetch_sections(&self, detail: &mut Detail) -> Result<()> {
        let fll: LinkListConverter<Section> =
            LinkListConverter::new(&detail.url, "tbody > tr > td.itd .it5 > a", vec![]);
        detail.section_list = fll.try_get_list()?.result()?;
        Ok(())
    }

    fn fetch_pages(&self, section: &mut Section) -> Result<()> {
        let mut helper = http::SendHelper::new();
        helper.send_get(&section.url)?;

        match helper.result() {
            http::Result::Ok(html_s) => {
                let doc = html::parse_document(&html_s);
                let count_text = html::find_text(&doc, "div.gtb > p.gpc")?.trim();
                let caps = RE_COUNT
                    .captures(count_text)
                    .ok_or(err_msg(format!("no count info found, {}", &section.url)))?;

                let paging_count = 40.0;
                let count = caps
                    .get(1)
                    .ok_or(err_msg(format!("no count found, {}", &section.url)))?
                    .as_str()
                    .parse::<f32>()?;
                let page_count = (count / paging_count).ceil() as u32;
                let pure_url = RE_URL
                    .captures(&section.url)
                    .ok_or(err_msg(format!("Illegal url, {}", &section.url)))?
                    .get(1)
                    .ok_or(err_msg(format!(
                        "did not find a pure url, {}",
                        &section.url
                    )))?
                    .as_str();
                let mut url_list: Vec<String> = vec![];
                for i in 0..page_count {
                    let url = &format!("{}?p={}", pure_url, i);
                    let mut helper = http::SendHelper::new();
                    helper.send_get(url)?;
                    match helper.result() {
                        http::Result::Ok(html_s) => {
                            let doc = html::parse_document(&html_s);
                            let mut list: Vec<String> =
                                html::find_list_attr(&doc, ".gdtm > div > a", "href")?
                                    .iter()
                                    .map(|url| url.to_string())
                                    .collect();
                            url_list.append(&mut list);
                        }
                        http::Result::Err(e) => return Err(e),
                    }
                }

                for (i, url) in url_list.iter().enumerate() {
                    let mut helper = http::SendHelper::new();
                    helper.send_get(url)?;

                    match helper.result() {
                        http::Result::Ok(html_s) => {
                            let doc = html::parse_document(&html_s);
                            let url = html::find_attr(&doc, "#img", "src")?;
                            section.add_page(Page::new(i as u32, url));
                        }
                        http::Result::Err(e) => return Err(e),
                    }
                }
                if !section.has_name() {
                    section.name = html::find_text(&doc, "#gn")?.to_owned();
                }
                Ok(())
            }
            http::Result::Err(e) => Err(e),
        }
    }
}

lazy_static! {
    static ref RE_COUNT: Regex = Regex::new(r#"Showing \d+ - \d+ of (\d+) images"#).unwrap();
    static ref RE_URL: Regex = Regex::new(r#"(https?://e-hentai\.org/g/\d+/[^/]+/)"#).unwrap();
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_ehentai_index() {
        let list = Ehentai {}.index(0).unwrap();
        assert_eq!(25, list.len());
    }

    #[test]
    fn test_ehentai_fetch_pages() {
        let mut section = Section::new(UNKNOWN_NAME, "https://e-hentai.org/g/1354258/d29b587037/");
        Ehentai {}.fetch_pages(&mut section).unwrap();
        assert_eq!(11, section.page_list.len());
    }
}
