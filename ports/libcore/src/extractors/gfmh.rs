use super::{prelude::*, *};
use crate::{errors::*, html, http, jsrt, models::*};
use lazy_static::lazy_static;
use regex::Regex;
use serde_json::Value;

pub struct Gfmh;

impl Extractor for Gfmh {
    fn index(&self, more: u32) -> Result<Vec<Detail>> {
        let url = format!(
            "http://www.gufengmh.com/list/ribenmanhua/click/{}/",
            more + 1
        );
        let fll: LinkListConverter<Detail> =
            LinkListConverter::new(&url, ".book-list > li > .ell > a", vec![]);
        fll.try_get_list()?.result()
    }

    fn fetch_sections(&self, detail: &mut Detail) -> Result<()> {
        let url = &detail.url;
        let mut fll: LinkListConverter<Section> =
            LinkListConverter::new(&url, ".chapter-body > ul > li > a", vec![]);
        fll.set_href_prefix("http://www.gufengmh.com")
            .set_text_in_dom("span")
            .text_prefix_finder(&|doc| {
                let name = html::find_text(doc, ".book-title > h1 > span")?
                    .trim()
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
            http::Result::Ok(html_s) => {
                let caps = RE_CODE
                    .captures(&html_s)
                    .ok_or(err_msg(format!("no script code found, {}", &section.url)))?;
                let code = caps
                    .get(1)
                    .ok_or(err_msg(format!(
                        "no encryption code block found, {}",
                        &section.url
                    )))?
                    .as_str();
                let wrapper_code = format!("{}\n{}", code, &DECRYPT_BLOCK);
                // 托管给 JSRT 运行
                let output = jsrt::read_output(&wrapper_code)?;
                let v: Value = serde_json::from_str(&output)?;
                let path = v["path"]
                    .as_str()
                    .ok_or(err_msg(format!("no path found, {}", &section.url)))?;
                for (i, img) in v["images"]
                    .as_array()
                    .ok_or(err_msg(format!("no image list found, {}", &section.url)))?
                    .iter()
                    .enumerate()
                {
                    section.add_page(Page::new(
                        i as u32,
                        &format!(
                            "http://res.gufengmh.com/{}{}",
                            &path,
                            img.as_str()
                                .ok_or(err_msg(format!("no image found, {}", &section.url)))?
                        ),
                    ));
                }
                if !section.has_name() {
                    let doc = html::parse_document(&html_s);
                    section.name = format!(
                        "{} {}",
                        html::find_text(&doc, ".w996.title.pr h1")?,
                        html::find_text(&doc, ".w996.title.pr h2")?
                    )
                }
                Ok(())
            }
            http::Result::Err(e) => Err(e),
        }
    }
}

lazy_static! {
    static ref RE_CODE: Regex =
        Regex::new(r#"<script>;var siteName = "";([\s\S]+)</script><div class="chapter-view">"#)
            .unwrap();
}

const DECRYPT_BLOCK: &'static str = r#"
var images = []
chapterImages.forEach((img, i) => {
    images.push(img)
})
console.log(JSON.stringify({images: images, path: chapterPath}))
"#;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_gfmh_index() {
        let list = Gfmh {}.index(0).unwrap();
        assert_eq!(36, list.len());
    }

    #[test]
    fn test_gfmh_fetch_sections() {
        let mut detail = Detail::new(
            "一拳超人",
            "http://www.gufengmh.com/manhua/yiquanchaoren/",
        );
        Gfmh {}.fetch_sections(&mut detail).unwrap();
        assert_eq!(574, detail.section_list.len());
    }

    #[test]
    fn test_gfmh_fetch_pages() {
        let mut section = Section::new(
            "一拳超人 第141话",
            "http://www.gufengmh.com/manhua/yiquanchaoren/642366.html",
        );
        Gfmh {}.fetch_pages(&mut section).unwrap();
        assert_eq!(11, section.page_list.len());
    }
}
