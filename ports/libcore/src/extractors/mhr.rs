use super::{prelude::*, *};
use crate::{errors::*, html, http, jsrt, models::*};
use lazy_static::lazy_static;
use regex::Regex;
use serde_json::Value;

pub struct Mhr;

impl Extractor for Mhr {
    fn index(&self, _more: u32) -> Result<Vec<Detail>> {
        let url = "http://www.manhuaren.com/manhua-rank/";
        let mut fll: LinkListConverter<Detail> =
            LinkListConverter::new(url, "#rankList_1 > a", vec![]);
        fll.set_href_prefix("http://www.manhuaren.com")
            .set_text_in_dom(".rank-list-info-right-title");
        fll.try_get_list()?.result()
    }

    fn fetch_sections(&self, detail: &mut Detail) -> Result<()> {
        let url = &detail.url;
        let mut fll: LinkListConverter<Section> =
            LinkListConverter::new(&url, "ul.detail-list-select li > a", vec![]);
        fll.set_href_prefix("http://www.manhuaren.com")
            .text_prefix_finder(&|doc| {
                let name = html::find_text(doc, ".normal-top-title")?.to_string();
                Ok(name)
            });
        let section_list = fll.try_get_list()?.result()?;
        detail.section_list = section_list;
        detail.reverse_section_list();
        Ok(())
    }

    fn fetch_pages(&self, section: &mut Section) -> Result<()> {
        let mut helper = http::SendHelper::new();
        helper.send_get(&section.url)?;

        match helper.result() {
            http::Result::Ok(html_s) => {
                let caps = RE_CODE
                    .captures(&html_s)
                    .ok_or(err_msg("no script code found"))?;
                let code = caps
                    .get(1)
                    .ok_or(err_msg("no decrypt code block found"))?
                    .as_str();
                let wrapper_code = format!(
                    "{}\n{}",
                    &code, "console.log(JSON.stringify({images: newImgs}))"
                );
                let output = jsrt::read_output(&wrapper_code)?;
                let v: Value = serde_json::from_str(&output)?;
                for (i, img) in v["images"]
                    .as_array()
                    .ok_or(err_msg("no image list found in decrypt result"))?
                    .iter()
                    .enumerate()
                {
                    section.add_page(Page::new(
                        i as u32,
                        img.as_str()
                            .ok_or(err_msg("no image found in decrypt result"))?,
                    ));
                }
                if !section.has_name() {
                    let doc = html::parse_document(&html_s);
                    section.name = html::find_text(&doc, "p.view-fix-top-bar-title")?.to_owned();
                }
                Ok(())
            }
            http::Result::Err(e) => Err(e),
        }
    }
}

lazy_static! {
    static ref RE_CODE: Regex = Regex::new(r#"(eval\(.+\))[\s\S]*</script>"#).unwrap();
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_mhr_index() {
        let list = Mhr {}.index(0).unwrap();
        assert_eq!(30, list.len());
    }

    #[test]
    fn test_mhr_fetch_sections() {
        let mut detail = Detail::new(
            "约定的梦幻岛",
            "http://www.manhuaren.com/manhua-yuedingdemenghuandao/",
        );
        Mhr {}.fetch_sections(&mut detail).unwrap();
        assert_eq!(104, detail.section_list.len());
    }

    #[test]
    fn test_mhr_fetch_pages() {
        let mut section = Section::new(
            "约定的梦幻岛第99话",
            "http://www.manhuaren.com/m764478/",
        );
        Mhr {}.fetch_pages(&mut section).unwrap();
        assert_eq!(20, section.page_list.len());
    }
}
