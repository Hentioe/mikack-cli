use super::{prelude::*, *};
use crate::{errors::*, html, http, jsrt, models::*};
use lazy_static::lazy_static;
use regex::Regex;
use serde_json::Value;

pub struct Fcam;

impl Extractor for Fcam {
    fn index(&self, more: u32) -> Result<Vec<Detail>> {
        let url = if more > 0 {
            format!("http://www.verydm.com/index.php?r=comic/list&letter=&category_id=&story_id=&tag_id=&status=&show=grid&sort=hits&page={}", more + 1)
        } else {
            "http://www.verydm.com/index.php?r=comic/list&letter=&category_id=&story_id=&tag_id=&status=&show=grid&sort=hits&page=1".to_string()
        };
        let fll: LinkListConverter<Detail> =
            LinkListConverter::new(&url, "ul.grid-row.clearfix > li > p > a", vec![]);
        fll.try_get_list()?.result()
    }

    fn fetch_sections(&self, detail: &mut Detail) -> Result<()> {
        let url = &detail.url;
        let mut fll: LinkListConverter<Section> =
            LinkListConverter::new(&url, ".chapters > ul.clearfix > li > a", vec![]);
        fll.set_href_prefix("http://www.verydm.com")
            .text_prefix_finder(&|doc| {
                let name = doc
                    .select(&html::parse_select(".comic-name > h1")?)
                    .next()
                    .ok_or(err_msg(format!("did not get the name")))?
                    .text()
                    .next()
                    .ok_or(err_msg(format!("did not get the name")))?
                    .to_string();
                Ok(name)
            });
        let section_list = fll.try_get_list()?.result()?;
        detail.section_list = section_list;
        Ok(())
    }

    fn fetch_pages(&self, section: &mut Section) -> Result<()> {
        let mut helper = http::SendHelper::new();
        helper.send_get(&section.url)?;

        match helper.result() {
            http::Result::Ok(html_s) => Ok(()),
            http::Result::Err(e) => Err(e),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_fcam_index() {
        let list = Fcam {}.index(0).unwrap();
        assert_eq!(30, list.len());
    }

    #[test]
    fn test_fcam_fetch_sections() {
        let mut detail = Detail::new(
            "尼罗河女儿",
            "http://www.verydm.com/manhua/niluohenver",
        );
        Fcam {}.fetch_sections(&mut detail).unwrap();
        println!("{:?}", detail.section_list);
        assert_eq!(69, detail.section_list.len());
    }

    //    #[test]
    //    fn test_fcam_fetch_pages() {
    //        let mut section = Section::new(
    //            "火影忍者 第700话",
    //            "http://www.verydm.com/chapter.php?id=48141",
    //        );
    //        Hhmh {}.fetch_pages(&mut section).unwrap();
    //        assert_eq!(21, section.page_list.len());
    //    }
}
