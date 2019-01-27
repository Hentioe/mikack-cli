use super::{prelude::*, *};
use crate::{errors::*, html, http, models::*};
use lazy_static::lazy_static;
use regex::Regex;

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
                let name = html::find_text(doc, ".comic-name > h1")?.to_string();
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
                let doc = &html::parse_document(&html_s);
                let url = html::find_attr(doc, ".main[style=\"display:block\"] > img", "src")?;
                let caps = RE_URL
                    .captures(url)
                    .ok_or(err_msg("did not match url path or file format"))?;
                let path = caps
                    .get(1)
                    .ok_or(err_msg(format!("no path found, {}", url)))?
                    .as_str();
                let format = caps
                    .get(2)
                    .ok_or(err_msg(format!("no format found, {}", url)))?
                    .as_str();
                let count = html::count(doc, "select > option")?;
                for i in 1..count + 1 {
                    let n = if i < 10 {
                        format!("00{}", i)
                    } else if i < 100 {
                        format!("0{}", i)
                    } else {
                        i.to_string()
                    };
                    section.add_page(Page::new(
                        (i - 1) as u32,
                        &format!("{}/{}.{}", path, &n, format),
                    ));
                }
                if !section.has_name() {
                    let keywords = html::find_attr(doc, "meta[name=\"keywords\"]", "content")?;
                    section.name = keywords
                        .split("，")
                        .collect::<Vec<&str>>()
                        .get(0)
                        .ok_or(err_msg(format!("no name found in keywords, {}", keywords)))?
                        .to_string();
                }
                Ok(())
            }
            http::Result::Err(e) => Err(e),
        }
    }
}

lazy_static! {
    static ref RE_URL: Regex = Regex::new(r#"(.+)/\d+\.(jpg|png|jpeg)"#).unwrap();
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
        assert_eq!(69, detail.section_list.len());
    }

    #[test]
    fn test_fcam_fetch_pages() {
        let mut section = Section::new(
            "火影忍者 第700话",
            "http://www.verydm.com/chapter.php?id=48141",
        );
        Fcam {}.fetch_pages(&mut section).unwrap();
        assert_eq!(25, section.page_list.len());
    }
}
