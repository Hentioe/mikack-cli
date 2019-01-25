use super::{prelude::*, *};
use crate::{errors::*, html, http, jsrt, models::*};
use lazy_static::lazy_static;
use regex::Regex;
use serde_json::Value;

pub struct Xxmh;

impl Extractor for Xxmh {
    fn index(&self, more: u32) -> Result<Vec<Detail>> {
        let url = if more > 0 {
            format!("https://www.177mh.net/wanjie/index_{}.html", more)
        } else {
            "https://www.177mh.net/wanjie/index.html".to_string()
        };

        let mut fll: LinkListConverter<Detail> =
            LinkListConverter::new(&url, ".ar_list_co > ul > li > span > a", vec![]);
        fll.set_href_prefix("https://www.177mh.net");
        fll.try_get_list()?.result()
    }

    fn fetch_sections(&self, detail: &mut Detail) -> Result<()> {
        let url = &detail.url;
        let mut fll: LinkListConverter<Section> =
            LinkListConverter::new(&url, "ul.ar_list_col > li > a", vec![]);
        fll.set_href_prefix("https://www.177mh.net")
            .text_prefix_finder(&|doc| {
                let name = html::find_text(doc, ".ar_list_coc > li > h1")?.to_string();
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
                let caps = RE_DECRYPT_CODE
                    .captures(&html_s)
                    .ok_or(err_msg("did not get the script code block"))?;
                let code = caps
                    .get(1)
                    .ok_or(err_msg("did not get the decryption code block"))?
                    .as_str();
                let wrapper_code = format!("{}\n{}", code, DECRYPT_BLOCK);
                // 托管给 JSRT 并获取结果
                let output = jsrt::read_output(&wrapper_code)?;
                let v: Value = serde_json::from_str(&output)?;
                let msg = v["msg"]
                    .as_str()
                    .ok_or(err_msg("no msg found in decrypt result"))?;
                let img_s = v["img_s"]
                    .as_u64()
                    .ok_or(err_msg("no img_s found in decrypt result"))?;
                for (i, path) in msg.split("|").collect::<Vec<&str>>().iter().enumerate() {
                    section.add_page(Page::new(
                        i as u32,
                        &format!("https://hws.readingbox.net/h{}/{}", img_s, path),
                    ));
                }
                if !section.has_name() {
                    let doc = html::parse_document(&html_s);
                    section.name = html::find_text(&doc, "#tab_srv + h1 > a")?.to_owned();
                }
                Ok(())
            }
            http::Result::Err(e) => Err(e),
        }
    }
}

lazy_static! {
    static ref RE_DECRYPT_CODE: Regex =
        Regex::new(r#"<script type="text/javascript">[\s\n]+(eval.+)[\s\n]+</script>"#).unwrap();
}

const DECRYPT_BLOCK: &'static str = r#"
console.log(JSON.stringify({msg: msg, img_s: img_s}))
"#;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_xxmh_index() {
        let det_list = Xxmh {}.index(0).unwrap();
        assert_eq!(20, det_list.len());
    }

    #[test]
    fn test_xxmh_fetch_sections() {
        let mut detail = Detail::new(
            "拳愿奥米迦",
            "https://www.177mh.net/colist_242774.html",
        );
        Xxmh {}.fetch_sections(&mut detail).unwrap();
        assert_eq!(1, detail.section_list.len());
    }

    #[test]
    fn test_xxmh_fetch_pages() {
        let mut section = Section::new(
            "怪盗无限面相 短篇",
            "https://www.177mh.net/201901/407015.html",
        );
        Xxmh {}.fetch_pages(&mut section).unwrap();
        assert_eq!(45, section.page_list.len());
    }
}
